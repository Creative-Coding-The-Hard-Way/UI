use crate::{
    immediate_mode_graphics::{Drawable, Frame},
    vec2, vec3, vec4, Vec2, Vec4,
};
use anyhow::Result;

/// A Line is a single visual line connecting a start point and and end point.
pub struct Line {
    /// Where the line begins in world space.
    /// Defaults to the origin, (0.0, 0.0).
    pub start: Vec2,

    /// Where the line ends in world space.
    /// Defaults to one unit away from the origin, (1.0, 0.0).
    pub end: Vec2,

    /// The world-space depth for the line. Defaults to 0.0.
    pub depth: f32,

    /// The line's color, defaults to white.
    pub color: Vec4,

    /// The line's world-space width. Defaults to 1.0
    pub width: f32,

    /// The texture index applied to the line. Defaults to 0.
    pub texture_index: i32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: vec2(0.0, 0.0),
            end: vec2(1.0, 0.0),
            depth: 0.0,
            color: vec4(1.0, 1.0, 1.0, 1.0),
            width: 1.0,
            texture_index: 0,
        }
    }
}

impl Drawable for Line {
    /// 'Fill in' the line. This is identical to a call to outline.
    fn fill(&self, frame: &mut Frame) -> Result<()> {
        self.outline(frame)
    }

    /// Render the line by building a quad centered on the start and end points
    /// using the line's given width.
    fn outline(&self, frame: &mut Frame) -> Result<()> {
        use crate::immediate_mode_graphics::Vertex;

        let direction = self.end - self.start;

        // rotate the line's direction 90 degrees to the left, then normalize
        let normal = vec2(-direction.y, direction.x).normalize();
        let half_width = 0.5 * self.width * normal;

        let top_left = self.start + half_width;
        let bottom_left = self.start - half_width;
        let top_right = self.end + half_width;
        let bottom_right = self.end - half_width;

        let (uv_left, uv_right) = (0.0, 1.0);
        let (uv_top, uv_bottom) = (0.0, 1.0);

        frame.push_vertices(
            &[
                Vertex::new(
                    vec3(top_left.x, top_left.y, self.depth),
                    self.color,
                    vec2(uv_left, uv_top),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(top_right.x, top_right.y, self.depth),
                    self.color,
                    vec2(uv_right, uv_top),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(bottom_right.x, bottom_right.y, self.depth),
                    self.color,
                    vec2(uv_right, uv_bottom),
                    self.texture_index,
                ),
                Vertex::new(
                    vec3(bottom_left.x, bottom_left.y, self.depth),
                    self.color,
                    vec2(uv_left, uv_bottom),
                    self.texture_index,
                ),
            ],
            &[
                0, 1, 2, // Triangle 1
                3, 0, 2, // Triangle 2
            ],
        )
    }
}
