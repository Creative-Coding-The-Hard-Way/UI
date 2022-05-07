use anyhow::Result;

use crate::{
    immediate_mode_graphics::{Drawable, Frame, Vertex},
    ui2::primitives::{Line, Rect},
    vec2, vec3, vec4, Vec4,
};

/// A Tile is a rectangular area which can be rendered with a texture and
/// color.
#[derive(Debug, Copy, Clone)]
pub struct Tile {
    /// The coordinates in world space. This dictates the size and
    /// position of the tile.
    pub model: Rect,

    /// The coordinates in texture space. This controls which part of the
    /// texture is painted to the tile when rendered. Defaults to the entire
    /// texture.
    pub uv: Rect,

    /// The tile's world space depth. Defaults to 0.0.
    pub depth: f32,

    /// The tile's rgba color, defaults to white.
    pub color: Vec4,

    /// The line width to use when rendering the tile's outline.
    /// Defaults to 1.0.
    pub outline_width: f32,

    /// The texture to use when rendering the tile's outline.
    /// Defaults to 0.
    pub outline_texture_index: i32,

    /// The texture index to use when rendering the tile.
    /// Defaults to 0.
    pub texture_index: i32,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            model: Rect::new(1.0, -1.0, -1.0, 1.0),
            uv: Rect::new(0.0, 0.0, 1.0, 1.0),
            depth: 0.0,
            color: vec4(1.0, 1.0, 1.0, 1.0),
            outline_width: 1.0,
            outline_texture_index: 0,
            texture_index: 0,
        }
    }
}

impl Drawable for Tile {
    fn fill(&self, frame: &mut Frame) -> Result<()> {
        frame.push_vertices(
            &[
                Vertex::new(
                    vec3(self.model.left(), self.model.top(), self.depth),
                    self.color,
                    vec2(self.uv.left(), self.uv.top()),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(self.model.right(), self.model.top(), self.depth),
                    self.color,
                    vec2(self.uv.right(), self.uv.top()),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(self.model.right(), self.model.bottom(), self.depth),
                    self.color,
                    vec2(self.uv.right(), self.uv.bottom()),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(self.model.left(), self.model.bottom(), self.depth),
                    self.color,
                    vec2(self.uv.left(), self.uv.bottom()),
                    self.texture_index,
                ),
            ],
            &[
                0, 1, 2, // top triangle
                2, 3, 0, // bottom triangle
            ],
        )
    }

    fn outline(&self, frame: &mut Frame) -> Result<()> {
        let top_left = self.model.top_left;
        let top_right = vec2(self.model.right(), self.model.top());
        let bottom_left = vec2(self.model.left(), self.model.bottom());
        let bottom_right = self.model.bottom_right;
        let outline_properties = Line {
            depth: self.depth,
            color: self.color,
            texture_index: self.outline_texture_index,
            width: self.outline_width,
            ..Default::default()
        };

        // draw the top
        Line {
            start: top_left,
            end: top_right,
            ..outline_properties
        }
        .fill(frame)?;

        // draw the bottom
        Line {
            start: bottom_left,
            end: bottom_right,
            ..outline_properties
        }
        .fill(frame)?;

        // draw the left
        Line {
            start: top_left,
            end: bottom_left,
            ..outline_properties
        }
        .fill(frame)?;

        // draw the right
        Line {
            start: top_right,
            end: bottom_right,
            ..outline_properties
        }
        .fill(frame)?;

        Ok(())
    }
}
