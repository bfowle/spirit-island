//! `si-companion`: the native companion app (native app spec, milestone N1).
//!
//! ```text
//! si-companion                          live: follows the Steam game
//! si-companion --capture <dir>          replay a recorded game (← → step, Space play)
//! si-companion --capture <dir> --at T5/FastPower --screenshot out.png [--size 1600x900] [--scale 1]
//! si-companion --screenshot out.png     the live game, no window
//! ```
//! `--at` takes a save index, `end`, or `T<turn>/<Step>`.

mod data;
mod views;

use anyhow::Context;
use si_steam::source::ReplayCommand;
use si_ui::theme;
use si_ui::{Input, Key, Renderer, TextCache, Ui, UiState};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use views::{Action, ViewState};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key as WKey, NamedKey};
use winit::window::{CursorIcon, Window, WindowId};

struct Args {
    capture: Option<PathBuf>,
    at: Option<String>,
    screenshot: Option<PathBuf>,
    size: (u32, u32),
    scale: f32,
    /// Pin a phase tab (`spirit`, `fast`, `event`, `fear`, `invaders`, `slow`) or `rules`.
    tab: Option<String>,
}

fn parse_args() -> anyhow::Result<Args> {
    let mut a = Args { capture: None, at: None, screenshot: None, size: (1600, 900), scale: 1.0, tab: None };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || it.next().with_context(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--capture" => a.capture = Some(PathBuf::from(value()?)),
            "--at" => a.at = Some(value()?),
            "--screenshot" => a.screenshot = Some(PathBuf::from(value()?)),
            "--size" => {
                let v = value()?;
                let (w, h) = v.split_once('x').context("--size WxH")?;
                a.size = (w.parse()?, h.parse()?);
            }
            "--scale" => a.scale = value()?.parse()?,
            "--tab" => a.tab = Some(value()?),
            other => anyhow::bail!("unknown argument {other}"),
        }
    }
    Ok(a)
}

fn main() -> anyhow::Result<()> {
    let args = parse_args()?;
    if let Some(png) = args.screenshot.clone() {
        return screenshot(&args, &png);
    }
    let source = data::Source::start(args.capture.as_deref())?;
    let pending_seek = match (&args.capture, &args.at) {
        (Some(c), Some(at)) => data::resolve_at(c, at),
        _ => None,
    };
    let view = ViewState { source_label: source.label.clone(), ..Default::default() };
    let mut app = App { source, gfx: None, text: None, ui_state: UiState::default(), input: Input::default(), view, pending_seek };
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}

/// Render one moment of a capture (or, without `--capture`, the live game)
/// to a PNG, no window.
fn screenshot(args: &Args, png: &std::path::Path) -> anyhow::Result<()> {
    let mut source = data::Source::start(args.capture.as_deref())?;
    let deadline = Instant::now() + Duration::from_secs(600);
    let wait = |source: &data::Source, done: &dyn Fn(&si_steam::companion::CompanionState) -> bool| -> anyhow::Result<()> {
        while !done(&source.peek()) {
            anyhow::ensure!(Instant::now() < deadline, "timed out waiting for the game state");
            std::thread::sleep(Duration::from_millis(40));
        }
        Ok(())
    };
    let mut target = 0;
    if let Some(capture) = args.capture.as_ref() {
        target = args.at.as_deref().map(|a| data::resolve_at(capture, a).with_context(|| format!("no save matches --at {a}"))).transpose()?.unwrap_or(0);
        wait(&source, &|st| st.source.replay.is_some())?;
        source.send(ReplayCommand::Seek(target));
        wait(&source, &|st| st.source.replay.as_ref().is_some_and(|r| r.index == target))?;
    } else {
        wait(&source, &|st| st.public.is_some() || st.source.mode == "off")?;
    }
    let st = source.latest();
    let mut text = TextCache::new();
    let mut view = ViewState { source_label: source.label.clone(), gpu: "offscreen".into(), ..Default::default() };
    match args.tab.as_deref() {
        Some("rules") => view.rules = true,
        Some(t) => view.pinned = views::PHASES.iter().position(|p| p.eq_ignore_ascii_case(t) || p.split(' ').next().is_some_and(|w| w.eq_ignore_ascii_case(t))),
        None => {}
    }
    let t0 = Instant::now();
    let info = si_ui::offscreen::render_png(png, args.size, args.scale, theme::BG, &mut text, |ui| {
        views::draw(ui, &mut view, &st);
    })?;
    println!("{} · {:?} · save {} → {}", info, t0.elapsed(), target, png.display());
    Ok(())
}

struct Gfx {
    window: Arc<Window>,
    surface: si_ui::wgpu::Surface<'static>,
    device: si_ui::wgpu::Device,
    queue: si_ui::wgpu::Queue,
    config: si_ui::wgpu::SurfaceConfiguration,
    renderer: Renderer,
    gpu: String,
}

impl Gfx {
    async fn new(window: Arc<Window>) -> anyhow::Result<Gfx> {
        use si_ui::wgpu;
        let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
        desc.backends = wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::GL;
        let instance = wgpu::Instance::new(desc);
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { compatible_surface: Some(&surface), ..Default::default() }).await?;
        let gpu = format!("{:?} · {}", adapter.get_info().backend, adapter.get_info().name);
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        let size = window.inner_size();
        let config = surface.get_default_config(&adapter, size.width.max(1), size.height.max(1)).context("surface config")?;
        surface.configure(&device, &config);
        let renderer = Renderer::new(&device, &queue, config.format);
        Ok(Gfx { window, surface, device, queue, config, renderer, gpu })
    }
}

struct App {
    source: data::Source,
    gfx: Option<Gfx>,
    text: Option<TextCache>,
    ui_state: UiState,
    input: Input,
    view: ViewState,
    pending_seek: Option<usize>,
}

fn map_key(k: &WKey) -> Option<Key> {
    Some(match k {
        WKey::Named(NamedKey::ArrowLeft) => Key::Left,
        WKey::Named(NamedKey::ArrowRight) => Key::Right,
        WKey::Named(NamedKey::ArrowUp) => Key::Up,
        WKey::Named(NamedKey::ArrowDown) => Key::Down,
        WKey::Named(NamedKey::Home) => Key::Home,
        WKey::Named(NamedKey::End) => Key::End,
        WKey::Named(NamedKey::PageUp) => Key::PageUp,
        WKey::Named(NamedKey::PageDown) => Key::PageDown,
        WKey::Named(NamedKey::Space) => Key::Space,
        WKey::Named(NamedKey::Escape) => Key::Escape,
        WKey::Character(s) => Key::Char(s.chars().next()?),
        _ => return None,
    })
}

impl App {
    fn redraw(&self) {
        if let Some(g) = &self.gfx {
            g.window.request_redraw();
        }
    }

    fn draw(&mut self) {
        let st = self.source.latest();
        let (Some(g), Some(text)) = (self.gfx.as_mut(), self.text.as_mut()) else { return };
        use si_ui::wgpu;
        let frame = match g.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            _ => {
                g.surface.configure(&g.device, &g.config);
                return;
            }
        };
        let t0 = Instant::now();
        let target = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let scale = g.window.scale_factor() as f32;
        let size = (g.config.width, g.config.height);
        let (out, actions, hand) = {
            let mut ui = Ui::new(text, &mut self.ui_state, &self.input, scale, (size.0 as f32 / scale, size.1 as f32 / scale), g.renderer.srgb);
            let actions = views::draw(&mut ui, &mut self.view, &st);
            let hand = ui.wants_hand;
            (ui.finish(), actions, hand)
        };
        let mut encoder = g.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        if let Err(e) = g.renderer.render(&g.device, &g.queue, &mut encoder, &target, size, &out, text, theme::BG) {
            eprintln!("render: {e:#}");
        }
        g.queue.submit([encoder.finish()]);
        g.window.pre_present_notify();
        g.queue.present(frame);
        text.end_frame();
        g.window.set_cursor(if hand { CursorIcon::Pointer } else { CursorIcon::Default });
        self.view.frame_ms = t0.elapsed().as_secs_f32() * 1000.0;

        self.input.clicked = false;
        self.input.wheel = 0.0;
        self.input.keys.clear();
        for a in actions {
            match a {
                Action::Replay(cmd) => self.source.send(cmd),
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gfx.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Spirit Island companion")
            .with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0))
            .with_min_inner_size(winit::dpi::LogicalSize::new(1200.0, 700.0));
        let window = Arc::new(el.create_window(attrs).expect("create window"));
        match pollster::block_on(Gfx::new(window)) {
            Ok(g) => {
                self.view.gpu = g.gpu.clone();
                self.gfx = Some(g);
            }
            Err(e) => {
                eprintln!("GPU setup failed: {e:#}");
                el.exit();
                return;
            }
        }
        if self.text.is_none() {
            self.text = Some(TextCache::new());
        }
        self.redraw();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let scale = self.gfx.as_ref().map(|g| g.window.scale_factor() as f32).unwrap_or(1.0);
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                if let Some(g) = self.gfx.as_mut() {
                    g.config.width = s.width.max(1);
                    g.config.height = s.height.max(1);
                    g.surface.configure(&g.device, &g.config);
                }
                self.redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => self.redraw(),
            WindowEvent::CursorMoved { position, .. } => {
                self.input.mouse = Some((position.x as f32 / scale, position.y as f32 / scale));
                self.redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.input.mouse = None;
                self.redraw();
            }
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                self.input.clicked = true;
                self.redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.input.wheel += match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 40.0,
                };
                self.redraw();
            }
            WindowEvent::ModifiersChanged(m) => self.input.shift = m.state().shift_key(),
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                if let Some(k) = map_key(&event.logical_key) {
                    self.input.keys.push(k);
                    self.redraw();
                }
            }
            WindowEvent::RedrawRequested => self.draw(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        if let Some(i) = self.pending_seek
            && self.source.peek().source.replay.is_some()
        {
            self.source.send(ReplayCommand::Seek(i));
            self.pending_seek = None;
        }
        if self.source.changed() {
            self.redraw();
        }
        el.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(60)));
    }
}
