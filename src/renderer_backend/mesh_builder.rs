use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static>
    {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
            wgpu::VertexAttribute{
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute{
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 3]>() as u64,
                shader_location: 1
            }
        ];

        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] 
{
    unsafe{ core::slice::from_raw_parts((p as *const T) as *const u8, core::mem::size_of::<T>()) }
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer 
{
    let vertices = [
        Vertex{position: [-0.75, -0.75, 0.0], color: [1.0, 0.0, 0.0]},
        Vertex{position: [ 0.75, -0.75, 0.0], color: [0.0, 1.0, 0.0]},
        Vertex{position: [ 0.00,  0.75, 0.0], color: [0.0, 0.0, 1.0]},
    ];

    let bytes = unsafe { any_as_u8_slice(&vertices) };

    let buf_desc = wgpu::util::BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX
    };

    let buf = device.create_buffer_init(&buf_desc);
    buf
}