use crate::{
    font::{FontMetrics, FontRender},
    primitives::{Interpolate, Point, Size},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{Glyph, RenderTarget, SolidBrush},
    view::{HorizontalTextAlignment, WhitespaceWrap},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a, T, F> {
    pub origin: Point,
    pub size: Size,
    pub font: &'a F,
    pub text: T,
    pub alignment: HorizontalTextAlignment,
}

impl<T: Clone, F> Clone for Text<'_, T, F> {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            size: self.size,
            font: self.font,
            text: self.text.clone(),
            alignment: self.alignment,
        }
    }
}

impl<T: AsRef<str>, F> AnimatedJoin for Text<'_, T, F> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // Text content jumps
        self.origin = Interpolate::interpolate(source.origin, self.origin, domain.factor);
        self.size = Interpolate::interpolate(source.size, self.size, domain.factor);
    }
}

impl<C: Copy, T: AsRef<str> + Clone, F: FontRender<C>> Render<C> for Text<'_, T, F> {
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
        let mut target = Text {
            origin: Point::new(50, 25),
            size: Size::new(200, 100),
            font: &font,
            text: "World",
            alignment: HorizontalTextAlignment::Center,
        };

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
        let source = Text {
            origin: Point::new(0, 0),
            size: Size::new(100, 50),
            font: &font,
            text: "Hello",
            alignment: HorizontalTextAlignment::Leading,
        };
        let original_target = Text {
            origin: Point::new(50, 25),
            size: Size::new(200, 100),
            font: &font,
            text: "World",
            alignment: HorizontalTextAlignment::Center,
        };
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
        let source = Text {
            origin: Point::new(0, 0),
            size: Size::new(50, 25),
            font: &font,
            text: "Start",
            alignment: HorizontalTextAlignment::Leading,
        };
        let original_target = Text {
            origin: Point::new(100, 50),
            size: Size::new(150, 75),
            font: &font,
            text: "End",
            alignment: HorizontalTextAlignment::Trailing,
        };
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
