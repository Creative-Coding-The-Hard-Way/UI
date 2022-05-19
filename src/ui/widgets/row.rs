use ::anyhow::Result;

use crate::{
    builder_field,
    immediate_mode_graphics::Frame,
    ui::{
        primitives::Dimensions,
        widgets::{Element, Widget},
        Input, InternalState,
    },
    vec2, Vec2,
};

/// A Row is a collection of wigets which is arranged in a single horizontal
/// row.
pub struct Row<Message> {
    children: Vec<Element<Message>>,
    child_dimensions: Vec<Dimensions>,
    max_dimensions: Dimensions,
}

impl<Message> Row<Message> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            child_dimensions: vec![],
            max_dimensions: Dimensions::new(0.0, 0.0),
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

        self.child_dimensions.clear();
        self.child_dimensions.reserve(self.children.len());

        let mut remaining_size = *max_size;
        let mut bounds = Dimensions::new(0.0, 0.0);
        for child in &mut self.children {
            let child_bounds =
                child.dimensions(internal_state, &remaining_size);
            self.child_dimensions.push(child_bounds);

            bounds.width += child_bounds.width;
            bounds.height = bounds.height.max(child_bounds.height);

            remaining_size.width -= child_bounds.width;
        }

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
            let remaining_height =
                self.max_dimensions.height - dimensions.height;

            let child_position =
                desired_position + vec2(0.0, 0.5 * remaining_height);

            child.set_top_left_position(internal_state, child_position);

            desired_position.x += dimensions.width;
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