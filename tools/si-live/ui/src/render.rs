//! The renderer: one wgpu pipeline for rounded rectangles (instanced quads,
//! SDF edges) and glyphon for text, drawn layer by layer.

use crate::painter::{Frame, RectInstance};
use crate::text::TextCache;
use crate::theme::Rgba;
use glyphon::{Cache, Color, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    instances: wgpu::Buffer,
    capacity: usize,
    atlas: TextAtlas,
    viewport: Viewport,
    text: [TextRenderer; 2],
    swash: SwashCache,
    pub srgb: bool,
}

const INSTANCE_SIZE: usize = std::mem::size_of::<RectInstance>();

fn instance_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("si-ui rect instances"),
        size: (capacity * INSTANCE_SIZE) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

impl Renderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("si-ui rects"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
        });
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("si-ui uniforms"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("si-ui uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("si-ui uniforms"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() }],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("si-ui rects"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        let attributes = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("si-ui rects"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: INSTANCE_SIZE as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &attributes,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleStrip, ..Default::default() },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            cache: None,
            multiview_mask: None,
        });
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let text = [
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None),
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None),
        ];
        let capacity = 1024;
        Renderer {
            pipeline,
            uniforms,
            bind_group,
            instances: instance_buffer(device, capacity),
            capacity,
            atlas,
            viewport,
            text,
            swash: SwashCache::new(),
            srgb: format.is_srgb(),
        }
    }

    /// Draw `frame` into `view` (cleared to `clear`).
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        size: (u32, u32),
        frame: &Frame,
        text: &mut TextCache,
        clear: Rgba,
    ) -> anyhow::Result<()> {
        self.atlas.trim();
        let (w, h) = (size.0.max(1), size.1.max(1));
        queue.write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&[w as f32, h as f32, 0.0, 0.0]));

        let all: Vec<RectInstance> = frame.layers.iter().flat_map(|l| l.rects.iter().copied()).collect();
        if all.len() > self.capacity {
            self.capacity = all.len().next_power_of_two();
            self.instances = instance_buffer(device, self.capacity);
        }
        if !all.is_empty() {
            queue.write_buffer(&self.instances, 0, bytemuck::cast_slice(&all));
        }

        self.viewport.update(queue, Resolution { width: w, height: h });
        for (i, layer) in frame.layers.iter().enumerate() {
            let areas: Vec<TextArea> = layer
                .texts
                .iter()
                .filter_map(|t| {
                    let e = text.entries.get(&t.key)?;
                    Some(TextArea {
                        buffer: &e.buffer,
                        left: t.x,
                        top: t.y,
                        scale: t.scale,
                        bounds: TextBounds { left: t.clip[0], top: t.clip[1], right: t.clip[2], bottom: t.clip[3] },
                        default_color: Color::rgb(228, 230, 239),
                        custom_glyphs: &[],
                    })
                })
                .collect();
            self.text[i].prepare(device, queue, &mut text.fonts, &mut self.atlas, &self.viewport, areas, &mut self.swash)?;
        }

        let bg = if self.srgb { clear.linear() } else { clear.unorm() };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("si-ui frame"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: bg[0] as f64, g: bg[1] as f64, b: bg[2] as f64, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        let mut start = 0u32;
        for (i, layer) in frame.layers.iter().enumerate() {
            let n = layer.rects.len() as u32;
            if n > 0 {
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.set_vertex_buffer(0, self.instances.slice(..));
                pass.draw(0..4, start..start + n);
            }
            start += n;
            self.text[i].render(&self.atlas, &self.viewport, &mut pass)?;
        }
        Ok(())
    }
}
