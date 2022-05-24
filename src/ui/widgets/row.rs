use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::triangles::Frame,
    ui::{
        primitives::{DimensionList, Dimensions, Justify, SpaceBetween},
        widgets::{Element, Widget},
        Input, InternalState,
    },
    Vec2,
};

/// A Row is a collection of wigets which is arranged in a single horizontal
/// row.
pub struct Row<Message> {
    children: Vec<(Element<Message>, Justify)>,
    child_dimensions: DimensionList,
}

impl<Message> Row<Message> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            child_dimensions: DimensionList::horizontal(),
        }
    }

    pub fn space_between(self, space_between: SpaceBetween) -> Self {
        Self {
            child_dimensions: self
                .child_dimensions
                .space_between(space_between),
            ..self
        }
    }

    /// Add a child element to the end of the row.
    pub fn child<W>(mut self, child: W, justify: Justify) -> Self
    where
        W: Into<Element<Message>>,
    {
        self.children.push((child.into(), justify));
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
        for (child, _) in &mut self.children {
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
        for (child, _) in &self.children {
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

        self.child_dimensions.set_max_size(max_size);

        let mut remaining_size = *max_size;

        for (child, justify) in &mut self.children {
            let child_bounds =
                child.dimensions(internal_state, &remaining_size);

            remaining_size = self
                .child_dimensions
                .add_child_dimensions(child_bounds, *justify);
        }

        self.child_dimensions.dimensions()
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let positions = self.child_dimensions.compute_child_positions();
        for ((child, _), child_pos) in
            self.children.iter_mut().zip(positions.iter())
        {
            child.set_top_left_position(internal_state, position + child_pos);
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
