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

/// A Col is a collection of wigets which is arranged in a single horizontal
/// row.
pub struct Col<Message> {
    children: Vec<Element<Message>>,
    child_dimensions: Vec<Dimensions>,
    max_dimensions: Dimensions,
    space_between: f32,
}

impl<Message> Col<Message> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            child_dimensions: vec![],
            max_dimensions: Dimensions::new(0.0, 0.0),
            space_between: 0.0,
        }
    }

    builder_field!(children, Vec<Element<Message>>);
    builder_field!(space_between, f32);

    /// Add a child element to the end of the row.
    pub fn child<W>(mut self, child: W) -> Self
    where
        W: Into<Element<Message>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<Message> Widget<Message> for Col<Message> {
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

        self.child_dimensions.clear();
        self.child_dimensions.reserve(self.children.len());

        let mut remaining_size = *max_size;
        let mut bounds = Dimensions::new(0.0, 0.0);
        for child in &mut self.children {
            let child_bounds =
                child.dimensions(internal_state, &remaining_size);
            self.child_dimensions.push(child_bounds);

            bounds.height += child_bounds.height;
            bounds.width = bounds.width.max(child_bounds.width);

            remaining_size.height -= child_bounds.height + self.space_between;
        }
        bounds.height +=
            self.space_between * (self.children.len() - 1).max(0) as f32;

        self.max_dimensions = bounds.min(max_size);
        self.max_dimensions
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let mut desired_position = position;
        for (child, dimensions) in
            self.children.iter_mut().zip(self.child_dimensions.iter())
        {
            child.set_top_left_position(internal_state, desired_position);
            desired_position.y += dimensions.height + self.space_between;
        }
    }
}

impl<Message> Into<Element<Message>> for Col<Message>
where
    Message: 'static,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
