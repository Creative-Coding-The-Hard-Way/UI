use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        ui::{widgets::prelude::*, UIState},
        vec4,
    },
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExampleMessage {
    ToggleFullscreen,
    AngleSlider(f32),
}

pub struct ExampleUi {
    em: f32,
    font: Font,
    is_fullscreen: bool,
    pub angle: f32,
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
            angle: 0.0,
        })
    }
}

impl UIState for ExampleUi {
    type Message = ExampleMessage;

    fn view(&self) -> Element<Self::Message> {
        let message = if self.is_fullscreen {
            "Windowed"
        } else {
            "Fullscreen"
        };

        let fullscreen_button = text_button(&self.font, &message)
            .on_click(ExampleMessage::ToggleFullscreen)
            .color(vec4(1.0, 1.0, 1.0, 0.0))
            .hover_color(vec4(1.0, 1.0, 1.0, 0.1))
            .pressed_color(vec4(1.0, 1.0, 1.0, 0.5))
            .container()
            .border(1.0, vec4(0.0, 0.0, 0.0, 0.75), 0)
            .padding(0.5 * self.em);

        let angle_slider = slider(gen_id!(), 0.0, 2.0 * std::f32::consts::PI)
            .on_change(ExampleMessage::AngleSlider)
            .value(self.angle);

        align(
            col()
                .child(fullscreen_button, Justify::Center)
                .child(label(&self.font, "Sprite Angle"), Justify::Center)
                .child(angle_slider, Justify::Center)
                .space_between(SpaceBetween::Fixed(self.em))
                .container()
                .max_width(Constraint::FixedMaxSize(10.0 * self.em)),
        )
        .alignment(HAlignment::Right, VAlignment::Top)
        .into()
    }

    fn update(&mut self, message: &ExampleMessage) {
        match *message {
            ExampleMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
            }
            ExampleMessage::AngleSlider(angle) => {
                self.angle = angle;
            }
        }
    }
}
