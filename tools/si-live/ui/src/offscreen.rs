//! Render one frame without a window and save it as a PNG: screenshot tests
//! and design review on the same GPU and fonts the app uses.

use crate::painter::{Frame, Input, Ui, UiState};
use crate::render::Renderer;
use crate::text::TextCache;
use crate::theme::Rgba;
use std::path::Path;

pub fn render_png(path: &Path, size: (u32, u32), scale: f32, clear: Rgba, text: &mut TextCache, build: impl FnOnce(&mut Ui)) -> anyhow::Result<String> {
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::GL;
    let instance = wgpu::Instance::new(desc);
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))?;
    let info = format!("{:?} · {}", adapter.get_info().backend, adapter.get_info().name);
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut renderer = Renderer::new(&device, &queue, format);

    let input = Input::default();
    let mut state = UiState::default();
    let frame: Frame = {
        let mut ui = Ui::new(text, &mut state, &input, scale, (size.0 as f32 / scale, size.1 as f32 / scale), true);
        build(&mut ui);
        ui.finish()
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("si-ui offscreen"),
        size: wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let row = size.0 * 4;
    let padded = row.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("si-ui readback"),
        size: (padded * size.1) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    renderer.render(&device, &queue, &mut encoder, &view, size, &frame, text, clear)?;
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(size.1) },
        },
        wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
    );
    queue.submit([encoder.finish()]);
    let slice = readback.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::PollType::wait_indefinitely())?;
    let data = slice.get_mapped_range().map_err(|e| anyhow::anyhow!("map readback: {e:?}"))?;
    let mut pixels = Vec::with_capacity((row * size.1) as usize);
    for y in 0..size.1 as usize {
        let start = y * padded as usize;
        pixels.extend_from_slice(&data[start..start + row as usize]);
    }
    drop(data);
    readback.unmap();

    let file = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), size.0, size.1);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(&pixels)?;
    Ok(info)
}
