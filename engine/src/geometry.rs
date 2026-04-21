use bytemuck::{Pod, Zeroable};

/// A single vertex: position + RGB face color.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color:    [f32; 3],
}

const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &ATTRIBS,
        }
    }
}

// ── unit cube: 6 faces × 4 unique vertices = 24 vertices ──────────────────────

/// Front  (+Z) — warm orange
const C_FRONT:  [f32; 3] = [1.00, 0.45, 0.10];
/// Back   (−Z) — cool indigo
const C_BACK:   [f32; 3] = [0.20, 0.35, 1.00];
/// Top    (+Y) — bright yellow
const C_TOP:    [f32; 3] = [1.00, 0.90, 0.10];
/// Bottom (−Y) — deep violet
const C_BOT:    [f32; 3] = [0.60, 0.10, 0.90];
/// Right  (+X) — emerald green
const C_RIGHT:  [f32; 3] = [0.10, 0.88, 0.35];
/// Left   (−X) — crimson
const C_LEFT:   [f32; 3] = [0.90, 0.10, 0.30];

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    // Front (+Z) — CCW viewed from +Z
    Vertex { position: [-0.5, -0.5,  0.5], color: C_FRONT },
    Vertex { position: [ 0.5, -0.5,  0.5], color: C_FRONT },
    Vertex { position: [ 0.5,  0.5,  0.5], color: C_FRONT },
    Vertex { position: [-0.5,  0.5,  0.5], color: C_FRONT },

    // Back (−Z) — CCW viewed from −Z
    Vertex { position: [ 0.5, -0.5, -0.5], color: C_BACK },
    Vertex { position: [-0.5, -0.5, -0.5], color: C_BACK },
    Vertex { position: [-0.5,  0.5, -0.5], color: C_BACK },
    Vertex { position: [ 0.5,  0.5, -0.5], color: C_BACK },

    // Right (+X) — CCW viewed from +X
    Vertex { position: [ 0.5, -0.5,  0.5], color: C_RIGHT },
    Vertex { position: [ 0.5, -0.5, -0.5], color: C_RIGHT },
    Vertex { position: [ 0.5,  0.5, -0.5], color: C_RIGHT },
    Vertex { position: [ 0.5,  0.5,  0.5], color: C_RIGHT },

    // Left (−X) — CCW viewed from −X
    Vertex { position: [-0.5, -0.5, -0.5], color: C_LEFT },
    Vertex { position: [-0.5, -0.5,  0.5], color: C_LEFT },
    Vertex { position: [-0.5,  0.5,  0.5], color: C_LEFT },
    Vertex { position: [-0.5,  0.5, -0.5], color: C_LEFT },

    // Top (+Y) — CCW viewed from +Y
    Vertex { position: [-0.5,  0.5,  0.5], color: C_TOP },
    Vertex { position: [ 0.5,  0.5,  0.5], color: C_TOP },
    Vertex { position: [ 0.5,  0.5, -0.5], color: C_TOP },
    Vertex { position: [-0.5,  0.5, -0.5], color: C_TOP },

    // Bottom (−Y) — CCW viewed from −Y
    Vertex { position: [-0.5, -0.5, -0.5], color: C_BOT },
    Vertex { position: [ 0.5, -0.5, -0.5], color: C_BOT },
    Vertex { position: [ 0.5, -0.5,  0.5], color: C_BOT },
    Vertex { position: [-0.5, -0.5,  0.5], color: C_BOT },
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
     0,  1,  2,   0,  2,  3, // Front
     4,  5,  6,   4,  6,  7, // Back
     8,  9, 10,   8, 10, 11, // Right
    12, 13, 14,  12, 14, 15, // Left
    16, 17, 18,  16, 18, 19, // Top
    20, 21, 22,  20, 22, 23, // Bottom
];
