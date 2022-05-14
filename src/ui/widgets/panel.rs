use ::anyhow::Result;

use crate::{
    builder_field,
    immediate_mode_graphics::{Drawable, Frame},
    ui::{
        primitives::{Dimensions, Rect, Tile},
        widgets::{Element, Widget},
        Input, InternalState,
    },
    vec4, Vec2, Vec4,
};

// A Panel is a colored rectangle, often used as a background for some other
// element.
pub struct Panel<Message> {
    child: Element<Message>,
    background: Rect,
    color: Vec4,
    texture_index: i32,
    grow: bool,
}

impl<Message> Panel<Message> {
    /// Wrap another widget in a new panel.
    pub fn new(widget: impl Into<Element<Message>>) -> Self {
        Self {
            child: widget.into(),
            background: Rect::new(0.0, 0.0, 0.0, 0.0),
            color: vec4(0.8, 0.8, 1.0, 0.1),
            texture_index: 0,
            grow: false,
        }
    }

    builder_field!(grow, bool);
    builder_field!(color, Vec4);
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
        if !self.grow {
            self.background = child_dimensions.as_rect();
        } else {
            self.background = max_size.as_rect();
        }
        self.background.dimensions()
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let offset = position - self.background.top_left;
        self.background = self.background.translate(offset);
        self.child.set_top_left_position(internal_state, position);
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
