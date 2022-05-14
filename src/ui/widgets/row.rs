use ::anyhow::Result;

use crate::{
    builder_field,
    immediate_mode_graphics::Frame,
    ui::{
        primitives::Dimensions,
        widgets::{Element, Widget},
        Input, InternalState,
    },
    Vec2,
};

/// A Row is a collection of wigets which is arranged in a single horizontal
/// row.
pub struct Row<Message> {
    children: Vec<Element<Message>>,
    widths: Vec<f32>,
}

impl<Message> Row<Message> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            widths: vec![],
        }
    }

    builder_field!(children, Vec<Element<Message>>);

    /// Add a child element to the end of the row.
    pub fn child<W>(mut self, child: W) -> Self
    where
        W: Into<Element<Message>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<Message> Widget<Message> for Row<Message> {
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        for child in &mut self.children {
            if let Some(message) =
                child.handle_event(internal_state, input, event)?
            {
                return Ok(Some(message));
            }
        }
        Ok(None)
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        for child in &self.children {
            child.draw_frame(internal_state, frame)?;
        }
        Ok(())
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        if self.children.is_empty() {
            return Dimensions::new(0.0, 0.0);
        }

        self.widths.clear();
        self.widths.reserve(self.children.len());

        let mut remaining_size = *max_size;
        let mut bounds = Dimensions::new(0.0, 0.0);
        for child in &mut self.children {
            let child_bounds =
                child.dimensions(internal_state, &remaining_size);
            self.widths.push(child_bounds.width);
            bounds.width += child_bounds.width;
            bounds.height = bounds.height.max(child_bounds.height);
            remaining_size.width -= child_bounds.width;
        }
        bounds.min(max_size)
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let mut desired_position = position;
        for (child, width) in self.children.iter_mut().zip(self.widths.iter()) {
            child.set_top_left_position(internal_state, desired_position);
            desired_position.x += width;
        }
    }
}

impl<Message> Into<Element<Message>> for Row<Message>
where
    Message: 'static,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
