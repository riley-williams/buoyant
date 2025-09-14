use core::ops::Range;

use heapless::Vec;

use crate::{
    font::{FontMetrics, FontRender},
    primitives::{Interpolate, Point, Size, geometry::Rectangle},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{Glyph, RenderTarget, SolidBrush},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub range: Range<usize>,
    pub pixel_width: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a, T, F, const LINES: usize> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a F,
    pub text: T,
    pub alignment: HorizontalTextAlignment,
    pub lines: Vec<Line, LINES>,
}

impl<'a, T: AsRef<str>, F> Text<'a, T, F, 8> {
    pub fn new(
        origin: Point,
        size: Size,
        font: &'a F,
        text: T,
        alignment: HorizontalTextAlignment,
        lines: Vec<Line, 8>,
    ) -> Self {
        Self {
            origin,
            size,
            font,
            text,
            alignment,
            lines,
        }
    }
}
impl<T: Clone, F, const N: usize> Clone for Text<'_, T, F, N> {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            size: self.size,
            font: self.font,
            text: self.text.clone(),
            alignment: self.alignment,
            lines: self.lines.clone(),
        }
    }
}

impl<T: AsRef<str>, F, const N: usize> AnimatedJoin for Text<'_, T, F, N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // Text content (and line breaks) jump
        self.origin = Interpolate::interpolate(source.origin, self.origin, domain.factor);
        self.size = Interpolate::interpolate(source.size, self.size, domain.factor);
    }
}

impl<C: Copy, T: AsRef<str> + Clone, F: FontRender<C>, const LINE_BREAKS: usize> Render<C>
    for Text<'_, T, F, LINE_BREAKS>
{
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let clip_rect = render_target.clip_rect();
        let bounding_box = Rectangle::new(self.origin, self.size);
        if self.size.area() == 0 || !bounding_box.intersects(&clip_rect) {
            return;
        }

        let metrics = self.font.metrics();

        let brush = SolidBrush::new(*style);

        let line_height = metrics.default_line_height();

        let mut height = 0;

        for line in &self.lines {
            let line_x = self
                .alignment
                .align(self.size.width as i32, line.pixel_width as i32)
                + self.origin.x;

            let mut x = 0;

            let line_offset = Point::new(line_x, self.origin.y + height);
            let line_bounding_box =
                Rectangle::new(line_offset, Size::new(line.pixel_width, line_height));

            if !line_bounding_box.intersects(&clip_rect) {
                height += line_height as i32;
                if height >= self.size.height as i32 {
                    break;
                }
                continue;
            }
            let Some(s) = self.text.as_ref().get(line.range.clone()) else {
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
            );

            height += line_height as i32;
            if height >= self.size.height as i32 {
                break;
            }
        }
        let remaining_text = self.lines.last().map_or(self.text.as_ref(), |last_range| {
            // Get the remaining text after the last line
            self.text.as_ref().get(last_range.range.end..).unwrap_or("")
        });
        if remaining_text.is_empty() {
            return;
        }

        let wrap = WhitespaceWrap::new(remaining_text, self.size.width, &metrics);

        let clip_rect = render_target.clip_rect();

        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = metrics.str_width(line);

            let line_x = self.alignment.align(self.size.width as i32, width as i32) + self.origin.x;
            let mut x = 0;

            let line_offset = Point::new(line_x, self.origin.y + height);
            let line_bounding_box = Rectangle::new(line_offset, Size::new(width, line_height));
            if line_bounding_box.origin.y > clip_rect.origin.y + clip_rect.size.height as i32 {
                break;
            }
            if (line_bounding_box.origin.y + line_bounding_box.size.height as i32)
                < clip_rect.origin.y
            {
                height += line_height as i32;
                if height >= self.size.height as i32 {
                    break;
                }
                continue;
            }

            render_target.draw_glyphs(
                line_offset,
                &brush,
                line.chars().map(|c| {
                    let glyph = Glyph {
                        character: c,
                        offset: Point::new(x, 0),
                    };
                    x += metrics.advance(glyph.character) as i32;
                    glyph
                }),
                self.font,
            );

            height += line_height as i32;
            if height >= self.size.height as i32 {
                break;
            }
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
        Text {
            text: target.text.as_ref(),
            origin,
            size,
            font: target.font,
            alignment: target.alignment,
            lines: target.lines.clone(),
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
            Point::new(0, 0),
            Size::new(100, 50),
            &font,
            "Hello",
            HorizontalTextAlignment::Leading,
            Vec::new(),
        );
        let mut target = Text::new(
            Point::new(50, 25),
            Size::new(200, 100),
            &font,
            "World",
            HorizontalTextAlignment::Center,
            Vec::new(),
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
            Point::new(0, 0),
            Size::new(100, 50),
            &font,
            "Hello",
            HorizontalTextAlignment::Leading,
            Vec::new(),
        );
        let original_target = Text::new(
            Point::new(50, 25),
            Size::new(200, 100),
            &font,
            "World",
            HorizontalTextAlignment::Center,
            Vec::new(),
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
            Point::new(0, 0),
            Size::new(50, 25),
            &font,
            "Start",
            HorizontalTextAlignment::Leading,
            Vec::new(),
        );
        let original_target = Text::new(
            Point::new(100, 50),
            Size::new(150, 75),
            &font,
            "End",
            HorizontalTextAlignment::Trailing,
            Vec::new(),
        );
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(128));

        // Position and size should be interpolated
        assert!(target.origin.x > source.origin.x && target.origin.x < original_target.origin.x);
        assert!(target.origin.y > source.origin.y && target.origin.y < original_target.origin.y);
        assert!(
            target.size.width > source.size.width && target.size.width < original_target.size.width
        );
        assert!(
            target.size.height > source.size.height
                && target.size.height < original_target.size.height
        );

        // Text and alignment should come from target
        assert_eq!(target.text, original_target.text);
        assert_eq!(target.alignment, original_target.alignment);
    }
}
