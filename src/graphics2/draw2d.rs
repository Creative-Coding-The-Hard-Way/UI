use ::anyhow::Result;

use crate::graphics2::{Frame, Vec2, Vec3, Vec4, Vertex};

#[derive(Debug, Copy, Clone)]
pub struct QuadArgs {
    pub center: Vec2,
    pub dimensions: Vec2,
    pub rgba: Vec4,
    pub texture_index: i32,
    pub depth: f32,
    pub angle: f32, // th
}

#[derive(Debug, Copy, Clone)]
pub struct LineArgs {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
    pub rgba: Vec4,
    pub texture_index: i32,
    pub depth: f32,
}

impl Default for QuadArgs {
    fn default() -> QuadArgs {
        QuadArgs {
            center: Vec2::zeros(),
            dimensions: Vec2::zeros(),
            rgba: Vec4::new(1.0, 1.0, 1.0, 1.0),
            texture_index: 0,
            depth: 0.0,
            angle: 0.0,
        }
    }
}

impl Default for LineArgs {
    fn default() -> LineArgs {
        LineArgs {
            start: Vec2::zeros(),
            end: Vec2::zeros(),
            width: 1.0,
            rgba: Vec4::new(1.0, 1.0, 1.0, 1.0),
            texture_index: 0,
            depth: 0.0,
        }
    }
}

/// Frame extension methods for drawing lines and quads.
pub trait Draw2D {
    /// Draw a single line with a start and end point.
    fn draw_line(&mut self, args: LineArgs) -> Result<()>;

    /// Draw a textured quad at a given location.
    fn draw_quad(&mut self, args: QuadArgs) -> Result<()>;
}

impl Draw2D for Frame {
    fn draw_line(&mut self, args: LineArgs) -> Result<()> {
        let line_direction = args.end - args.start;
        let normal = Vec2::new(-line_direction.y, line_direction.x).normalize();
        let half_sized = normal * args.width * 0.5;

        let top_left = args.start + half_sized;
        let bottom_left = args.start - half_sized;
        let top_right = args.end + half_sized;
        let bottom_right = args.end - half_sized;

        let (uv_left, uv_right) = (0.0, 1.0);
        let (uv_bottom, uv_top) = (0.0, 1.0);

        self.push_vertices(
            &[
                Vertex::new(
                    Vec3::new(top_left.x, top_left.y, args.depth),
                    args.rgba,
                    Vec2::new(uv_left, uv_top),
                    args.texture_index,
                ),
                Vertex::new(
                    Vec3::new(top_right.x, top_right.y, args.depth),
                    args.rgba,
                    Vec2::new(uv_right, uv_top),
                    args.texture_index,
                ),
                Vertex::new(
                    Vec3::new(bottom_right.x, bottom_right.y, args.depth),
                    args.rgba,
                    Vec2::new(uv_right, uv_bottom),
                    args.texture_index,
                ),
                Vertex::new(
                    Vec3::new(bottom_left.x, bottom_left.y, args.depth),
                    args.rgba,
                    Vec2::new(uv_left, uv_bottom),
                    args.texture_index,
                ),
            ],
            &[
                0, 1, 2, // t1
                3, 0, 2, // t2
            ],
        )?;

        Ok(())
    }

    fn draw_quad(&mut self, args: QuadArgs) -> Result<()> {
        let rotation = nalgebra::Rotation2::new(args.angle);

        let half_size = 0.5 * args.dimensions;
        let bottom_left = rotation * Vec2::new(-half_size.x, -half_size.y);
        let bottom_right = rotation * Vec2::new(half_size.x, -half_size.y);
        let top_right = rotation * Vec2::new(half_size.x, half_size.y);
        let top_left = rotation * Vec2::new(-half_size.x, half_size.y);

        let (uv_left, uv_right) = (0.0, 1.0);
        let (uv_bottom, uv_top) = (0.0, 1.0);

        self.push_vertices(
            &[
                Vertex::new(
                    new_vec3(args.center + bottom_left, args.depth),
                    args.rgba,
                    Vec2::new(uv_left, uv_bottom),
                    args.texture_index,
                ),
                Vertex::new(
                    new_vec3(args.center + top_left, args.depth),
                    args.rgba,
                    Vec2::new(uv_left, uv_top),
                    args.texture_index,
                ),
                Vertex::new(
                    new_vec3(args.center + top_right, args.depth),
                    args.rgba,
                    Vec2::new(uv_right, uv_top),
                    args.texture_index,
                ),
                Vertex::new(
                    new_vec3(args.center + bottom_right, args.depth),
                    args.rgba,
                    Vec2::new(uv_right, uv_bottom),
                    args.texture_index,
                ),
            ],
            &[
                0, 1, 2, // t1
                2, 3, 0, // t2
            ],
        )?;
        Ok(())
    }
}

fn new_vec3(vec: Vec2, depth: f32) -> Vec3 {
    Vec3::new(vec.x, vec.y, depth)
}
