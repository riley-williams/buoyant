use crate::{
    font::{FontMetrics, FontRender},
    primitives::{Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{Glyph, RenderTarget, SolidBrush},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'a, T, F> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a F,
    pub text: T,
    pub alignment: HorizontalTextAlignment,
}

impl<T: AsRef<str>, F> AnimatedJoin for Text<'_, T, F> {
    fn join(source: Self, mut target: Self, domain: &AnimationDomain) -> Self {
        if domain.factor == 0 {
            return source;
        }
        target.origin = Interpolate::interpolate(source.origin, target.origin, domain.factor);
        target.size = Interpolate::interpolate(source.size, target.size, domain.factor);
        target
    }
}

impl<C: Copy, T: AsRef<str>, F: FontRender<C>> Render<C> for Text<'_, T, F> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        if self.size.area() == 0 {
            return;
        }

        let metrics = self.font.metrics();

        let brush = SolidBrush::new(*style);

        let origin = self.origin + offset;
        let line_height = metrics.default_line_height();

        let mut height = 0;
        let wrap = WhitespaceWrap::new(self.text.as_ref(), self.size.width, &metrics);

        let metrics = self.font.metrics();

        for line in wrap {
            // TODO: WhitespaceWrap should also return the width of the line
            let width = metrics.str_width(line);

            let line_x = self.alignment.align(self.size.width as i32, width as i32) + origin.x;

            let mut x = 0;
            render_target.draw_glyphs(
                Point::new(line_x, origin.y + height),
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
        offset: Point,
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
        }
        .render(render_target, style, offset);
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
        let source = Text {
            origin: Point::new(0, 0),
            size: Size::new(100, 50),
            font: &font,
            text: "Hello",
            alignment: HorizontalTextAlignment::Leading,
        };
        let target = Text {
            origin: Point::new(50, 25),
            size: Size::new(200, 100),
            font: &font,
            text: "World",
            alignment: HorizontalTextAlignment::Center,
        };

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        // At factor 0, should have source's position, size, text and font
        assert_eq!(result.origin, source.origin);
        assert_eq!(result.size, source.size);
        assert_eq!(result.text, source.text);
        assert_eq!(result.alignment, source.alignment);
    }

    #[test]
    fn animated_join_at_end() {
        let font = CharacterBufferFont;
        let source = Text {
            origin: Point::new(0, 0),
            size: Size::new(100, 50),
            font: &font,
            text: "Hello",
            alignment: HorizontalTextAlignment::Leading,
        };
        let target = Text {
            origin: Point::new(50, 25),
            size: Size::new(200, 100),
            font: &font,
            text: "World",
            alignment: HorizontalTextAlignment::Center,
        };

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        // At factor 255, should be identical to target
        assert_eq!(result.origin, target.origin);
        assert_eq!(result.size, target.size);
        assert_eq!(result.text, target.text);
        assert_eq!(result.alignment, target.alignment);
    }

    #[test]
    fn animated_join_interpolates_position_and_size() {
        let font = CharacterBufferFont;
        let source = Text {
            origin: Point::new(0, 0),
            size: Size::new(50, 25),
            font: &font,
            text: "Start",
            alignment: HorizontalTextAlignment::Leading,
        };
        let target = Text {
            origin: Point::new(100, 50),
            size: Size::new(150, 75),
            font: &font,
            text: "End",
            alignment: HorizontalTextAlignment::Trailing,
        };

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(128));

        // Position and size should be interpolated
        assert!(result.origin.x > source.origin.x && result.origin.x < target.origin.x);
        assert!(result.origin.y > source.origin.y && result.origin.y < target.origin.y);
        assert!(result.size.width > source.size.width && result.size.width < target.size.width);
        assert!(result.size.height > source.size.height && result.size.height < target.size.height);

        // Text and alignment should come from target
        assert_eq!(result.text, target.text);
        assert_eq!(result.alignment, target.alignment);
    }
}
