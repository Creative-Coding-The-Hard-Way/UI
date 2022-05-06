use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::{Drawable, Frame},
    ui::{
        primitives::{Rect, Tile},
        Bounds,
    },
    vec2, vec4, Vec2,
};

#[derive(Debug, Copy, Clone)]
pub struct Button {
    pub dimensions: Rect,
    pub shadow_offset: Vec2,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            dimensions: Rect::centered_at(0.0, 0.0, 64.0, 64.0),
            shadow_offset: vec2(8.0, 8.0),
        }
    }
}

impl Bounds for Button {
    fn bounds(&self) -> Rect {
        self.dimensions
    }
}

impl Button {
    pub fn draw_active(&self, frame: &mut Frame) -> Result<()> {
        self.draw_shadow(frame)?;
        Tile {
            model: self.dimensions.translate(0.5 * self.shadow_offset),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        }
        .fill(frame)
    }

    pub fn draw_focused(&self, frame: &mut Frame) -> Result<()> {
        self.draw_shadow(frame)?;
        Tile {
            model: self.dimensions,
            color: vec4(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        }
        .fill(frame)
    }

    pub fn draw_unfocused(&self, frame: &mut Frame) -> Result<()> {
        self.draw_shadow(frame)?;
        Tile {
            model: self.dimensions,
            color: vec4(0.5, 0.5, 0.5, 1.0),
            ..Default::default()
        }
        .fill(frame)
    }

    fn draw_shadow(&self, frame: &mut Frame) -> Result<()> {
        Tile {
            model: self.dimensions.translate(self.shadow_offset),
            color: vec4(0.0, 0.0, 0.0, 0.3),
            ..Default::default()
        }
        .fill(frame)
    }
}
