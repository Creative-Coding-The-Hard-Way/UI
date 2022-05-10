use ::anyhow::Result;

use crate::ui::{
    primitives::{Dimensions, Padding, Rect},
    widgets::{Element, Widget},
    Input, InternalState,
};

/// The PaddedWidget can add padding to any other widget.
/// It's often more convenient to use the [`WithPadding`] trait so any
/// [`Widget`] impl automatically gets the 'with_padding` method.
pub struct PaddedWidget<Message, W: Widget<Message>> {
    widget: W,
    interior_region: Rect,
    padding: Padding,
    _phantom_data: std::marker::PhantomData<Message>,
}

impl<Message, W: Widget<Message>> PaddedWidget<Message, W> {
    /// Add Padding to a widget.
    pub fn new(widget: W, padding: f32) -> Self {
        Self {
            widget,
            interior_region: Rect::new(0.0, 0.0, 0.0, 0.0),
            padding: Padding::all(padding),
            _phantom_data: Default::default(),
        }
    }

    /// Borrow the internal widget.
    pub fn widget(&self) -> &W {
        &self.widget
    }

    /// Get a mutable borrow of the internal widget.
    pub fn widget_mut(&mut self) -> &mut W {
        &mut self.widget
    }

    pub fn left(self, left: f32) -> Self {
        Self {
            padding: self.padding.left(left),
            ..self
        }
    }

    pub fn right(self, right: f32) -> Self {
        Self {
            padding: self.padding.right(right),
            ..self
        }
    }

    pub fn bottom(self, bottom: f32) -> Self {
        Self {
            padding: self.padding.bottom(bottom),
            ..self
        }
    }

    pub fn top(self, top: f32) -> Self {
        Self {
            padding: self.padding.top(top),
            ..self
        }
    }
}

impl<Message, W: Widget<Message>> Widget<Message> for PaddedWidget<Message, W> {
    #[inline]
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        self.widget.handle_event(internal_state, input, event)
    }

    #[inline]
    fn draw_frame(
        &self,
        internal_state: &mut crate::ui::InternalState,
        frame: &mut crate::immediate_mode_graphics::Frame,
    ) -> anyhow::Result<()> {
        self.widget.draw_frame(internal_state, frame)
    }

    #[inline]
    fn dimensions(
        &mut self,
        internal_state: &mut crate::ui::InternalState,
        max_size: &crate::ui::primitives::Dimensions,
    ) -> crate::ui::primitives::Dimensions {
        let max_minus_padding = Dimensions::new(
            max_size.width - (self.padding.left + self.padding.right),
            max_size.height - (self.padding.top + self.padding.bottom),
        );
        let interior_dimensions =
            self.widget.dimensions(internal_state, &max_minus_padding);

        self.interior_region = interior_dimensions.as_rect();

        let padded_size = self.padding.apply(self.interior_region).dimensions();
        padded_size
    }

    #[inline]
    fn set_top_left_position(
        &mut self,
        internal_state: &mut crate::ui::InternalState,
        position: crate::Vec2,
    ) {
        let with_padding = self.padding.apply(self.interior_region);
        let offset = position - with_padding.top_left;

        self.interior_region = self.interior_region.translate(offset);

        self.widget.set_top_left_position(
            internal_state,
            self.interior_region.top_left,
        );
    }
}

impl<Message, W: Widget<Message>> Into<Element<Message>>
    for PaddedWidget<Message, W>
where
    Message: 'static,
    W: 'static,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}

/// Define an extra associated method which allows adding padding to any other
/// widget implementation.
pub trait WithPadding<Message, W: Widget<Message>> {
    fn with_padding(self, padding: f32) -> PaddedWidget<Message, W>;
}

impl<Message, W: Widget<Message>> WithPadding<Message, W> for W {
    fn with_padding(self, padding: f32) -> PaddedWidget<Message, W> {
        PaddedWidget::new(self, padding)
    }
}
