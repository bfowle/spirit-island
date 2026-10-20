//! The renderer: meshes (land fills, arrow heads), SDF shapes (rounded
//! rectangles, lines, glows), glyphon text and icons, drawn layer by layer.

use crate::painter::{Frame, MeshVertex, RectInstance};
use crate::text::TextCache;
use crate::theme::Rgba;
use glyphon::{
    Buffer, Cache, Color, ContentType, CustomGlyph, Metrics, RasterizeCustomGlyphRequest, RasterizedCustomGlyph, Resolution, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use std::collections::HashMap;

/// An icon's source: an SVG drawn as a tintable mask, or a full-color image.
#[derive(Clone)]
pub enum IconSource {
    /// SVG; its alpha becomes a mask the draw call tints.
    Mask(Vec<u8>),
    /// SVG or PNG, drawn in its own colors.
    Color(Vec<u8>),
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    mesh_pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    instances: wgpu::Buffer,
    capacity: usize,
    vertices: wgpu::Buffer,
    vertex_capacity: usize,
    indices: wgpu::Buffer,
    index_capacity: usize,
    atlas: TextAtlas,
    viewport: Viewport,
    text: [TextRenderer; 3],
    swash: SwashCache,
    icons: HashMap<u16, IconSource>,
    empty: Option<Buffer>,
    /// Images as textures: bind group and size, by id.
    image_pipeline: wgpu::RenderPipeline,
    image_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    images: HashMap<u16, (wgpu::BindGroup, u32, u32)>,
    image_vertices: wgpu::Buffer,
    image_capacity: usize,
    pub srgb: bool,
}

/// A textured-quad vertex: physical position, texture coordinates,
/// premultiplied tint, clip.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ImageVertex {
    pos: [f32; 2],
    uv: [f32; 2],
    tint: [f32; 4],
    clip: [f32; 4],
}

const INSTANCE_SIZE: usize = std::mem::size_of::<RectInstance>();
const IMAGE_VERTEX_SIZE: usize = std::mem::size_of::<ImageVertex>();
const VERTEX_SIZE: usize = std::mem::size_of::<MeshVertex>();
const PNG_SIGNATURE: &[u8] = &[0x89, b'P', b'N', b'G'];

fn buffer(device: &wgpu::Device, label: &str, bytes: usize, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: bytes.max(16) as u64, usage: usage | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false })
}

fn pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    buffers: &[Option<wgpu::VertexBufferLayout>],
    topology: wgpu::PrimitiveTopology,
    format: wgpu::TextureFormat,
    label: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState { module: shader, entry_point: Some("vs_main"), buffers, compilation_options: wgpu::PipelineCompilationOptions::default() },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState { topology, ..Default::default() },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        cache: None,
        multiview_mask: None,
    })
}

/// Rasterize an icon to `w`×`h`: SVG through resvg, PNG decoded and scaled.
fn rasterize(source: &IconSource, w: u32, h: u32) -> Option<RasterizedCustomGlyph> {
    let (bytes, mask) = match source {
        IconSource::Mask(b) => (b, true),
        IconSource::Color(b) => (b, false),
    };
    let mut pixmap = resvg::tiny_skia::Pixmap::new(w.max(1), h.max(1))?;
    if bytes.starts_with(PNG_SIGNATURE) {
        let png = resvg::tiny_skia::Pixmap::decode_png(bytes).ok()?;
        let s = (w as f32 / png.width() as f32).min(h as f32 / png.height() as f32);
        let paint = resvg::tiny_skia::PixmapPaint { quality: resvg::tiny_skia::FilterQuality::Bicubic, ..Default::default() };
        pixmap.draw_pixmap(0, 0, png.as_ref(), &paint, resvg::tiny_skia::Transform::from_scale(s, s), None);
    } else {
        let tree = resvg::usvg::Tree::from_data(bytes, &resvg::usvg::Options::default()).ok()?;
        let size = tree.size();
        let s = (w as f32 / size.width()).min(h as f32 / size.height());
        let dx = (w as f32 - size.width() * s) / 2.0;
        let dy = (h as f32 - size.height() * s) / 2.0;
        resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(s, s).post_translate(dx, dy), &mut pixmap.as_mut());
    }
    let data = if mask {
        pixmap.pixels().iter().map(|p| p.alpha()).collect()
    } else {
        pixmap
            .pixels()
            .iter()
            .flat_map(|p| {
                let c = p.demultiply();
                [c.red(), c.green(), c.blue(), c.alpha()]
            })
            .collect()
    };
    Some(RasterizedCustomGlyph { data, content_type: if mask { ContentType::Mask } else { ContentType::Color } })
}

impl Renderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
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
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("si-ui"), bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });

        let rect_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("si-ui rects"), source: wgpu::ShaderSource::Wgsl(include_str!("rect.wgsl").into()) });
        let rect_attrs = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4];
        let rects = pipeline(
            device,
            &layout,
            &rect_shader,
            &[Some(wgpu::VertexBufferLayout { array_stride: INSTANCE_SIZE as u64, step_mode: wgpu::VertexStepMode::Instance, attributes: &rect_attrs })],
            wgpu::PrimitiveTopology::TriangleStrip,
            format,
            "si-ui rects",
        );
        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("si-ui mesh"), source: wgpu::ShaderSource::Wgsl(include_str!("mesh.wgsl").into()) });
        let mesh_attrs = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4];
        let mesh = pipeline(
            device,
            &layout,
            &mesh_shader,
            &[Some(wgpu::VertexBufferLayout { array_stride: VERTEX_SIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &mesh_attrs })],
            wgpu::PrimitiveTopology::TriangleList,
            format,
            "si-ui mesh",
        );

        // Images: a texture and sampler per image, drawn as quads.
        let image_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("si-ui image"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });
        let image_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("si-ui image"), bind_group_layouts: &[Some(&bgl), Some(&image_layout)], immediate_size: 0 });
        let image_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("si-ui image"), source: wgpu::ShaderSource::Wgsl(include_str!("image.wgsl").into()) });
        let image_attrs = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4];
        let image_pipeline = pipeline(
            device,
            &image_pipeline_layout,
            &image_shader,
            &[Some(wgpu::VertexBufferLayout { array_stride: IMAGE_VERTEX_SIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &image_attrs })],
            wgpu::PrimitiveTopology::TriangleList,
            format,
            "si-ui image",
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("si-ui image"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let text = [
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None),
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None),
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None),
        ];
        Renderer {
            pipeline: rects,
            mesh_pipeline: mesh,
            uniforms,
            bind_group,
            instances: buffer(device, "si-ui rects", 1024 * INSTANCE_SIZE, wgpu::BufferUsages::VERTEX),
            capacity: 1024,
            vertices: buffer(device, "si-ui mesh", 4096 * VERTEX_SIZE, wgpu::BufferUsages::VERTEX),
            vertex_capacity: 4096,
            indices: buffer(device, "si-ui mesh indices", 8192 * 4, wgpu::BufferUsages::INDEX),
            index_capacity: 8192,
            atlas,
            viewport,
            text,
            swash: SwashCache::new(),
            icons: HashMap::new(),
            empty: None,
            image_pipeline,
            image_layout,
            sampler,
            images: HashMap::new(),
            image_vertices: buffer(device, "si-ui images", 64 * 6 * IMAGE_VERTEX_SIZE, wgpu::BufferUsages::VERTEX),
            image_capacity: 64 * 6,
            srgb: format.is_srgb(),
        }
    }

    /// Register an icon for `Ui::icon(…, id, …)`.
    pub fn add_icon(&mut self, id: u16, source: IconSource) {
        self.icons.insert(id, source);
    }

    /// Register a PNG as an image for `Ui::image(…, id, …)`: decoded once,
    /// uploaded as a texture. Returns its size, or `None` if it can't be read.
    pub fn add_image(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, id: u16, png: &[u8]) -> Option<(u32, u32)> {
        let pixmap = resvg::tiny_skia::Pixmap::decode_png(png).ok()?;
        let (w, h) = (pixmap.width(), pixmap.height());
        // The PNG's colors are sRGB; sampled through an sRGB view they come
        // out linear for an sRGB target, raw for a plain one.
        let format = if self.srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("si-ui image"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // tiny-skia pixels are premultiplied RGBA, which is what the blend expects.
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixmap.data(),
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("si-ui image"),
            layout: &self.image_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.images.insert(id, (bind_group, w, h));
        Some((w, h))
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

        // Upload every layer's shapes and meshes into one buffer each.
        let rects: Vec<RectInstance> = frame.layers.iter().flat_map(|l| l.rects.iter().copied()).collect();
        if rects.len() > self.capacity {
            self.capacity = rects.len().next_power_of_two();
            self.instances = buffer(device, "si-ui rects", self.capacity * INSTANCE_SIZE, wgpu::BufferUsages::VERTEX);
        }
        if !rects.is_empty() {
            queue.write_buffer(&self.instances, 0, bytemuck::cast_slice(&rects));
        }
        let mut verts: Vec<MeshVertex> = Vec::new();
        let mut idx: Vec<u32> = Vec::new();
        let mut mesh_ranges = Vec::new();
        for l in &frame.layers {
            let base = verts.len() as u32;
            let start = idx.len() as u32;
            verts.extend_from_slice(&l.mesh);
            idx.extend(l.indices.iter().map(|i| i + base));
            mesh_ranges.push(start..idx.len() as u32);
        }
        if verts.len() > self.vertex_capacity {
            self.vertex_capacity = verts.len().next_power_of_two();
            self.vertices = buffer(device, "si-ui mesh", self.vertex_capacity * VERTEX_SIZE, wgpu::BufferUsages::VERTEX);
        }
        if idx.len() > self.index_capacity {
            self.index_capacity = idx.len().next_power_of_two();
            self.indices = buffer(device, "si-ui mesh indices", self.index_capacity * 4, wgpu::BufferUsages::INDEX);
        }
        if !verts.is_empty() {
            queue.write_buffer(&self.vertices, 0, bytemuck::cast_slice(&verts));
            queue.write_buffer(&self.indices, 0, bytemuck::cast_slice(&idx));
        }
        // Every layer's images as quads: six vertices each, in draw order.
        let mut image_verts: Vec<ImageVertex> = Vec::new();
        let mut image_draws: Vec<Vec<(u16, u32)>> = Vec::new();
        for l in &frame.layers {
            let mut draws = Vec::new();
            for im in l.images.iter().filter(|im| self.images.contains_key(&im.id)) {
                let [x, y, w, h] = im.rect;
                let v = |px: f32, py: f32, u: f32, vv: f32| ImageVertex { pos: [px, py], uv: [u, vv], tint: im.tint, clip: im.clip };
                let start = image_verts.len() as u32;
                image_verts.extend([v(x, y, 0.0, 0.0), v(x + w, y, 1.0, 0.0), v(x + w, y + h, 1.0, 1.0), v(x, y, 0.0, 0.0), v(x + w, y + h, 1.0, 1.0), v(x, y + h, 0.0, 1.0)]);
                draws.push((im.id, start));
            }
            image_draws.push(draws);
        }
        if image_verts.len() > self.image_capacity {
            self.image_capacity = image_verts.len().next_power_of_two();
            self.image_vertices = buffer(device, "si-ui images", self.image_capacity * IMAGE_VERTEX_SIZE, wgpu::BufferUsages::VERTEX);
        }
        if !image_verts.is_empty() {
            queue.write_buffer(&self.image_vertices, 0, bytemuck::cast_slice(&image_verts));
        }

        self.viewport.update(queue, Resolution { width: w, height: h });
        if self.empty.is_none() {
            self.empty = Some(Buffer::new(&mut text.fonts, Metrics::new(1.0, 1.0)));
        }
        let Renderer { text: renderers, atlas, viewport, swash, icons, empty, .. } = self;
        let empty = empty.as_ref().expect("empty buffer");
        for (i, layer) in frame.layers.iter().enumerate() {
            let glyphs: Vec<[CustomGlyph; 1]> = layer
                .icons
                .iter()
                .map(|ic| {
                    [CustomGlyph {
                        id: ic.id,
                        left: 0.0,
                        top: 0.0,
                        width: ic.w,
                        height: ic.h,
                        color: Some(Color::rgba(ic.color.0, ic.color.1, ic.color.2, ic.color.3)),
                        snap_to_physical_pixel: true,
                        metadata: 0,
                    }]
                })
                .collect();
            let mut areas: Vec<TextArea> = layer
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
            areas.extend(layer.icons.iter().zip(&glyphs).map(|(ic, g)| TextArea {
                buffer: empty,
                left: ic.x,
                top: ic.y,
                scale: ic.scale,
                bounds: TextBounds { left: ic.clip[0], top: ic.clip[1], right: ic.clip[2], bottom: ic.clip[3] },
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: g,
            }));
            renderers[i].prepare_with_custom(device, queue, &mut text.fonts, atlas, viewport, areas, swash, |req: RasterizeCustomGlyphRequest| {
                icons.get(&req.id).and_then(|src| rasterize(src, req.width as u32, req.height as u32))
            })?;
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
            let r = &mesh_ranges[i];
            if r.end > r.start {
                pass.set_pipeline(&self.mesh_pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.set_vertex_buffer(0, self.vertices.slice(..));
                pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(r.clone(), 0, 0..1);
            }
            if !image_draws[i].is_empty() {
                pass.set_pipeline(&self.image_pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.set_vertex_buffer(0, self.image_vertices.slice(..));
                for (id, start) in &image_draws[i] {
                    if let Some((bg, _, _)) = self.images.get(id) {
                        pass.set_bind_group(1, bg, &[]);
                        pass.draw(*start..*start + 6, 0..1);
                    }
                }
            }
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
