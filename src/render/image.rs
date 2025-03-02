use embedded_graphics::Drawable;
use embedded_graphics::{image::ImageDrawable, prelude::PixelColor};

use crate::primitives::{Interpolate as _, Point};

use super::{AnimatedJoin, AnimationDomain, EmbeddedGraphicsRender};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<'a, T> {
    pub origin: Point,
    pub image: &'a T,
}

impl<'a, T> Image<'a, T> {
    pub const fn new(origin: Point, image: &'a T) -> Self {
        Self { origin, image }
    }
}

impl<T> AnimatedJoin for Image<'_, T> {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        Self {
            origin: Point::interpolate(source.origin, target.origin, domain.factor),
            image: source.image,
        }
    }
}

impl<C: PixelColor, T: ImageDrawable<Color = C>> EmbeddedGraphicsRender<C> for Image<'_, T> {
    fn render(
        &self,
        render_target: &mut impl embedded_graphics::prelude::DrawTarget<Color = C>,
        _style: &C,
        offset: crate::primitives::Point,
    ) {
        let origin = self.origin + offset;
        let image = embedded_graphics::image::Image::new(self.image, origin.into());
        _ = image.draw(render_target);
    }

    fn render_animated(
        render_target: &mut impl embedded_graphics::prelude::DrawTarget<Color = C>,
        source: &Self,
        target: &Self,
        _style: &C,
        offset: crate::primitives::Point,
        domain: &super::AnimationDomain,
    ) {
        let origin = offset + Point::interpolate(source.origin, target.origin, domain.factor);
        let image = embedded_graphics::image::Image::new(target.image, origin.into());
        _ = image.draw(render_target);
    }
}
