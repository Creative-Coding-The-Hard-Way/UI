use crate::graphics2::{Frame, Vec2, Vec3, Vec4, Vertex};

use anyhow::Result;

/// A rectangle is a primitive with a bottom, top, left, and right.
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Rect {
    /// Create a new rectange with the given coordinates.
    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    /// The Width of the rectangle, always positive.
    pub fn width(&self) -> f32 {
        (self.right - self.left).abs()
    }

    /// The height of the rectangle, always positive.
    pub fn height(&self) -> f32 {
        (self.top - self.bottom).abs()
    }
}

/// A Quad is a renderable type which has coordinates in world space,
/// texture coordinates.
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub model: Rect,
    pub uv: Rect,
    pub depth: f32,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            model: Rect::new(1.0, -1.0, -1.0, 1.0),
            uv: Rect::new(0.0, 0.0, 1.0, 1.0),
            depth: 0.0,
        }
    }
}

impl Quad {
    pub fn draw(
        &self,
        frame: &mut Frame,
        color: Vec4,
        tex_index: i32,
    ) -> Result<()> {
        frame.push_vertices(
            &[
                Vertex::new(
                    Vec3::new(self.model.left, self.model.top, self.depth),
                    color,
                    Vec2::new(self.uv.left, self.uv.top),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.model.right, self.model.top, self.depth),
                    color,
                    Vec2::new(self.uv.right, self.uv.top),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.model.right, self.model.bottom, self.depth),
                    color,
                    Vec2::new(self.uv.right, self.uv.bottom),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.model.left, self.model.bottom, self.depth),
                    color,
                    Vec2::new(self.uv.left, self.uv.bottom),
                    tex_index,
                ),
            ],
            &[
                0, 1, 2, // top triangle
                2, 3, 0, // bottom triangle
            ],
        )
    }
}
