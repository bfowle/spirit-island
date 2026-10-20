// Spirit Island (Steam, Unity/Mono x64): reload a checkpoint save in the running app.
//
// Run by `si inject` through Frida's standalone frida-inject.exe:
//   frida-inject.exe -n SpiritIsland.exe -s reload_checkpoint.js -P {"path": "C:\\…\\save-Undo-990.dat", "name": "save-Undo-990.dat"}
// frida-inject calls rpc.exports.init(stage, parameters) once the script is loaded.
//
// si inject has already copied the checkpoint into the app's save folder (the loader's
// disk path is GetSaveFolder() + the file name) under the undo name and the continue
// name. Two routes, both on the Unity main thread (inside an Update), where view code
// must run:
//  - a game is open: GameViewController.ReloadFromFile(filename, allowCloudSave=false,
//    isUndo=true, isFullSync=false), the app's own Undo path (LoadGame from disk,
//    teardown, DisplayGame);
//  - the main menu: GameSerializer.IsContinueValid() (the app loads the continue file to
//    check it) and MainMenuViewController.ContinueClicked() to open it.
// Names and arities were read from MainAssembly.dll's metadata on 2026-10-20
// (Handelabra.SpiritIsland.View.GameSerializer / GameViewController / MainMenuViewController).
//
// Every step reports with send({step, …}) and, when parameters.log is given, as a JSON
// line appended to that file (frida-inject holds its stdout until it exits in attach
// mode, so si inject follows the file). A managed exception comes back as text in
// {step: "error"}; {step: "done"} ends the run (si inject then detaches).

'use strict';

let logPath = null;
function report(payload) {
  send(payload);
  if (logPath) {
    try {
      const f = new File(logPath, 'a');
      f.write(JSON.stringify(payload) + '\n');
      f.flush();
      f.close();
    } catch (e) {
      send({ step: 'log-error', message: String(e) });
    }
  }
}

const MONO = 'mono-2.0-bdwgc.dll';
const NS = 'Handelabra.SpiritIsland.View';

// Frida 17: exports are looked up on the Module object, not through a static helper.
function api(name, ret, args) {
  const mod = Process.findModuleByName(MONO);
  if (mod === null) throw new Error(`module not loaded: ${MONO} (is this SpiritIsland.exe, the Steam build?)`);
  const p = mod.findExportByName(name);
  if (p === null) throw new Error(`mono export missing: ${name}`);
  // 'propagate': a hardware fault inside managed code reaches Mono's own handler, which
  // turns a null dereference into a NullReferenceException the game code catches; with
  // Frida's default the fault surfaced here as "access violation accessing 0x2c".
  return new NativeFunction(p, ret, args, { exceptions: 'propagate' });
}

let mono = null;
function loadMono() {
  if (mono) return mono;
  mono = {
    get_root_domain: api('mono_get_root_domain', 'pointer', []),
    thread_attach: api('mono_thread_attach', 'pointer', ['pointer']),
    assembly_foreach: api('mono_assembly_foreach', 'void', ['pointer', 'pointer']),
    assembly_get_image: api('mono_assembly_get_image', 'pointer', ['pointer']),
    image_get_name: api('mono_image_get_name', 'pointer', ['pointer']),
    class_from_name: api('mono_class_from_name', 'pointer', ['pointer', 'pointer', 'pointer']),
    class_get_method_from_name: api('mono_class_get_method_from_name', 'pointer', ['pointer', 'pointer', 'int']),
    class_get_type: api('mono_class_get_type', 'pointer', ['pointer']),
    type_get_object: api('mono_type_get_object', 'pointer', ['pointer', 'pointer']),
    compile_method: api('mono_compile_method', 'pointer', ['pointer']),
    runtime_invoke: api('mono_runtime_invoke', 'pointer', ['pointer', 'pointer', 'pointer', 'pointer']),
    string_new: api('mono_string_new', 'pointer', ['pointer', 'pointer']),
    string_to_utf8: api('mono_string_to_utf8', 'pointer', ['pointer']),
    array_new: api('mono_array_new', 'pointer', ['pointer', 'pointer', 'int']),
    array_addr_with_size: api('mono_array_addr_with_size', 'pointer', ['pointer', 'int', 'int']),
    get_byte_class: api('mono_get_byte_class', 'pointer', []),
    object_get_class: api('mono_object_get_class', 'pointer', ['pointer']),
    object_unbox: api('mono_object_unbox', 'pointer', ['pointer']),
    array_length: api('mono_array_length', 'pointer', ['pointer']),
    class_get_name: api('mono_class_get_name', 'pointer', ['pointer']),
    method_full_name: api('mono_method_full_name', 'pointer', ['pointer', 'int']),
    domain_get: api('mono_domain_get', 'pointer', []),
    gchandle_new: api('mono_gchandle_new', 'uint32', ['pointer', 'int']),
    gchandle_free: api('mono_gchandle_free', 'void', ['uint32']),
    method_signature: api('mono_method_signature', 'pointer', ['pointer']),
    signature_get_param_count: api('mono_signature_get_param_count', 'uint32', ['pointer']),
  };
  return mono;
}

const state = { domain: null, images: {}, gvcClass: null, menuClass: null, serializer: null, pending: null, hooked: false, frames: 0, ticks: { 'in-game': 0, 'main-menu': 0 }, route: null };

function findImages() {
  const cb = new NativeCallback((assembly, _user) => {
    const image = mono.assembly_get_image(assembly);
    state.images[mono.image_get_name(image).readUtf8String()] = image;
  }, 'void', ['pointer', 'pointer']);
  mono.assembly_foreach(cb, NULL);
  return Object.keys(state.images);
}

function klass(imageName, ns, name) {
  const image = state.images[imageName];
  if (!image) throw new Error(`image not loaded: ${imageName} (loaded: ${Object.keys(state.images).join(', ')})`);
  const k = mono.class_from_name(image, Memory.allocUtf8String(ns), Memory.allocUtf8String(name));
  if (k.isNull()) throw new Error(`class not found: ${ns}.${name} in ${imageName}`);
  return k;
}

function method(k, name, argc) {
  const m = mono.class_get_method_from_name(k, Memory.allocUtf8String(name), argc);
  if (m.isNull()) throw new Error(`method not found: ${name}/${argc}`);
  return m;
}

// Any arity: the method and how many parameters it really takes.
function methodAny(k, name) {
  const m = mono.class_get_method_from_name(k, Memory.allocUtf8String(name), -1);
  if (m.isNull()) throw new Error(`method not found: ${name}`);
  return { m, argc: mono.signature_get_param_count(mono.method_signature(m)) };
}

function invoke(m, instance, args) {
  const argv = Memory.alloc(Process.pointerSize * Math.max(1, args.length));
  args.forEach((a, i) => argv.add(i * Process.pointerSize).writePointer(a));
  const exc = Memory.alloc(Process.pointerSize);
  exc.writePointer(NULL);
  const r = mono.runtime_invoke(m, instance, argv, exc);
  const e = exc.readPointer();
  if (!e.isNull()) {
    const toString = method(mono.object_get_class(e), 'ToString', 0);
    const s = mono.runtime_invoke(toString, e, NULL, NULL);
    throw new Error('managed exception: ' + mono.string_to_utf8(s).readUtf8String());
  }
  return r;
}

// value-type arguments go to mono_runtime_invoke as pointers to the raw value
function boolArg(v) {
  const p = Memory.alloc(4);
  p.writeU8(v ? 1 : 0);
  return p;
}

// Nullable<T> parameters go to mono_runtime_invoke as a boxed T, or NULL for "no value".
function nullBoolArg() {
  return NULL;
}

// Managed objects the script creates are not on any managed stack, so the Boehm GC may
// collect them between creation and use; a pinned GC handle keeps each one alive until
// the call that uses it has returned.
const pins = [];
function pin(obj) {
  pins.push(mono.gchandle_new(obj, 1));
  return obj;
}
function unpinAll() {
  while (pins.length) mono.gchandle_free(pins.pop());
}

function str(s) {
  return pin(mono.string_new(state.domain, Memory.allocUtf8String(s)));
}

function unboxBool(obj) {
  if (obj.isNull()) throw new Error('runtime_invoke returned null where a boxed bool was expected');
  return mono.object_unbox(obj).readU8() !== 0;
}

function byteArray(buffer) {
  const bytes = new Uint8Array(buffer);
  const arr = pin(mono.array_new(state.domain, mono.get_byte_class(), bytes.length));
  mono.array_addr_with_size(arr, 1, 0).writeByteArray(buffer);
  return arr;
}

function setup() {
  loadMono();
  state.domain = mono.get_root_domain();
  mono.thread_attach(state.domain);
  const names = findImages();
  state.gvcClass = klass('MainAssembly', NS, 'GameViewController');
  state.menuClass = klass('MainAssembly', NS, 'MainMenuViewController');
  state.serializer = klass('MainAssembly', NS, 'GameSerializer');
  method(state.gvcClass, 'ReloadFromFile', 4);
  method(state.gvcClass, 'Update', 0);
  method(state.menuClass, 'Update', 0);
  method(state.menuClass, 'ContinueClicked', 0);
  method(state.serializer, 'DoesSaveFilenameExist', 2);
  method(state.serializer, 'IsContinueValid', 0);
  method(state.serializer, 'get_UseSteamCloud', 0);
  method(state.serializer, 'GetSaveFolder', 0);
  methodAny(state.serializer, 'DeleteSaveFilename');
  report({ step: 'setup', images: names.length, pid: Process.id });
}

// Every native step reports before it runs, so a fault names the call.
function step(what, f) {
  report({ step: 'native', what });
  const r = f();
  return r;
}

function utf8(monoString) {
  return monoString.isNull() ? null : mono.string_to_utf8(monoString).readUtf8String();
}

// Probes with the app's own simplest methods: no arguments, then one value type, then an
// array, then a static call with a reference argument. Each reports its result.
function probes(job) {
  const cloud = step('runtime_invoke get_UseSteamCloud()', () => invoke(method(state.serializer, 'get_UseSteamCloud', 0), NULL, []));
  report({ step: 'probe', what: 'UseSteamCloud', value: cloud.isNull() ? null : unboxBool(cloud) });
  const folder = step('runtime_invoke GetSaveFolder()', () => invoke(method(state.serializer, 'GetSaveFolder', 0), NULL, []));
  report({ step: 'probe', what: 'GetSaveFolder', value: utf8(folder) });
  const exists = method(state.serializer, 'DoesSaveFilenameExist', 2);
  for (const name of [job.name, job.continueName]) {
    const ex = step(`runtime_invoke DoesSaveFilenameExist(${name}, false)`, () => invoke(exists, NULL, [str(name), boolArg(false)]));
    report({ step: 'probe', what: 'on disk for the app', name, value: ex.isNull() ? null : unboxBool(ex) });
  }
}

// In a game: the app's own Undo path, reading the file si inject placed in the save folder.
function reloadInGame(gvc, job) {
  probes(job);
  // ReloadFromFile(string filename, bool allowCloudSave, bool isUndo, bool isFullSync)
  step(`runtime_invoke ReloadFromFile(${job.name}, false, true, false)`, () => invoke(method(state.gvcClass, 'ReloadFromFile', 4), gvc, [str(job.name), boolArg(false), boolArg(true), boolArg(false)]));
  report({ step: 'reload', route: 'in-game', ok: true });
}

// At the main menu: Continue loads the app's continue file if one exists (in Steam Cloud
// when cloud is on), else the newest undo file on disk, which is ours. So the existing
// continue game is replaced: deleted where the app keeps it, as the app's own New Game
// does when it abandons a game. The old game is still in its capture.
function continueFromMenu(menu, job) {
  probes(job);
  const exists = method(state.serializer, 'DoesSaveFilenameExist', 2);
  const del = methodAny(state.serializer, 'DeleteSaveFilename');
  report({ step: 'probe', what: 'DeleteSaveFilename arity', argc: del.argc });
  for (const useCloud of [true, false]) {
    const there = unboxBool(step(`runtime_invoke DoesSaveFilenameExist(${job.continueName}, ${useCloud})`, () => invoke(exists, NULL, [str(job.continueName), boolArg(useCloud)])));
    report({ step: 'continue-file', where: useCloud ? 'cloud' : 'disk', exists: there });
    if (there) {
      const args = del.argc >= 2 ? [str(job.continueName), boolArg(useCloud)] : [str(job.continueName)];
      step(`runtime_invoke DeleteSaveFilename(${job.continueName}${del.argc >= 2 ? ', ' + useCloud : ''})`, () => invoke(del.m, NULL, args));
      const still = unboxBool(invoke(exists, NULL, [str(job.continueName), boolArg(useCloud)]));
      report({ step: 'continue-file', where: useCloud ? 'cloud' : 'disk', deleted: !still, note: "the app's previous continue game is replaced by the checkpoint (its capture keeps it)" });
    }
  }
  const valid = step('runtime_invoke IsContinueValid()', () => invoke(method(state.serializer, 'IsContinueValid', 0), NULL, []));
  const ok = valid.isNull() ? false : unboxBool(valid);
  report({ step: 'continue-valid', ok });
  if (!ok) throw new Error('the app did not accept the checkpoint as its continue game (IsContinueValid false); the app log has the reason. If UseSteamCloud is true above, open any game and run again: the in-game route reads from disk.');
  step('runtime_invoke ContinueClicked()', () => invoke(method(state.menuClass, 'ContinueClicked', 0), menu, []));
  report({ step: 'reload', route: 'main-menu', ok: true });
}

// Run the job on the Unity main thread at the next Update of whichever controller is
// live: GameViewController (a game is open) or MainMenuViewController (the menu).
function scheduleReload(name, buffer) {
  return new Promise((resolve, reject) => {
    state.pending = { name, continueName: state.continueName || 'save-ContinueGame.dat', buffer, resolve, reject };
    if (state.hooked) return;
    state.hooked = true;
    // Both controllers can tick at once (the menu stays alive under a game), so the first
    // frame only starts a short count; after it, a ticking game controller wins.
    const hook = (klassRef, route, run) => {
      const addr = mono.compile_method(method(klassRef, 'Update', 0));
      Interceptor.attach(addr, {
        onEnter(args) {
          state.frames += 1;
          state.ticks[route] += 1;
          if (state.frames === 1) {
            report({ step: 'frame', route, note: 'the main thread is running; deciding the route over the next half second' });
            setTimeout(() => {
              state.route = state.ticks['in-game'] > 0 ? 'in-game' : 'main-menu';
              report({ step: 'route', chosen: state.route, ticks: state.ticks });
            }, 500);
          }
          if (state.route !== route) return;
          const job = state.pending;
          if (!job) return;
          state.pending = null;
          try {
            run(args[0], job);
            job.resolve({ route });
          } catch (e) {
            job.reject(e);
          } finally {
            unpinAll();
          }
        },
      });
      report({ step: 'hooked', route, address: addr.toString() });
    };
    hook(state.gvcClass, 'in-game', reloadInGame);
    hook(state.menuClass, 'main-menu', continueFromMenu);
    const watchdog = () => {
      if (state.pending && state.frames === 0) {
        report({ step: 'waiting', note: 'no Update frame yet from the game or the main menu: click the game window (Unity pauses in the background), and be at the main menu or inside a game' });
        setTimeout(watchdog, 8000);
      }
    };
    setTimeout(watchdog, 6000);
  });
}

async function run(name, path) {
  try {
    setup();
    const buffer = File.readAllBytes(path);
    const head = new Uint8Array(buffer, 0, 2);
    if (!((head[0] === 0 && head[1] === 1) || (head[0] === 0x1f && head[1] === 0x8b))) {
      throw new Error(`${path} does not look like a Spirit Island save (first bytes ${head[0]} ${head[1]})`);
    }
    report({ step: 'read', path, bytes: buffer.byteLength });
    const r = await scheduleReload(name, buffer);
    report({ step: 'done', ...r });
  } catch (e) {
    report({ step: 'error', message: String(e) });
    report({ step: 'done', ok: false });
  }
}

rpc.exports = {
  // frida-inject entry: parameters from -P
  init(stage, parameters) {
    const p = parameters || {};
    logPath = p.log || null;
    state.continueName = p.continue_name || 'save-ContinueGame.dat';
    report({ step: 'init', stage, pid: Process.id, name: p.name, path: p.path });
    if (!p.path) {
      report({ step: 'error', message: 'parameters.path (the .dat to load) is required' });
      report({ step: 'done', ok: false });
      return;
    }
    run(p.name || 'save-Undo-990.dat', p.path);
  },
  // for a persistent session (the study board): setup once, reload many times
  setup() {
    setup();
    return { ok: true };
  },
  reload(name, path) {
    return scheduleReload(name, File.readAllBytes(path));
  },
};
