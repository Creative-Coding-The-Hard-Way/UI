///! # UI Screen Space - The Coordinate System
///!
///! All coordinate values in this module assume a coordinate system where
///! (0,0) is the top left corner of the screen and (width,height) is the
///! bottom right corner of the screen.
///!
///! E.g. positive X points to the right, and positive Y points down.
///!
use crate::{math, Mat4};

pub mod primitives;
pub mod widgets;

mod bounds;
mod dimensions;
mod id;
mod input;
mod internal_state;
mod ui;

pub use self::{
    bounds::Bounds,
    dimensions::Dimensions,
    id::{id_hash, Id},
    input::Input,
    internal_state::InternalState,
    ui::{UIState, UI},
};

/// Create a new projection matrix which defines the UI Screen Space based
/// on the given width and height.
pub fn ui_screen_space_projection(viewport: Dimensions) -> Mat4 {
    math::projections::ortho(
        0.0,
        viewport.width,
        viewport.height,
        0.0,
        0.0,
        1.0,
    )
}
