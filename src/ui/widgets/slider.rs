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

/// A slider can be used to control a continuously varying value.
pub struct Slider<Message> {
    on_change: FnOnce(f32) -> Message,
}

impl<Message> Widget<Message> for Slider<Message> {
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        todo!()
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        todo!()
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        todo!()
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        todo!()
    }
}
