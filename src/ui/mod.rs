///! # Coordinate System
///!
///! All coordinate values in this module assume a coordinate system where
///! (0,0) is the top left corner of the screen and (width,height) is the
///! bottom right corner of the screen.
///!
///! E.g. positive X points to the right, and positive Y points down.
///!
use crate::{math::projections::ortho, Mat4};

pub mod primitives;
pub mod text;

mod state;

pub use self::state::{Id, MouseState, State};

/// Generate a view projection matrix which defines coordinate values where
/// (0,0) is the top left corner of the screen and (width,height) is the
/// bottom right corner of the screen.
///
/// E.g. positive X points to the right, and positive Y points down.
///
/// The z-axis ranges from 0.0 on the near plane and 1.0 on the far plane, but
/// most of the time depth-testing is disabled for UI rendering so this is
/// typically unimportant.
pub fn ui_projection(framebuffer_width: u32, framebuffer_height: u32) -> Mat4 {
    ortho(
        0.0,
        framebuffer_width as f32,
        framebuffer_height as f32,
        0.0,
        0.0,
        1.0,
    )
}
