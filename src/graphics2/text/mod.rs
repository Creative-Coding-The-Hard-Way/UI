mod text_error;

use std::collections::HashMap;

use ab_glyph::OutlinedGlyph;
use ::{
    ab_glyph::{Font, FontArc, PxScaleFont, ScaleFont},
    anyhow::Result,
    std::{fs::File, io::Read, path::Path},
};

use crate::{
    asset_loader::MipmapData,
    graphics2::{Frame, Vec2, Vec3, Vec4, Vertex},
};

#[derive(Copy, Clone, Debug)]
pub struct GlyphTexCoords {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Copy, Clone, Debug)]
struct Quad {
    tex_coords: GlyphTexCoords,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

pub struct Text {
    pub font: PxScaleFont<FontArc>,
    pub rasterized: MipmapData,
    pub tex_coords:
        std::collections::HashMap<ab_glyph::GlyphId, GlyphTexCoords>,
}

pub struct AtlasGlyph {
    outline: OutlinedGlyph,
    x: u32,
    y: u32,
}

impl Quad {
    pub fn draw(&self, tex_index: i32, frame: &mut Frame) -> Result<()> {
        let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
        frame.push_vertices(
            &[
                Vertex::new(
                    Vec3::new(self.left, self.top, 0.0),
                    white,
                    Vec2::new(self.tex_coords.left, self.tex_coords.top),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.right, self.top, 0.0),
                    white,
                    Vec2::new(self.tex_coords.right, self.tex_coords.top),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.right, self.bottom, 0.0),
                    white,
                    Vec2::new(self.tex_coords.right, self.tex_coords.bottom),
                    tex_index,
                ),
                Vertex::new(
                    Vec3::new(self.left, self.bottom, 0.0),
                    white,
                    Vec2::new(self.tex_coords.left, self.tex_coords.bottom),
                    tex_index,
                ),
            ],
            &[
                0, 1, 2, // top triangle
                2, 3, 0, // bottom triangle
            ],
        )
    }
}

impl Text {
    /// Create a Text object from a font in the given file with the given
    /// scale in pixels.
    pub fn from_font_file(path: impl AsRef<Path>, scale: f32) -> Result<Self> {
        let bytes = {
            let mut buffer = vec![];
            File::open(path)?.read_to_end(&mut buffer)?;
            buffer
        };
        let font = FontArc::try_from_vec(bytes)?;
        Self::from_font(font.into_scaled(scale))
    }

    /// Create a Text object from the given pixel-scaled font.
    pub fn from_font(font: PxScaleFont<FontArc>) -> Result<Self> {
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

        let mut tex_coords = std::collections::HashMap::new();
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

            tex_coords.insert(
                glyph_id,
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
            font,
            rasterized,
            tex_coords,
        })
    }

    pub fn layout_text<T>(
        &self,
        pos: Vec2,
        content: &T,
    ) -> Result<Vec<ab_glyph::Glyph>>
    where
        T: AsRef<str>,
    {
        let mut glyphs = vec![];
        let v_advance = self.font.line_gap() + self.font.height();
        let mut cursor = ab_glyph::point(pos.x, pos.y);

        let mut previous_glyph: Option<ab_glyph::Glyph> = None;
        for char in content.as_ref().chars() {
            let mut glyph = self.font.scaled_glyph(char);
            glyph.position = cursor;
            cursor.x += self.font.h_advance(glyph.id);
            if char.is_control() {
                if char == '\n' {
                    cursor.x = pos.x;
                    cursor.y -= v_advance;
                }
                previous_glyph = None;
                continue;
            }

            if let Some(previous) = previous_glyph.take() {
                glyph.position.x += self.font.kern(previous.id, glyph.id);
            }
            previous_glyph = Some(glyph.clone());

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
        let glyphs = self.layout_text(pos, contents)?;

        for glyph in glyphs {
            let id = glyph.id;
            if let Some(outline) = self.font.outline_glyph(glyph) {
                if let Some(tex_coords) = self.tex_coords.get(&id) {
                    let bounds = outline.px_bounds();
                    let quad = Quad {
                        tex_coords: *tex_coords,
                        top: bounds.max.y + bounds.height(),
                        bottom: bounds.max.y,
                        left: bounds.min.x,
                        right: bounds.max.x,
                    };
                    quad.draw(1, frame)?;
                }
            }
        }

        Ok(())
    }
}
