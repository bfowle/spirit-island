// Triangle meshes with per-vertex color (premultiplied) and a clip rect;
// coordinates in physical pixels. Land fills and arrow heads.

struct Uniforms {
    screen: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct Vert {
    @location(0) pos: vec2<f32>,
    @location(1) _pad: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) clip: vec4<f32>,
};

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) p: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) clip: vec4<f32>,
};

@vertex
fn vs_main(v: Vert) -> VOut {
    var o: VOut;
    o.pos = vec4<f32>(v.pos.x / u.screen.x * 2.0 - 1.0, 1.0 - v.pos.y / u.screen.y * 2.0, 0.0, 1.0);
    o.p = v.pos;
    o.color = v.color;
    o.clip = v.clip;
    return o;
}

@fragment
fn fs_main(i: VOut) -> @location(0) vec4<f32> {
    if (i.p.x < i.clip.x || i.p.y < i.clip.y || i.p.x >= i.clip.z || i.p.y >= i.clip.w) {
        discard;
    }
    return i.color;
}
