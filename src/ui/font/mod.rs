mod layout;
mod rasterize;

use ::{
    ab_glyph::{Font as AbFont, FontArc, GlyphId, PxScaleFont, ScaleFont},
    anyhow::Result,
    std::{collections::HashMap, fs::File, io::Read, path::Path},
};

use crate::{
    asset_loader::AssetLoader,
    builder_field,
    ui::primitives::{Rect, Tile},
    vec4, Vec4,
};

/// This struct contains all of the information required to render rasterized
/// glyphs on screen.
#[derive(Debug, Clone)]
pub struct Font {
    /// The underlying TTF/OTF font is used to layout glyphs.
    font: PxScaleFont<FontArc>,

    /// When a font is constructed, all of the glyphs are rasterized into a
    /// single texture. This keeps track of which texture to use when
    /// generating [`Tiles`] for rendering.
    texture_index: i32,

    /// When a font is constructed, all of the glyphs are rasterized into a
    /// single texture. This map tracks the texture coordinates for each glyph
    /// supported by the current font.
    glyph_texture_coords: HashMap<GlyphId, Rect>,

    /// The color of the text when rendered.
    text_color: Vec4,
}

impl Font {
    builder_field!(text_color, Vec4);

    /// Create a new font instance by reading the .ttf or .otf font file at the
    /// specified path.
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
        Self::from_ab_glyph_font(font.into_scaled(scale), asset_loader)
    }

    /// Resize this font using the same underlying ttf/otf file.
    pub fn rescale(
        self,
        scale: f32,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let rescaled_font = self.font.with_scale(scale);
        let font = Self::from_ab_glyph_font(rescaled_font, asset_loader)?;
        Ok(Self {
            font: font.font,
            texture_index: font.texture_index,
            glyph_texture_coords: font.glyph_texture_coords,
            ..self
        })
    }

    /// Create a new font instance from the given [`ab_glyph`] font.
    pub fn from_ab_glyph_font(
        font: PxScaleFont<FontArc>,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let glyphs = Self::layout_chars(
            &font,
            10,   // px padding between glyphs
            2048, // max width
            font.codepoint_ids().map(|(_id, char)| char),
        );

        let (rasterized_glyphs, glyph_texture_coords) =
            Self::rasterize_glyphs(&font, &glyphs);

        let texture_index =
            asset_loader.create_texture_with_data(&[rasterized_glyphs])?;

        Ok(Self {
            font,
            texture_index,
            glyph_texture_coords,
            text_color: vec4(1.0, 1.0, 1.0, 1.0),
        })
    }

    /// Build renderable tiles for the glyphs in the provided string.
    ///
    /// # Returns
    ///
    /// A tuple containing two things:
    ///
    /// 1. The set of renderable Tiles which can be used to draw the text.
    /// 2. The bounding rect for all Tiles based on font metrics. This can be
    ///    used when laying out and positioning text on-screen to get a
    ///    consistent arrangement regardless of which glyphs are being
    ///    rendered.
    ///
    pub fn build_text_tiles<T>(&self, content: T) -> (Vec<Tile>, Rect)
    where
        T: AsRef<str>,
    {
        let glyphs = Self::layout_text(&self.font, content);
        let mut tiles = Vec::with_capacity(glyphs.len());
        let mut total_bounds: Option<Rect> = None;

        glyphs
            .into_iter()
            .filter_map(|glyph| {
                // only draw glyphs that have an outline
                self.font
                    .outline_glyph(glyph.clone())
                    .map(|outline| (glyph, outline))
            })
            .filter_map(|(glyph, outline)| {
                // only draw glyphs if their texture coords are available
                self.glyph_texture_coords
                    .get(&glyph.id)
                    .map(|tex_coords| (glyph, *tex_coords, outline))
            })
            .for_each(|(glyph, texture_coords, outline)| {
                // build a tile with the tex coords and outline
                let bounds = outline.px_bounds();
                let tile = Tile {
                    model: Rect::new(
                        bounds.min.y.round(),
                        bounds.min.x.round(),
                        bounds.max.y.round(),
                        bounds.max.x.round(),
                    ),
                    uv: texture_coords,
                    texture_index: self.texture_index,
                    color: self.text_color,
                    ..Default::default()
                };
                tiles.push(tile);

                // compute the updated total bounds rect
                let glyph_bounds: Rect = self.font.glyph_bounds(&glyph).into();
                if let Some(total) = total_bounds.take() {
                    total_bounds = Some(total.expand(glyph_bounds));
                } else {
                    total_bounds = Some(glyph_bounds);
                }
            });

        (tiles, total_bounds.unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0)))
    }

    /// Get the computed line height for text rendered with this font.
    pub fn line_height(&self) -> f32 {
        self.font.height()
    }
}

impl Into<Rect> for ab_glyph::Rect {
    fn into(self) -> Rect {
        Rect::new(self.min.y, self.min.x, self.max.y, self.max.x)
    }
}
