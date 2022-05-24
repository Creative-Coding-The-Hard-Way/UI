use ::anyhow::Result;

mod sprite;
mod vertex;

pub mod triangles;

pub use self::{sprite::Sprite, vertex::Vertex};

/// Types which implement this trait manage a stream of vertices which are
/// rendered to the screen during the current frame.
pub trait VertexStream {
    /// Push vertices into the frame. Indices index into the given vertex slice.
    fn push_vertices(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Result<()>;
}
