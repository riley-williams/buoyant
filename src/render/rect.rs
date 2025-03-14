use crate::primitives::Interpolate;
use crate::primitives::{Point, Size};
use crate::render::{AnimationDomain, CharacterRender, CharacterRenderTarget};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

impl AnimatedJoin for Rect {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let origin = Point::interpolate(source.origin, target.origin, domain.factor);
        let size = Size::interpolate(source.size, target.size, domain.factor);
        Self::new(origin, size)
    }
}

impl<C> CharacterRender<C> for Rect {
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        style: &C,
        offset: Point,
    ) {
        let origin = self.origin + offset;
        for y in origin.y..origin.y + self.size.height as i16 {
            for x in origin.x..origin.x + self.size.width as i16 {
                render_target.draw_color(Point::new(x, y), style);
            }
        }
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        // TODO: expecting these clones to be optimized away, check
        AnimatedJoin::join(source.clone(), target.clone(), domain).render(
            render_target,
            style,
            offset,
        );
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics::primitives::{PrimitiveStyle, StyledDrawable as _};
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::primitives::Point;
    use crate::render::{AnimatedJoin, AnimationDomain, EmbeddedGraphicsRender};

    use super::Rect;

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for Rect {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            let origin = self.origin + offset;
            let eg_rect =
                embedded_graphics::primitives::Rectangle::new(origin.into(), self.size.into());
            _ = eg_rect.draw_styled(&PrimitiveStyle::with_fill(*style), render_target);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            // TODO: expecting these clones to be optimized away, check
            AnimatedJoin::join(source.clone(), target.clone(), domain).render(
                render_target,
                style,
                offset,
            );
        }
    }
}
