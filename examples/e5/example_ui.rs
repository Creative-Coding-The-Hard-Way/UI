use ccthw::{
    gen_id2,
    ui2::{
        widgets::{Align, Container, Element, HAlignment, VAlignment},
        UIState,
    },
    vec4,
};

pub struct ExampleUi {
    r: f32,
}

impl ExampleUi {
    pub fn new() -> Self {
        Self { r: 0.0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExampleUiMessages {
    MyButtonClicked,
}

impl UIState for ExampleUi {
    type Message = ExampleUiMessages;

    fn view(&self) -> Element<Self::Message> {
        Align::new(
            Container::new(gen_id2!(), 512.0, 256.0)
                .with_color(vec4(self.r, 0.1, 0.1, 1.0))
                .with_should_grow(false)
                .with_on_click(ExampleUiMessages::MyButtonClicked),
        )
        .with_h_alignment(HAlignment::Left)
        .with_v_alignment(VAlignment::Bottom)
        .into()
    }

    fn update(&mut self, message: &ExampleUiMessages) {
        self.r += 1.0;
        log::info!("Clicked! {:?} {:?}", message, self.r);
    }
}
