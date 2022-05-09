use ::anyhow::Result;

use crate::{
    builder_field,
    immediate_mode_graphics::{Drawable, Frame},
    ui::{
        primitives::{Dimensions, Rect, Tile},
        widgets::{Element, Widget},
        Input, InternalState,
    },
    vec2, vec4, Vec2, Vec4,
};

/// A panel can be used as a background for a wrapped element. It's also a
/// convenient way to add additional padding to an element which may otherwise
/// not include it.
///
/// tl;dr - it's a bit like a <div>
pub struct Panel<Message> {
    child: Element<Message>,
    padding: f32,
    background: Rect,
    color: Vec4,
    texture_index: i32,
}

impl<Message> Panel<Message> {
    /// Wrap another widget in a new panel.
    pub fn new(widget: impl Into<Element<Message>>) -> Self {
        Self {
            child: widget.into(),
            padding: 0.0,
            background: Rect::new(0.0, 0.0, 0.0, 0.0),
            color: vec4(0.8, 0.8, 1.0, 0.1),
            texture_index: 0,
        }
    }

    builder_field!(color, Vec4);
    builder_field!(padding, f32);
    builder_field!(texture_index, i32);
}

impl<Message> Widget<Message> for Panel<Message> {
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        self.child.handle_event(internal_state, input, event)
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        Tile {
            model: self.background,
            color: self.color,
            texture_index: self.texture_index,
            ..Default::default()
        }
        .fill(frame)?;
        self.child.draw_frame(internal_state, frame)
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        let child_dimensions = self.child.dimensions(internal_state, max_size);
        self.background = Rect::new(
            0.0,
            0.0,
            child_dimensions.height,
            child_dimensions.width,
        );
        let with_padding = Dimensions::new(
            child_dimensions.width + self.padding * 2.0,
            child_dimensions.height + self.padding * 2.0,
        );
        with_padding.min(max_size)
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let current_position = self.background.top_left;
        let desired_position = position + vec2(self.padding, self.padding);
        let offset = desired_position - current_position;

        self.background = self.background.translate(offset);

        self.child
            .set_top_left_position(internal_state, desired_position);
    }
}

impl<Message> Into<Element<Message>> for Panel<Message>
where
    Message: 'static,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
