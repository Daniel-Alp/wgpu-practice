struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-0.75, -0.75),
        vec2<f32>( 0.75,  -0.75),
        vec2<f32>(  0.0,  0.75),
    );

    var col = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );

    var out: VertexOut;
    out.position = vec4<f32>(pos[i], 0.0, 1.0);
    out.color = col[i];
    return out;
}

@fragment
// @location(0) color attachment that we are writing to
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}