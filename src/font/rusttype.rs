//! Implementation of the Font trait for rusttype fonts.
//!
//! There may be issues related to kerning, see layout implementation in <https://docs.rs/rusttype/latest/rusttype/enum.Font.html>

use core::fmt::Debug;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::PixelColor;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_ttf::{AntiAliasing, FontTextStyleBuilder};
use rusttype::{IntoGlyphId, Scale};

use crate::font::{self, CustomSize};
use crate::primitives::{Interpolate, Point};
use crate::primitives::{Size, geometry::Rectangle};
use crate::render_target::surface::AsDrawTarget;

use super::{Font, FontMetrics, FontRender};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustTypeAttributes {
    size: u32,
}

impl CustomSize for RustTypeAttributes {
    fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
}

impl Interpolate for RustTypeAttributes {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            size: u32::interpolate(from.size, to.size, amount),
        }
    }
}

impl Default for RustTypeAttributes {
    fn default() -> Self {
        Self { size: 12 }
    }
}

struct AttributedFont<'a> {
    font: &'a rusttype::Font<'a>,
    attributes: RustTypeAttributes,
}

impl Font for rusttype::Font<'_> {
    type Attributes = RustTypeAttributes;

    fn metrics(&self, attributes: &Self::Attributes) -> impl FontMetrics {
        // FIXME: Seems like there should be a way to do this without cloning?
        AttributedFont {
            font: self,
            attributes: attributes.clone(),
        }
    }
}

impl font::Sealed for rusttype::Font<'_> {}

impl<C> FontRender<C> for rusttype::Font<'static>
where
    C: PixelColor + Debug + Into<Rgb888> + From<Rgb888>,
{
    fn draw(
        &self,
        character: char,
        offset: Point,
        color: C,
        background_color: Option<C>,
        attributes: &Self::Attributes,
        surface: &mut impl crate::render_target::Surface<Color = C>,
    ) {
        let mut font = FontTextStyleBuilder::new(self.clone())
            .font_size(attributes.size)
            .text_color(color)
            .build();
        if let Some(background_color) = background_color {
            font.anti_aliasing = AntiAliasing::SolidColor(background_color);
        }

        let mut draw_target = surface.draw_target();

        let mut char_buf: [u8; 4] = [0; 4];
        let s: &str = character.encode_utf8(&mut char_buf);
        _ = font.draw_string(
            s,
            offset.into(),
            embedded_graphics::text::Baseline::Top,
            &mut draw_target,
        );
    }
}

impl FontMetrics for AttributedFont<'_> {
    #[expect(clippy::cast_precision_loss)]
    fn rendered_size(&self, character: char) -> Option<Rectangle> {
        // The origin of the rect returned by pixel_bounding_box is relative to the baseline
        let ascent = self
            .font
            .v_metrics(Scale::uniform(self.attributes.size as f32))
            .ascent;
        self.font
            .glyph(character.into_glyph_id(self.font))
            .scaled(rusttype::Scale::uniform(self.attributes.size as f32))
            .positioned(rusttype::Point { x: 0.0, y: ascent })
            .pixel_bounding_box()
            .map(|bb| {
                Rectangle::new(
                    Point::new(bb.min.x, bb.min.y),
                    Size::new((bb.max.x - bb.min.x) as u32, (bb.max.y - bb.min.y) as u32),
                )
            })
    }

    #[expect(clippy::cast_precision_loss)]
    fn vertical_metrics(&self) -> font::VMetrics {
        let metrics = self
            .font
            .v_metrics(Scale::uniform(self.attributes.size as f32));
        font::VMetrics {
            ascent: metrics.ascent as i32,
            descent: metrics.descent as i32,
            line_spacing: metrics.line_gap as i32,
        }
    }

    #[expect(clippy::cast_precision_loss)]
    fn advance(&self, character: char) -> u32 {
        self.font
            .glyph(character.into_glyph_id(self.font))
            .scaled(rusttype::Scale::uniform(self.attributes.size as f32))
            .h_metrics()
            .advance_width
            .round()
            .max(0.0) as u32
    }
}
