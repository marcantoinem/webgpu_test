#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub fn generate_vertices_circle(nb_vertices: u32, color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    assert!(nb_vertices >= 4);
    let angle_increment = 2.0 * std::f32::consts::PI / nb_vertices as f32;
    let mut angle: f32 = 0.0;
    let mut vertices = vec![];
    for _ in 0..nb_vertices {
        vertices.push(Vertex {
            position: [0.5 * angle.cos(), 0.5 * angle.sin(), 0.0],
            color,
        });
        angle += angle_increment;
    }
    let mut indexes = vec![];
    for i in 0..(nb_vertices - 2) {
        indexes.push(i as u16);
        indexes.push(i as u16 + 1);
        indexes.push(nb_vertices as u16 - 1);
    }
    (vertices, indexes)
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
