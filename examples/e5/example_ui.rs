use ccthw::{
    gen_id2,
    ui2::{
        widgets::{Align, Button, Element},
        UIState,
    },
    vec4,
};

pub struct ExampleUi {}

impl ExampleUi {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExampleMessage {
    ToggleFullscreen,
}

impl UIState for ExampleUi {
    type Message = ExampleMessage;

    fn view(&self) -> Element<Self::Message> {
        let toggle_fullscreen_button = Button::new(gen_id2!())
            .color(vec4(0.1, 0.1, 0.1, 1.0))
            .hover_color(vec4(0.3, 0.3, 0.3, 1.0))
            .pressed_color(vec4(0.5, 0.5, 0.5, 1.0))
            .dimensions((100.0, 50.0))
            .on_click(ExampleMessage::ToggleFullscreen);

        Align::new(toggle_fullscreen_button).into()
    }

    fn update(&mut self, message: &ExampleMessage) {
        log::info!("Clicked! {:?}", message);
    }
}
