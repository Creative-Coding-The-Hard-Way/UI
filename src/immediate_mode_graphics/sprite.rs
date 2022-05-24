use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::{Vertex, VertexStream},
    vec2, vec3, vec4, Vec2,
};

/// A sprite is a sized quad which can be rendered on screen.
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    /// The sprite's width.
    pub width: f32,

    /// The sprite's height.
    pub height: f32,

    /// The sprite's world-space position.
    pub position: Vec2,

    /// The sprite's orientation relative to the positive X axis.
    pub angle_in_radians: f32,

    /// The world-space depth to render the sprite at.
    pub depth: f32,

    /// The sprite's texture. This is the index provided by the [`AssetLoader`]
    /// when reading a texture.
    pub texture_index: i32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            position: vec2(0.0, 0.0),
            angle_in_radians: 0.0,
            depth: 0.0,
            texture_index: 0,
        }
    }
}

impl Sprite {
    pub fn draw(&self, vertices: &mut impl VertexStream) -> Result<()> {
        let rotation_matrix = nalgebra::Rotation2::new(self.angle_in_radians);

        let hw = 0.5 * self.width;
        let hh = 0.5 * self.height;
        let top_left = self.position + rotation_matrix * vec2(-hw, hh);
        let top_right = self.position + rotation_matrix * vec2(hw, hh);
        let bottom_left = self.position + rotation_matrix * vec2(-hw, -hh);
        let bottom_right = self.position + rotation_matrix * vec2(hw, -hh);

        let uv_left = 0.0;
        let uv_right = 1.0;
        let uv_top = 0.0;
        let uv_bottom = 1.0;

        vertices.push_vertices(
            &[
                Vertex::new(
                    vec3(top_left.x, top_left.y, self.depth),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec2(uv_left, uv_top),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(top_right.x, top_right.y, self.depth),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec2(uv_right, uv_top),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(bottom_right.x, bottom_right.y, self.depth),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec2(uv_right, uv_bottom),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(bottom_left.x, bottom_left.y, self.depth),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec2(uv_left, uv_bottom),
                    self.texture_index,
                ),
            ],
            &[
                0, 1, 2, // first triangle
                0, 2, 3, // second triangle
            ],
        )
    }
}
