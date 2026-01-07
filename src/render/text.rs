use heapless::Vec;

use crate::{
    font::{Font, FontMetrics, FontRender},
    primitives::{Interpolate, Point, Size, geometry::Rectangle},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{Glyph, RenderTarget, SolidBrush},
    view::{CharacterWrap, HorizontalTextAlignment, WordWrap, WrapStrategy},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(Rust, packed)]
pub struct Line {
    pub range: (u16, u16),
    pub pixel_width: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a, T, F: Font, const LINES: usize> {
    pub origin: (i16, i16),
    pub size: (u16, u16),
    pub font: &'a F,
    pub text: T,
    pub attributes: F::Attributes,
    pub alignment: HorizontalTextAlignment,
    pub lines: Vec<Line, LINES, u8>,
    pub max_lines: u16,
    pub wrap: WrapStrategy,
}

impl<const N: usize, T, F: Font> Text<'_, T, F, N> {
    const _ASSERT_LINES_FIT_IN_U8: () = assert!(N <= u8::MAX as usize);
}

// FIXME: Remove and just use struct init
impl<'a, T: AsRef<str>, F: Font> Text<'a, T, F, 8> {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        origin: (i16, i16),
        size: (u16, u16),
        font: &'a F,
        text: T,
        attributes: F::Attributes,
        alignment: HorizontalTextAlignment,
        lines: Vec<Line, 8, u8>,
        max_lines: u16,
        wrap: WrapStrategy,
    ) -> Self {
        Self {
            origin,
            size,
            font,
            text,
            attributes,
            alignment,
            lines,
            max_lines,
            wrap,
        }
    }
}

impl<T: Clone, F: Font, const N: usize> Clone for Text<'_, T, F, N> {
    fn clone(&self) -> Self {
        () = Self::_ASSERT_LINES_FIT_IN_U8;

        Self {
            origin: self.origin,
            size: self.size,
            font: self.font,
            text: self.text.clone(),
            attributes: self.attributes.clone(),
            alignment: self.alignment,
            lines: self.lines.clone(),
            max_lines: self.max_lines,
            wrap: self.wrap,
        }
    }
}

impl<T: AsRef<str>, F: Font, const N: usize> AnimatedJoin for Text<'_, T, F, N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        () = Self::_ASSERT_LINES_FIT_IN_U8;

        // Text content (and line breaks) jump
        self.origin = Interpolate::interpolate(source.origin, self.origin, domain.factor);
        self.size = Interpolate::interpolate(source.size, self.size, domain.factor);
        self.attributes = Interpolate::interpolate(
            source.attributes.clone(),
            self.attributes.clone(),
            domain.factor,
        );
    }
}

impl<C: Copy, T: AsRef<str> + Clone, F: FontRender<C>, const LINE_BREAKS: usize> Render<C>
    for Text<'_, T, F, LINE_BREAKS>
{
    #[expect(clippy::too_many_lines)]
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let clip_rect = render_target.clip_rect();
        let origin = Point::new(self.origin.0 as i32, self.origin.1 as i32);
        let size = Size::new(self.size.0 as u32, self.size.1 as u32);
        let bounding_box = Rectangle::new(origin, size);
        if size.width == 0 || size.height == 0 || !bounding_box.intersects(&clip_rect) {
            return;
        }

        let metrics = self.font.metrics(&self.attributes);
        let brush = SolidBrush::new(*style);
        let line_height = metrics.vertical_metrics().line_height();

        let mut height = 0;
        let mut line_count = 0;

        for line in &self.lines {
            if line_count >= self.max_lines {
                break;
            }
            line_count += 1;

            let line_x = self
                .alignment
                .align(size.width as i32, line.pixel_width as i32)
                + origin.x;

            let mut x = 0;

            let line_offset = Point::new(line_x, origin.y + height);
            let line_bounding_box =
                Rectangle::new(line_offset, Size::new(line.pixel_width as u32, line_height));

            if !line_bounding_box.intersects(&clip_rect) {
                height += line_height as i32;
                if height >= size.height as i32 {
                    break;
                }
                continue;
            }
            let range = line.range.0 as usize..line.range.1 as usize;
            let Some(s) = self.text.as_ref().get(range) else {
                continue; // Skip invalid lines
            };
            render_target.draw_glyphs(
                line_offset,
                &brush,
                s.chars().map(|c| {
                    let glyph = Glyph {
                        character: c,
                        offset: Point::new(x, 0),
                    };
                    x += metrics.advance(glyph.character) as i32;
                    glyph
                }),
                self.font,
                &self.attributes,
            );

            height += line_height as i32;
        }
        let remaining_text = self.lines.last().map_or(self.text.as_ref(), |last_range| {
            // Get the remaining text after the last line
            self.text
                .as_ref()
                .get(last_range.range.1 as usize..)
                .unwrap_or("")
        });
        if remaining_text.is_empty() {
            return;
        }
        // Pass false for precise wrapping, it doesn't affect rendered lines.
        // The width passed here should be the advance-based width, not the tighter precise
        //  bounding box width so that the text is wrapped the same.
        let mut word_wrap = WordWrap::new(remaining_text, size.width, &metrics, false);
        let mut character_wrap = CharacterWrap::new(remaining_text, size.width, &metrics, false);
        let wrap = core::iter::from_fn(|| match self.wrap {
            WrapStrategy::Word => word_wrap.next(),
            WrapStrategy::Character => character_wrap.next(),
        });
        let clip_rect = render_target.clip_rect();

        for line in wrap {
            if line_count >= self.max_lines {
                break;
            }
            line_count += 1;

            let width = line.width;

            let line_x = self.alignment.align(size.width as i32, width as i32) + origin.x;
            let mut x = 0;

            let line_offset = Point::new(line_x, origin.y + height);
            let line_bounding_box = Rectangle::new(line_offset, Size::new(width, line_height));
            if line_bounding_box.origin.y > clip_rect.origin.y + clip_rect.size.height as i32 {
                break;
            }
            if (line_bounding_box.origin.y + line_bounding_box.size.height as i32)
                < clip_rect.origin.y
            {
                height += line_height as i32;
                if height >= size.height as i32 {
                    break;
                }
                continue;
            }

            render_target.draw_glyphs(
                line_offset,
                &brush,
                line.content.chars().map(|c| {
                    let glyph = Glyph {
                        character: c,
                        offset: Point::new(x, 0),
                    };
                    x += metrics.advance(glyph.character) as i32;
                    glyph
                }),
                self.font,
                &self.attributes,
            );

            height += line_height as i32;
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        domain: &AnimationDomain,
    ) {
        let origin = Interpolate::interpolate(source.origin, target.origin, domain.factor);
        let size = Interpolate::interpolate(source.size, target.size, domain.factor);
        let attributes = Interpolate::interpolate(
            source.attributes.clone(),
            target.attributes.clone(),
            domain.factor,
        );

        Text {
            text: target.text.as_ref(),
            origin,
            size,
            font: target.font,
            attributes,
            alignment: target.alignment,
            lines: target.lines.clone(),
            max_lines: target.max_lines,
            wrap: target.wrap,
        }
        .render(render_target, style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::CharacterBufferFont;
    use crate::view::HorizontalTextAlignment;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    #[test]
    fn animated_join_at_start() {
        let font = CharacterBufferFont;
        let source = Text::new(
            (0, 0),
            (100, 50),
            &font,
            "Hello",
            (),
            HorizontalTextAlignment::Leading,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );
        let mut target = Text::new(
            (50, 25),
            (200, 100),
            &font,
            "World",
            (),
            HorizontalTextAlignment::Center,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );

        target.join_from(&source, &animation_domain(0));

        // At factor 0, should have source's position, size, text and font
        assert_eq!(target.origin, source.origin);
        assert_eq!(target.size, source.size);
        assert_eq!(target.text, target.text);
        assert_eq!(target.alignment, target.alignment);
    }

    #[test]
    fn animated_join_at_end() {
        let font = CharacterBufferFont;
        let source = Text::new(
            (0, 0),
            (100, 50),
            &font,
            "Hello",
            (),
            HorizontalTextAlignment::Leading,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );
        let original_target = Text::new(
            (50, 25),
            (200, 100),
            &font,
            "World",
            (),
            HorizontalTextAlignment::Center,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        // At factor 255, should be identical to target
        assert_eq!(target.origin, original_target.origin);
        assert_eq!(target.size, original_target.size);
        assert_eq!(target.text, original_target.text);
        assert_eq!(target.alignment, original_target.alignment);
    }

    #[test]
    fn animated_join_interpolates_position_and_size() {
        let font = CharacterBufferFont;
        let source = Text::new(
            (0, 0),
            (50, 25),
            &font,
            "Start",
            (),
            HorizontalTextAlignment::Leading,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );
        let original_target = Text::new(
            (100, 50),
            (150, 75),
            &font,
            "End",
            (),
            HorizontalTextAlignment::Trailing,
            Vec::new(),
            100,
            WrapStrategy::Word,
        );
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(128));

        // Position and size should be interpolated
        assert!(target.origin.0 > source.origin.0 && target.origin.0 < original_target.origin.0);
        assert!(target.origin.1 > source.origin.1 && target.origin.1 < original_target.origin.1);
        assert!(target.size.0 > source.size.0 && target.size.0 < original_target.size.0);
        assert!(target.size.1 > source.size.1 && target.size.1 < original_target.size.1);

        // Text and alignment should come from target
        assert_eq!(target.text, original_target.text);
        assert_eq!(target.alignment, original_target.alignment);
    }
}
