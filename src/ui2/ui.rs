use crate::{
    immediate_mode_graphics::Frame,
    ui2::{
        primitives::{Dimensions, Rect},
        ui_screen_space_projection,
        widgets::{Element, Widget},
        Input, InternalState,
    },
    vec2, Mat4,
};

use ::anyhow::Result;

pub trait UIState {
    type Message;

    /// Build the UI view based on the current state.
    fn view(&self) -> Element<Self::Message>;

    /// Update user state based on the given message.
    fn update(&mut self, message: &Self::Message);
}

/// The entrypoint for every UI. It manages internal state and knows how to
/// render the UI to a frame.
pub struct UI<C: UIState> {
    viewport: Rect,
    projection: Mat4,
    custom: C,
    current_view: Element<C::Message>,
    internal_state: InternalState,
    input: Input,
}

impl<C: UIState> UI<C> {
    /// Create a new UI instance with the given viewport width and height.
    pub fn new(viewport: Dimensions, custom_ui: C) -> Self {
        let mut ui = Self {
            viewport: Rect::new(0.0, 0.0, viewport.height, viewport.width),
            projection: ui_screen_space_projection(viewport),
            current_view: custom_ui.view(),
            custom: custom_ui,
            internal_state: InternalState::new(),
            input: Input::new(),
        };
        ui.layout();
        ui
    }

    /// Handle GLFW input events.
    /// Events are dispatched to the UI implementation automatically.
    pub fn handle_event(
        &mut self,
        event: &glfw::WindowEvent,
    ) -> Result<Option<C::Message>> {
        use glfw::WindowEvent;

        self.input.handle_event(event);
        match *event {
            WindowEvent::FramebufferSize(width, height) => {
                self.viewport =
                    Rect::new(0.0, 0.0, height as f32, width as f32);
                self.projection =
                    ui_screen_space_projection((width, height).into());
            }
            _ => (),
        }

        let message_opt = self.current_view.handle_event(
            &mut self.internal_state,
            &self.input,
            event,
        )?;

        if let Some(message) = &message_opt {
            self.custom.update(message);
            self.flush();
        } else {
            self.layout();
        }

        Ok(message_opt)
    }

    /// For the UI view to be regenerated and update the layout.
    /// This happens automatically after every update.
    pub fn flush(&mut self) {
        self.current_view = self.custom.view();
        self.layout();
    }

    /// Layout the Widgets into a single UI.
    fn layout(&mut self) {
        let _root_widget_dimensions = self
            .current_view
            .dimensions(&mut self.internal_state, &self.viewport.dimensions());
        self.current_view
            .set_top_left_position(&mut self.internal_state, vec2(0.0, 0.0));
    }

    /// Render the UI to the frame.
    ///
    /// # NOTE
    ///
    /// The UI is assumed to have complete control over the frame. As such, it
    /// sets the Frame's view projection. E.g. you'll get weird results if you
    /// try to change the projection after rendering or if you otherwise try
    /// to render to this frame.
    ///
    pub fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        frame.set_view_projection(self.projection)?;
        self.current_view
            .draw_frame(&mut self.internal_state, frame)?;
        Ok(())
    }
}
