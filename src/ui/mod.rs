///! # UI Screen Space - The Coordinate System
///!
///! All coordinate values in this module assume a coordinate system where
///! (0,0) is the top left corner of the screen and (width,height) is the
///! bottom right corner of the screen.
///!
///! E.g. positive X points to the right, and positive Y points down.
///!
pub mod primitives;
pub mod text;

mod bounds;
mod button;
mod id;
mod state;

pub use self::{
    bounds::Bounds,
    button::Button,
    id::{id_hash, Id},
    state::{MouseState, State},
};
