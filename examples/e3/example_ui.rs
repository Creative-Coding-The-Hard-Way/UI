use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        gen_id,
        ui::{
            widgets::{Align, Button, Element, Label, Panel, WithPadding},
            Font, UIState,
        },
        vec4,
    },
};

pub struct ExampleUi {
    em: f32,
    panel_texture: i32,
    font: Font,
}

impl ExampleUi {
    pub fn new(asset_loader: &mut AssetLoader) -> Result<Self> {
        let em = 32.0;
        let font = Font::from_font_file(
            "assets/Roboto-Regular.ttf",
            1.0 * em,
            asset_loader,
        )?;
        let panel_texture = asset_loader.read_texture("assets/Panel.png")?;
        Ok(Self {
            em,
            panel_texture,
            font,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExampleMessage {
    ToggleFullscreen,
}

impl UIState for ExampleUi {
    type Message = ExampleMessage;

    fn view(&self) -> Element<Self::Message> {
        let _toggle_fullscreen_button = Button::new(gen_id!())
            .color(vec4(0.1, 0.1, 0.1, 1.0))
            .hover_color(vec4(0.3, 0.3, 0.3, 1.0))
            .pressed_color(vec4(0.5, 0.5, 0.5, 1.0))
            .dimensions((100.0, 50.0))
            .on_click(ExampleMessage::ToggleFullscreen);

        let label =
            Label::new(&self.font, "Hello World:-").with_padding(self.em * 0.5);

        Align::new(Panel::new(label).texture_index(self.panel_texture))
            .vertical_alignment(ccthw::ui::widgets::VAlignment::Center)
            .horizontal_alignment(ccthw::ui::widgets::HAlignment::Center)
            .into()
    }

    fn update(&mut self, message: &ExampleMessage) {
        log::info!("Clicked! {:?}", message);
    }
}