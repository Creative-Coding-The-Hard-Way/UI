///! # Coordinate System
///!
///! All coordinate values in this module assume a coordinate system where
///! (0,0) is the top left corner of the screen and (width,height) is the
///! bottom right corner of the screen.
///!
///! E.g. positive X points to the right, and positive Y points down.
///!
pub mod primitives;
pub mod text;

mod state;

pub use self::state::{id_hash, Id, MouseState, State};
