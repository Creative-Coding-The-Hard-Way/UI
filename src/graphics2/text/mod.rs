mod text_error;

use std::collections::HashMap;

use ab_glyph::OutlinedGlyph;
use ::{
    ab_glyph::{Font, FontArc, ScaleFont},
    anyhow::Result,
    std::{fs::File, io::Read, path::Path},
};

use crate::asset_loader::MipmapData;

pub struct GlyphTexCoords {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub struct Text {
    pub font: FontArc,
    pub rasterized: MipmapData,
    pub tex_coords: std::collections::HashMap<char, GlyphTexCoords>,
}

pub struct AtlasGlyph {
    outline: OutlinedGlyph,
    x: u32,
    y: u32,
}

impl Text {
    pub fn from_font_file(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = {
            let mut buffer = vec![];
            File::open(path)?.read_to_end(&mut buffer)?;
            buffer
        };
        let scale = 128.0;
        let raw_font = FontArc::try_from_vec(bytes)?;
        let font = raw_font.as_scaled(scale);
        let v_advance = (font.line_gap() + font.height()) as u32;
        let max_width = 2024;
        let h_padding = 10;

        let mut h_offset: u32 = 0;
        let mut v_offset: u32 = v_advance;
        let mut max_height = v_offset;

        let mut glyphs = HashMap::new();
        for (_id, char) in font.codepoint_ids() {
            let glyph = font.scaled_glyph(char);
            let outline_opt = font.outline_glyph(glyph);
            if outline_opt.is_none() {
                continue;
            }
            let mut outline = outline_opt.unwrap();
            let bounds = outline.px_bounds();
            if (bounds.width().ceil() as u32 + h_offset + h_padding)
                >= max_width
            {
                h_offset = 0;
                v_offset += v_advance;
                max_height = v_offset.max(max_height);
            }

            let glyph = AtlasGlyph {
                outline,
                x: h_offset,
                y: v_offset,
            };
            glyphs.insert(char, glyph);

            h_offset += h_padding + bounds.width().ceil() as u32;
        }

        let mut rasterized = MipmapData::allocate(
            max_width,
            max_height,
            [0xFF, 0xFF, 0xFF, 0x00],
        );

        let mut tex_coords = std::collections::HashMap::new();

        for (text_char, glyph) in glyphs {
            let base_x = glyph.x;
            let base_y = glyph.y;
            let width = glyph.outline.px_bounds().width();
            let height = glyph.outline.px_bounds().height();

            tex_coords.insert(
                text_char,
                GlyphTexCoords {
                    top: (base_y as f32 / max_height as f32),
                    bottom: (base_y as f32 - height as f32) / max_height as f32,
                    left: base_x as f32 / max_width as f32,
                    right: (base_x as f32 + width as f32) / max_width as f32,
                },
            );

            glyph.outline.draw(|x, y, coverage| {
                rasterized.write_pixel(
                    base_x + x,
                    base_y - (y + 1),
                    [0xFF, 0xFF, 0xFF, (0xFF as f32 * coverage) as u8],
                );
            });
        }

        Ok(Self {
            font: raw_font,
            rasterized,
            tex_coords,
        })
    }
}

fn bounds(outline: &OutlinedGlyph) -> (u32, u32) {
    let bounds = outline.px_bounds();
    (bounds.width() as u32, bounds.height() as u32)
}
