use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        ui::{widgets::prelude::*, UIState},
        vec4,
    },
};

pub struct ExampleUi {
    em: f32,
    font: Font,
    is_fullscreen: bool,
    pub border_width: f32,
}

impl ExampleUi {
    pub fn new(
        content_scale: f32,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let em = 16.0 * content_scale;
        let font = Font::from_font_file(
            "assets/Roboto-Regular.ttf",
            1.0 * em,
            asset_loader,
        )?;
        Ok(Self {
            em,
            font,
            is_fullscreen: false,
            border_width: 1.0,
        })
    }

    pub fn text_button(
        &self,
        message: impl AsRef<str>,
        on_click: ExampleMessage,
    ) -> Element<ExampleMessage> {
        text_button(&self.font, &message)
            .on_click(on_click)
            .color(vec4(1.0, 1.0, 1.0, 0.0))
            .hover_color(vec4(1.0, 1.0, 1.0, 0.1))
            .pressed_color(vec4(1.0, 1.0, 1.0, 0.5))
            .container()
            .border(1.0, vec4(0.0, 0.0, 0.0, 0.75), 0)
            .into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExampleMessage {
    ToggleFullscreen,
    Increment,
    Decrement,
    ValueSlider(f32),
}

impl UIState for ExampleUi {
    type Message = ExampleMessage;

    fn view(&self) -> Element<Self::Message> {
        let em = self.em;

        let message = if self.is_fullscreen {
            "Windowed"
        } else {
            "Fullscreen"
        };

        let fullscreen_controls =
            self.text_button(message, ExampleMessage::ToggleFullscreen);

        let counter_controls = row()
            .child(
                self.text_button("-2", ExampleMessage::Decrement),
                Justify::Center,
            )
            .child(
                label(&self.font, format!("{}", self.border_width)),
                Justify::Center,
            )
            .child(
                self.text_button("+2", ExampleMessage::Increment),
                Justify::Center,
            )
            .space_between(SpaceBetween::EvenSpaceAround)
            .container()
            .max_width(Constraint::FixedMaxSize(7.0 * em));

        let window = Window::new(self.font.clone(), "window controls")
            .contents(
                col()
                    .child(fullscreen_controls, Justify::Center)
                    .child(counter_controls, Justify::Center)
                    .child(
                        row()
                            .child(
                                label(
                                    &self.font,
                                    format!("{}", self.border_width),
                                )
                                .container()
                                .padding(1.0 * em),
                                Justify::Center,
                            )
                            .child(
                                slider(gen_id!(), 0.0, 50.0)
                                    .value(self.border_width)
                                    .on_change(ExampleMessage::ValueSlider),
                                Justify::Center,
                            )
                            .space_between(SpaceBetween::EvenSpaceAround),
                        Justify::Center,
                    )
                    .space_between(SpaceBetween::Fixed(0.5 * em))
                    .container()
                    .margin(1.0 * em)
                    .background(vec4(0.0, 0.0, 0.0, 0.2), 0),
            )
            .container()
            .max_width(Constraint::PercentMaxSize(0.25))
            .background(vec4(0.0, 0.0, 0.3, 0.1), 0);

        align(window)
            .alignment(HAlignment::Right, VAlignment::Top)
            .into()
    }

    fn update(&mut self, message: &ExampleMessage) {
        match *message {
            ExampleMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
            }
            ExampleMessage::Increment => {
                self.border_width += 2.0;
            }
            ExampleMessage::Decrement => {
                self.border_width -= 2.0;
            }
            ExampleMessage::ValueSlider(value) => {
                self.border_width = value.round();
            }
        }
    }
}
