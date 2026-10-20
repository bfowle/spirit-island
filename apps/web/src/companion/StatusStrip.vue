<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import type { CompanionHealth, CompanionState, ReplayCommand } from './types'
import type { Connection } from './useCompanionStream'
import { endingLabel, stepLabel } from './format'

const props = defineProps<{
  state: CompanionState | null
  health: CompanionHealth | null
  connection: Connection
  lastError: string | null
}>()
const emit = defineEmits<{ replay: [cmd: ReplayCommand] }>()

const pub = computed(() => props.state?.public ?? null)
const src = computed(() => props.state?.source ?? props.health?.source ?? null)
const rp = computed(() => src.value?.replay ?? null)

const phaseText = computed(() => {
  const p = pub.value
  if (!p) return '—'
  return stepLabel(p.phase)
})
const stepText = computed(() => {
  const p = pub.value
  if (!p || !p.step || p.step === p.phase) return null
  return stepLabel(p.step)
})
const adversary = computed(() => {
  const a = pub.value?.adversaries ?? []
  if (!a.length) return 'No adversary'
  return a.map((x) => `${x.title.replace(/^The /, '')} L${x.level}`).join(' + ')
})
const boards = computed(() => (pub.value?.boards ?? []).map((b) => b.id).join(' + ') || '—')
const spirits = computed(() => (pub.value?.spirits ?? []).map((s) => s.title + (s.aspect ? ` (${s.aspect})` : '')).join(', ') || '—')
const warnings = computed(() => pub.value?.warnings ?? props.health?.warnings ?? [])

const modeLabel = computed(() => {
  const m = src.value?.mode
  if (m === 'replay') return 'Replay'
  if (m === 'live') return 'Live'
  if (m === 'off') return 'Off'
  return m ? m : 'Unknown'
})
const streamLabel = computed(() =>
  props.connection === 'open' ? 'Stream connected' : props.connection === 'connecting' ? 'Connecting' : 'Reconnecting',
)

// ── replay transport ──
const speeds = [1, 4, 8, 32]
const speed = ref(8)
watch(() => rp.value?.speed, (s) => { if (s) speed.value = s }, { immediate: true })
const speedOptions = computed(() => (speeds.includes(speed.value) ? speeds : [...speeds, speed.value].sort((a, b) => a - b)))
const scrub = ref(0)
const dragging = ref(false)
watch(() => rp.value?.index, (i) => { if (i != null && !dragging.value) scrub.value = i }, { immediate: true })

function togglePlay() {
  if (!rp.value) return
  emit('replay', rp.value.playing ? { action: 'pause' } : { action: 'play', speed: speed.value })
}
function step(by: number) { emit('replay', { action: 'step', by }) }
function onSpeed() { if (rp.value?.playing) emit('replay', { action: 'play', speed: speed.value }) }
function onScrubEnd() {
  dragging.value = false
  emit('replay', { action: 'seek', index: scrub.value })
}

function onKey(e: KeyboardEvent) {
  if (!rp.value) return
  const t = e.target as HTMLElement | null
  if (t && (t.tagName === 'INPUT' || t.tagName === 'SELECT' || t.tagName === 'TEXTAREA' || t.isContentEditable)) return
  if (e.key === ' ' && t?.tagName !== 'BUTTON') { e.preventDefault(); togglePlay() }
  else if (e.key === '[') step(-1)
  else if (e.key === ']') step(1)
}
onMounted(() => window.addEventListener('keydown', onKey))
onUnmounted(() => window.removeEventListener('keydown', onKey))
</script>

<template>
  <header class="strip">
    <div class="turn-block">
      <span class="turn-word">Turn</span>
      <span class="turn-num">{{ pub?.turn ?? '–' }}</span>
      <div class="phase">
        <span class="phase-main">{{ phaseText }}</span>
        <span v-if="stepText" class="phase-step">{{ stepText }} step</span>
      </div>
    </div>

    <dl class="facts">
      <div><dt>Adversary</dt><dd>{{ adversary }}</dd></div>
      <div><dt>Boards</dt><dd>{{ boards }}</dd></div>
      <div class="spirit"><dt>Spirit</dt><dd :title="spirits">{{ spirits }}</dd></div>
    </dl>

    <div v-if="pub?.ending" class="ending" role="status">
      Game over: {{ endingLabel(pub.ending) }}
    </div>

    <div class="source" :class="{ bad: src?.stale || connection !== 'open' || (src?.decode_errors ?? 0) > 0 }">
      <div class="src-row">
        <span class="mode" :data-mode="src?.mode">{{ modeLabel }}</span>
        <span class="dot" :class="connection" aria-hidden="true"></span>
        <span>{{ streamLabel }}</span>
        <span v-if="src?.stale" class="flag warn">Stale</span>
        <span v-if="(src?.decode_errors ?? 0) > 0" class="flag warn">{{ src?.decode_errors }} decode errors</span>
        <span v-if="warnings.length" class="flag warn" :title="warnings.join('\n')">{{ warnings.length }} warnings</span>
        <span v-if="health && !health.has_state" class="flag warn">No game state yet</span>
      </div>
      <div class="src-row sub">
        <span class="save" :title="src?.last_save ?? ''">{{ src?.last_save ?? 'No save read yet' }}</span>
        <span v-if="src?.engine_build" class="build">engine {{ src.engine_build }}</span>
      </div>
      <div v-if="lastError && connection !== 'open'" class="src-row sub err">{{ lastError }}</div>
      <div v-if="src?.note" class="src-row sub">{{ src.note }}</div>
    </div>

    <div v-if="rp" class="replay" aria-label="Replay controls">
      <div class="transport">
        <button type="button" class="tbtn" title="Step back one save ([)" @click="step(-1)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M4 3v10M13 3 6 8l7 5z" fill="currentColor" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/></svg>
          <span class="sr">Step back</span>
        </button>
        <button type="button" class="tbtn play" :title="rp.playing ? 'Pause (Space)' : 'Play (Space)'" @click="togglePlay">
          <svg v-if="!rp.playing" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M4 2.5 13.5 8 4 13.5z" fill="currentColor"/></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="0.6" fill="currentColor"/><rect x="9.5" y="2.5" width="3" height="11" rx="0.6" fill="currentColor"/></svg>
          <span class="sr">{{ rp.playing ? 'Pause' : 'Play' }}</span>
        </button>
        <button type="button" class="tbtn" title="Step forward one save (])" @click="step(1)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M12 3v10M3 3l7 5-7 5z" fill="currentColor" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/></svg>
          <span class="sr">Step forward</span>
        </button>
        <label class="speed">
          <span class="sr">Speed</span>
          <select v-model.number="speed" @change="onSpeed">
            <option v-for="s in speedOptions" :key="s" :value="s">{{ s }}×</option>
          </select>
        </label>
        <span class="clock" title="Capture clock of this save">{{ rp.clock }}</span>
      </div>
      <div class="scrub">
        <input
          type="range"
          :min="0"
          :max="Math.max(0, rp.total - 1)"
          v-model.number="scrub"
          aria-label="Replay position"
          @pointerdown="dragging = true"
          @input="dragging = true"
          @change="onScrubEnd"
        />
        <span class="pos">{{ scrub + 1 }} / {{ rp.total }}</span>
      </div>
      <div class="capture" :title="`Capture ${rp.capture}`">{{ rp.capture }}</div>
    </div>

    <div class="tabs-slot"><slot /></div>
  </header>
</template>

<style scoped>
.strip {
  display: flex;
  align-items: stretch;
  gap: 18px;
  padding: 8px 16px;
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  flex-wrap: wrap;
}
.turn-block { display: flex; align-items: center; gap: 8px; padding-right: 16px; border-right: 1px solid var(--border-subtle); }
.turn-word { font-family: var(--font-brand); font-size: 15px; color: var(--text-secondary); letter-spacing: 0.02em; }
.turn-num { font-family: var(--font-brand); font-size: 40px; line-height: 1; font-weight: 600; color: var(--text-white); }
.phase { display: flex; flex-direction: column; line-height: 1.2; margin-left: 4px; }
.phase-main { font-size: 15px; font-weight: 600; color: var(--text-white); }
.phase-step { font-size: 12px; color: var(--text-secondary); }

.facts { display: flex; gap: 16px; margin: 0; align-items: center; min-width: 0; }
.facts div { display: flex; flex-direction: column; min-width: 0; }
.facts dt { font-size: 11px; color: var(--text-muted); }
.facts dd { margin: 0; font-size: 13px; font-weight: 600; color: var(--text-primary); white-space: nowrap; }
.facts .spirit dd { max-width: 240px; overflow: hidden; text-overflow: ellipsis; }

.ending {
  align-self: center;
  padding: 6px 12px;
  border-radius: 8px;
  background: var(--danger-soft);
  border: 1px solid rgba(239, 68, 68, 0.45);
  color: #fecaca;
  font-weight: 600;
  font-size: 13px;
}

.source { display: flex; flex-direction: column; justify-content: center; gap: 2px; min-width: 0; font-size: 12px; color: var(--text-secondary); margin-left: auto; }
.src-row { display: flex; align-items: center; gap: 6px; white-space: nowrap; }
.src-row.sub { color: var(--text-muted); font-size: 11px; }
.src-row.err { color: #fca5a5; }
.save { max-width: 300px; overflow: hidden; text-overflow: ellipsis; }
.mode { font-weight: 700; color: var(--text-primary); padding: 0 6px; border-radius: 4px; background: var(--bg-raised); border: 1px solid var(--border-strong); }
.mode[data-mode='live'] { border-color: rgba(16, 185, 129, 0.5); color: #6ee7b7; }
.mode[data-mode='replay'] { border-color: rgba(124, 58, 237, 0.55); color: #c4b5fd; }
.dot { width: 7px; height: 7px; border-radius: 50%; background: var(--text-muted); }
.dot.open { background: var(--accent-green); }
.dot.reconnecting, .dot.connecting { background: var(--accent-amber); }
.flag { padding: 0 6px; border-radius: 4px; font-weight: 600; }
.flag.warn { background: var(--warn-soft); color: var(--accent-amber); }

.replay {
  display: grid;
  grid-template-columns: auto;
  gap: 4px;
  padding-left: 16px;
  border-left: 1px solid var(--border-subtle);
  min-width: 300px;
}
.transport { display: flex; align-items: center; gap: 6px; }
.tbtn { display: inline-grid; place-items: center; width: 30px; height: 28px; padding: 0; border-radius: 6px; }
.tbtn.play { width: 36px; background: var(--bg-active); border-color: var(--border-strong); }
.speed select { height: 28px; padding: 0 6px; font-size: 12px; }
.clock { font-variant-numeric: tabular-nums; font-size: 13px; color: var(--text-primary); margin-left: 4px; }
.scrub { display: flex; align-items: center; gap: 8px; }
.scrub input { flex: 1 1 auto; accent-color: var(--accent-purple); min-width: 160px; }
.pos { font-size: 11px; color: var(--text-secondary); font-variant-numeric: tabular-nums; white-space: nowrap; }
.capture { font-size: 11px; color: var(--text-muted); }
.tabs-slot { display: flex; align-items: center; }
.sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
</style>
