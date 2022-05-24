mod constraint;

use ::anyhow::Result;

use crate::{
    builder_field,
    immediate_mode_graphics::triangles::Frame,
    ui::{
        primitives::{Dimensions, Rect, Tile},
        widgets::{Element, Widget},
        Input, InternalState,
    },
    vec2, vec4, Vec2, Vec4,
};

pub use self::constraint::Constraint;

/// A generic container for another [`Widget`]. Containers have margin, padding,
/// and a border akin to the standard CSS box model.
pub struct Container<Message, Widget> {
    margin: Rect,
    padding: Rect,
    border: Option<Tile>,
    background: Tile,
    max_width: Constraint,
    max_height: Constraint,

    /// The Widget contained by this container
    pub child: Widget,

    _phantom_data: std::marker::PhantomData<Message>,
}

impl<Message, Widget> Container<Message, Widget> {
    /// Create a new Container to decorate another widget.
    pub fn new(widget: Widget) -> Self {
        Self {
            ///! Margin and Padding default to 0.0
            margin: Rect::new(0.0, 0.0, 0.0, 0.0),
            padding: Rect::new(0.0, 0.0, 0.0, 0.0),

            ///! Border defaults to disabled
            border: None,

            ///! The Background defaults to fully transparent.
            background: Tile {
                color: vec4(1.0, 1.0, 1.0, 0.0),
                ..Default::default()
            },

            max_width: Default::default(),
            max_height: Default::default(),

            child: widget,
            _phantom_data: Default::default(),
        }
    }

    builder_field!(max_width, Constraint);
    builder_field!(max_height, Constraint);

    /// Set the margin on all sides.
    pub fn margin(self, margin: f32) -> Self {
        Self {
            margin: Rect::new(margin, margin, margin, margin),
            ..self
        }
    }

    /// Set the padding on all sides.
    pub fn padding(self, padding: f32) -> Self {
        Self {
            padding: Rect::new(padding, padding, padding, padding),
            ..self
        }
    }

    /// Set all of the border properties.
    pub fn border(self, width: f32, color: Vec4, texture_index: i32) -> Self {
        Self {
            border: Some(Tile {
                outline_width: width,
                color,
                texture_index,
                ..Default::default()
            }),
            ..self
        }
    }

    /// Set all available background properties.
    pub fn background(self, color: Vec4, texture_index: i32) -> Self {
        Self {
            background: Tile {
                color,
                texture_index,
                ..self.background
            },
            ..self
        }
    }

    /// Get the border width. Defaults to 0.0 if no border is included.
    fn get_border_width(&self) -> f32 {
        self.border.map_or(0.0, |tile| tile.outline_width)
    }
}

impl<Message, Child> Widget<Message> for Container<Message, Child>
where
    Child: Widget<Message>,
{
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
        self.background.fill(frame)?;

        if let Some(border) = &self.border {
            border.outline(frame)?;
        }

        self.child.draw_frame(internal_state, frame)
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        let adjusted_max_size = Dimensions::new(
            self.max_width.apply(max_size.width),
            self.max_height.apply(max_size.height),
        );
        let border_width = self.get_border_width();
        let horizonal_inset = self.padding.left()
            + self.padding.right()
            + self.margin.left()
            + self.margin.right()
            + border_width * 2.0;
        let vertical_inset = self.padding.top()
            + self.padding.bottom()
            + self.margin.top()
            + self.margin.bottom()
            + border_width * 2.0;
        let max_child_dimensions = Dimensions::new(
            0f32.max(adjusted_max_size.width - horizonal_inset),
            0f32.max(adjusted_max_size.height - vertical_inset),
        );
        let child_dimensions =
            self.child.dimensions(internal_state, &max_child_dimensions);

        // Update the background size
        let background_dimensions = Dimensions::new(
            child_dimensions.width + self.margin.left() + self.margin.right(),
            child_dimensions.height + self.margin.top() + self.margin.bottom(),
        );
        self.background.model = background_dimensions.as_rect();

        // Update the border size
        if let Some(border) = &mut self.border {
            let border_dimensions = Dimensions::new(
                background_dimensions.width + border_width,
                background_dimensions.height + border_width,
            );
            border.model = border_dimensions.as_rect();
        }

        let total_dimensions = Dimensions::new(
            child_dimensions.width + horizonal_inset,
            child_dimensions.height + vertical_inset,
        );
        total_dimensions.min(&adjusted_max_size)
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let border_width = self.get_border_width();

        // Set the border position
        if let Some(border) = &mut self.border {
            let border_top_left = position
                + self.padding.top_left
                + vec2(0.5 * border_width, 0.5 * border_width);
            border.model = border.model.set_top_left_position(border_top_left);
        }

        // Set the background position
        let background_top_left =
            position + self.padding.top_left + vec2(border_width, border_width);
        self.background.model = self
            .background
            .model
            .set_top_left_position(background_top_left);

        // Set the child position
        let child_top_left = background_top_left + self.margin.top_left;
        self.child
            .set_top_left_position(internal_state, child_top_left);
    }
}

impl<Message, Child> Into<Element<Message>> for Container<Message, Child>
where
    Message: 'static,
    Child: 'static + Widget<Message>,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}

/// Define an extra associated method which automatically wraps any widget
/// with a container.
pub trait WithContainer<Message, W: Widget<Message>> {
    fn container(self) -> Container<Message, W>;
}

impl<Message, W: Widget<Message>> WithContainer<Message, W> for W {
    fn container(self) -> Container<Message, W> {
        Container::new(self)
    }
}
