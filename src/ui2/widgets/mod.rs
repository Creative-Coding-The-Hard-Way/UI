mod align;
mod button;
mod element;

use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::Frame,
    ui2::{primitives::Dimensions, Input, InternalState},
    Vec2,
};

pub use self::{
    align::{Align, HAlignment, VAlignment},
    button::Button,
    element::Element,
};

pub trait Widget<Message> {
    /// Handle events for this widget.
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>>;

    /// Render this widget to the current frame.
    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()>;

    /// Compute the widget's dimensions.
    /// A widget must not allow its dimensions to exceed the provided max
    /// size.
    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions;

    /// Set this widget's top-left position in screen space.
    /// This is always called by the parent widget.
    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    );
}
