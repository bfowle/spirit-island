// Textured quads (images registered with the renderer), premultiplied,
// tinted per draw, clipped per draw. Coordinates in physical pixels.

struct Uniforms {
    screen: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(1) @binding(0) var t: texture_2d<f32>;
@group(1) @binding(1) var s: sampler;

struct VIn {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tint: vec4<f32>,
    @location(3) clip: vec4<f32>,
};

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) p: vec2<f32>,
    @location(2) @interpolate(flat) tint: vec4<f32>,
    @location(3) @interpolate(flat) clip: vec4<f32>,
};

@vertex
fn vs_main(i: VIn) -> VOut {
    var o: VOut;
    o.pos = vec4<f32>(i.pos.x / u.screen.x * 2.0 - 1.0, 1.0 - i.pos.y / u.screen.y * 2.0, 0.0, 1.0);
    o.uv = i.uv;
    o.p = i.pos;
    o.tint = i.tint;
    o.clip = i.clip;
    return o;
}

@fragment
fn fs_main(i: VOut) -> @location(0) vec4<f32> {
    if (i.p.x < i.clip.x || i.p.y < i.clip.y || i.p.x >= i.clip.z || i.p.y >= i.clip.w) {
        discard;
    }
    return textureSample(t, s, i.uv) * i.tint;
}
