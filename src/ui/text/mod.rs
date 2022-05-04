use ::{
    ab_glyph::{Font, FontArc, GlyphId, OutlinedGlyph, PxScaleFont, ScaleFont},
    anyhow::Result,
    std::{collections::HashMap, fs::File, io::Read, path::Path},
};

use crate::{
    asset_loader::{AssetLoader, MipmapData},
    immediate_mode_graphics::{Drawable, Frame},
    ui::primitives::{Rect, Tile},
    Vec2,
};

#[derive(Debug, Clone)]
pub struct AtlasGlyph {
    outline: OutlinedGlyph,
    x: u32,
    y: u32,
}

/// All of the resources required to rasterize a font and render text with
/// textured tiles.
#[derive(Debug, Clone)]
pub struct Text {
    font: PxScaleFont<FontArc>,
    texture_index: i32,
    glyph_texture_coords: HashMap<GlyphId, Rect>,
}

impl Text {
    /// Create a Text object from a font in the given file with the given
    /// scale in pixels.
    pub fn from_font_file(
        path: impl AsRef<Path>,
        scale: f32,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let bytes = {
            let mut buffer = vec![];
            File::open(path)?.read_to_end(&mut buffer)?;
            buffer
        };
        let font = FontArc::try_from_vec(bytes)?;
        Self::from_font(font.into_scaled(scale), asset_loader)
    }

    /// Create a Text object from the given pixel-scaled font.
    pub fn from_font(
        font: PxScaleFont<FontArc>,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let v_advance = (font.line_gap() + font.height()) as u32;
        let max_width = 2024;
        let h_padding = 10;

        let mut h_offset: u32 = 0;
        let mut v_offset: u32 = v_advance;
        let mut max_height = v_offset;

        let mut glyphs = HashMap::new();
        for (_id, char) in font.codepoint_ids() {
            let glyph = font.scaled_glyph(char);
            let id = glyph.id;
            let outline_opt = font.outline_glyph(glyph);
            if outline_opt.is_none() {
                continue;
            }
            let outline = outline_opt.unwrap();
            let bounds = outline.px_bounds();
            if (bounds.width().ceil() as u32 + h_offset + h_padding)
                >= max_width
            {
                h_offset = 0;
                v_offset += v_advance;
                max_height = v_offset.max(max_height);
            }

            let atlas_glyph = AtlasGlyph {
                outline,
                x: h_offset,
                y: v_offset,
            };
            glyphs.insert(id, atlas_glyph);

            h_offset += h_padding + bounds.width().ceil() as u32;
        }

        let mut glyph_texture_coords = HashMap::new();
        let mut rasterized = MipmapData::allocate(
            max_width,
            max_height,
            [0xFF, 0xFF, 0xFF, 0x00],
        );

        for (glyph_id, glyph) in glyphs {
            let base_x = glyph.x;
            let base_y = glyph.y;
            let width = glyph.outline.px_bounds().width();
            let height = glyph.outline.px_bounds().height();

            glyph_texture_coords.insert(
                glyph_id,
                Rect::new(
                    base_y as f32 / max_height as f32,
                    base_x as f32 / max_width as f32,
                    (base_y as f32 - height as f32) / max_height as f32,
                    (base_x as f32 + width as f32) / max_width as f32,
                ),
            );

            glyph.outline.draw(|x, y, coverage| {
                rasterized.write_pixel(
                    base_x + x,
                    base_y - (y + 1),
                    [0xFF, 0xFF, 0xFF, (0xFF as f32 * coverage) as u8],
                );
            });
        }

        let texture_index =
            asset_loader.create_texture_with_data(&[rasterized])?;

        Ok(Self {
            font,
            texture_index,
            glyph_texture_coords,
        })
    }

    pub fn layout_text<T>(&self, content: &T) -> Result<Vec<ab_glyph::Glyph>>
    where
        T: AsRef<str>,
    {
        let mut glyphs = vec![];
        let v_advance = (self.font.line_gap() + self.font.height()).ceil();
        let mut cursor = ab_glyph::point(0.0, 0.0);

        let mut previous_glyph: Option<ab_glyph::Glyph> = None;
        for char in content.as_ref().chars() {
            let mut glyph = self.font.scaled_glyph(char);
            glyph.position = cursor;
            cursor.x += self.font.h_advance(glyph.id);
            if char.is_control() {
                if char == '\n' {
                    cursor.x = 0.0;
                    cursor.y += v_advance;
                }
                previous_glyph = None;
                continue;
            }

            if let Some(previous) = previous_glyph.take() {
                let kern = self.font.kern(previous.id, glyph.id);
                if kern > 0.0 {
                    log::info!(
                        "Kern for '{:?}{:?}' is {}",
                        previous.id,
                        glyph.id,
                        kern
                    );
                }
                glyph.position.x += kern;
            }
            previous_glyph = Some(glyph.clone());

            // Round each time we loop, this helps to prevent an accumulation
            // of little errors that make the text look misaligned.
            cursor.x = cursor.x.round();
            cursor.y = cursor.y.round();

            glyphs.push(glyph);
        }

        Ok(glyphs)
    }

    pub fn draw_text<T>(
        &self,
        frame: &mut Frame,
        pos: Vec2,
        contents: &T,
    ) -> Result<()>
    where
        T: AsRef<str>,
    {
        let glyphs = self.layout_text(contents)?;
        for glyph in glyphs {
            let id = glyph.id;
            if let Some(outline) = self.font.outline_glyph(glyph) {
                if let Some(tex_coords) = self.glyph_texture_coords.get(&id) {
                    let bounds = outline.px_bounds();
                    Tile {
                        uv: *tex_coords,
                        model: Rect::new(
                            pos.y + bounds.min.y,
                            pos.x + bounds.min.x,
                            pos.y + bounds.max.y,
                            pos.x + bounds.max.x,
                        ),
                        texture_index: self.texture_index,
                        ..Default::default()
                    }
                    .fill(frame)?;
                }
            }
        }

        Ok(())
    }
}
