use crate::{Vec2, Vec3, Vec4};

/// The supported Per-Vertex graphics data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    // The vertex position in model space.
    pub pos: [f32; 4],

    // The rgba color to be applied to this vertex.
    pub rgba: [f32; 4],

    // The texture coordinate associated with the vertex.
    pub uv: [f32; 2],

    // The texture index controls which texture will be applied to the vertex
    // when resterizing.
    // Defaults to 0.
    pub texture_index: i32,

    // Padding required for proper alignment inside the buffer.
    // See the OpenGL spec for notes regarding structure padding when elements
    // are stored in a SSBO:
    // https://www.khronos.org/registry/OpenGL/specs/gl/glspec45.core.pdf#page=159
    //
    // The bit that's relevant here:
    // > The structure may have padding at the end;
    // > the base offset of the member following the sub-structure is rounded
    // > up to the next multiple of the base alignment of the structure.
    //
    // Where the base alignment of the structure is:
    // > The base alignment of the structure is N , where N is the largest base
    // > alignment value of any of its members, and rounded up to the base
    // > alignment of a vec4.
    //
    // In this case, the base alignment is that of a vec4: 16. So the shader
    // will assume that every Vertex in the SSBO is aligned to 16 bytes. The
    // easiest way for us to manage that is to pad the structure so it's total
    // size is a multiple of 16 bytes. Hence, padding.
    pub _pad: i32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0, 1.0],
            rgba: [1.0, 1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
            texture_index: 0,
            _pad: 0,
        }
    }
}

impl Vertex {
    /// Create a new Vertex using nalgebra vectors.
    pub fn new(pos: Vec3, rgba: Vec4, uv: Vec2, texture_index: i32) -> Vertex {
        Self {
            pos: [pos.x, pos.y, pos.z, 1.0],
            rgba: rgba.into(),
            uv: uv.into(),
            texture_index,
            _pad: 0,
        }
    }
}
