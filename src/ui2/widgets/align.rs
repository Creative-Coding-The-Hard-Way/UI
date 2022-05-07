use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::Frame,
    ui2::{
        widgets::{Element, Widget},
        Dimensions, Input, InternalState,
    },
    vec2, Vec2,
};

/// Represents the Horizontal Alignment for a [`Widget`] wrapped by [`Align`].
#[derive(Debug, Copy, Clone)]
pub enum HAlignment {
    Center,
    Left,
    Right,
}

/// Represents the Vertical Alignment for a [`Widget`] wrapped by [`Align`].
#[derive(Debug, Copy, Clone)]
pub enum VAlignment {
    Center,
    Bottom,
    Top,
}

/// A [`Widget`] which wraps a contiained Widget to automatically aligns it
/// within the available space.
#[derive(Debug, Copy, Clone)]
pub struct Align<Message, W: Widget<Message>> {
    halignment: HAlignment,
    valignment: VAlignment,
    child: W,
    child_offset: Vec2,
    _phantom_data: std::marker::PhantomData<Message>,
}

impl<Message, W> Into<Element<Message>> for Align<Message, W>
where
    Message: 'static,
    W: Widget<Message> + 'static,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}

impl<Message, W: Widget<Message>> Align<Message, W> {
    /// Align the provided widget.
    pub fn new(child: W) -> Self {
        Self {
            halignment: HAlignment::Center,
            valignment: VAlignment::Center,
            child,
            child_offset: vec2(0.0, 0.0),
            _phantom_data: Default::default(),
        }
    }

    pub fn with_h_alignment(self, halignment: HAlignment) -> Self {
        Self { halignment, ..self }
    }

    pub fn with_v_alignment(self, valignment: VAlignment) -> Self {
        Self { valignment, ..self }
    }
}

impl<Message, W: Widget<Message>> Widget<Message> for Align<Message, W> {
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
        self.child.draw_frame(internal_state, frame)
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        let child_dimensions = self.child.dimensions(internal_state, max_size);
        let remaining_width = max_size.width - child_dimensions.width;
        let remaining_height = max_size.height - child_dimensions.height;

        self.child_offset = vec2(
            match self.halignment {
                HAlignment::Left => 0.0,
                HAlignment::Center => 0.5 * remaining_width,
                HAlignment::Right => remaining_width,
            },
            match self.valignment {
                VAlignment::Top => 0.0,
                VAlignment::Center => 0.5 * remaining_height,
                VAlignment::Bottom => remaining_height,
            },
        );

        *max_size
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        self.child.set_top_left_position(
            internal_state,
            position + self.child_offset,
        );
    }
}
