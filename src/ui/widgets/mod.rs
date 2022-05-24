mod align;
mod button;
mod col;
mod composite;
mod container;
mod element;
mod hsplit;
mod label;
mod row;
mod slider;
mod window;

pub mod prelude;

use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::triangles::Frame,
    ui::{primitives::Dimensions, Input, InternalState},
    Vec2,
};

pub use self::{
    align::{Align, HAlignment, VAlignment},
    button::Button,
    col::Col,
    composite::{ComposedElement, ComposedMessage, Composite, CompositeWidget},
    container::{Constraint, Container, WithContainer},
    element::Element,
    hsplit::HSplit,
    label::Label,
    row::Row,
    slider::Slider,
    window::Window,
};

/// Widgets are UI building blocks. Widgets ar responsible for handling system
/// events and transforming them into an instance of their own Message when
/// the relevant sequence of events occurs.
///
/// # Layout
///
/// Widgets have two methods which are used for laying out the UI. First,
/// the parent widget is responsible for calling [`dimensions`] on each of its
/// children. The parent is then allowed to call [`set_top_left_position`] to
/// position each child on the screen.
///
/// As such, the mental model is:
///   1. widgets control their own dimensions
///   2. widgets do not control their own position
///   3. parent widgets control the position of child widgets
///
/// The root widget is always positioned at (0, 0), the top left of the screen.
///
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
