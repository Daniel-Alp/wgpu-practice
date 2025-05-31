struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32> 
}

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
// @location(0) color attachment that we are writing to
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}