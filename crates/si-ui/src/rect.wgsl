// Rounded rectangles and lines with an optional border and soft edge,
// antialiased by a signed distance field. One instance per shape;
// coordinates in physical pixels.

struct Uniforms {
    screen: vec2<f32>,
    _pad: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct Inst {
    // kind 0: x, y, w, h. kind 1 (line): x0, y0, x1, y1.
    @location(0) rect: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) border_color: vec4<f32>,
    // radius (half width for a line), border width, kind, softness
    @location(3) params: vec4<f32>,
    // x0, y0, x1, y1
    @location(4) clip: vec4<f32>,
};

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) p: vec2<f32>,
    @location(1) @interpolate(flat) rect: vec4<f32>,
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) border_color: vec4<f32>,
    @location(4) @interpolate(flat) params: vec4<f32>,
    @location(5) @interpolate(flat) clip: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Inst) -> VOut {
    let corner = vec2<f32>(f32(vi & 1u), f32((vi >> 1u) & 1u));
    // Margin for the antialiased edge and any softness.
    let m = 1.0 + inst.params.w;
    var lo: vec2<f32>;
    var hi: vec2<f32>;
    if (inst.params.z > 0.5) {
        let r = inst.params.x + m;
        lo = min(inst.rect.xy, inst.rect.zw) - vec2<f32>(r);
        hi = max(inst.rect.xy, inst.rect.zw) + vec2<f32>(r);
    } else {
        lo = inst.rect.xy - vec2<f32>(m);
        hi = inst.rect.xy + inst.rect.zw + vec2<f32>(m);
    }
    let p = lo + corner * (hi - lo);
    var o: VOut;
    o.pos = vec4<f32>(p.x / u.screen.x * 2.0 - 1.0, 1.0 - p.y / u.screen.y * 2.0, 0.0, 1.0);
    o.p = p;
    o.rect = inst.rect;
    o.color = inst.color;
    o.border_color = inst.border_color;
    o.params = inst.params;
    o.clip = inst.clip;
    return o;
}

fn sd_round_box(p: vec2<f32>, half: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    return length(pa - ba * h);
}

@fragment
fn fs_main(i: VOut) -> @location(0) vec4<f32> {
    if (i.p.x < i.clip.x || i.p.y < i.clip.y || i.p.x >= i.clip.z || i.p.y >= i.clip.w) {
        discard;
    }
    var d: f32;
    if (i.params.z > 0.5) {
        d = sd_segment(i.p, i.rect.xy, i.rect.zw) - i.params.x;
    } else {
        let half = i.rect.zw * 0.5;
        let r = min(i.params.x, min(half.x, half.y));
        d = sd_round_box(i.p - (i.rect.xy + half), half, r);
    }
    // A crisp 1-pixel edge, or a soft falloff over `softness` pixels.
    let soft = max(i.params.w, 1.0);
    var coverage = clamp(0.5 - d / soft, 0.0, 1.0);
    if (i.params.w > 0.0) {
        coverage = clamp(1.0 - d / soft, 0.0, 1.0);
        coverage = coverage * coverage;
    }
    // Premultiplied colors, so a border over a transparent fill doesn't darken.
    let fill = vec4<f32>(i.color.rgb * i.color.a, i.color.a);
    var c = fill;
    let bw = i.params.y;
    if (bw > 0.0) {
        let border = vec4<f32>(i.border_color.rgb * i.border_color.a, i.border_color.a);
        let inner = clamp(0.5 - (d + bw), 0.0, 1.0);
        c = mix(border, fill, inner);
    }
    return c * coverage;
}
