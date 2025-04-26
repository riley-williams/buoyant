use crate::primitives::geometry::Rectangle;
use crate::primitives::{Interpolate as _, Point};
use crate::render_target::{Brush, ImageBrush, RenderTarget};

use super::{AnimatedJoin, AnimationDomain, Render};

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

impl<C: From<<I as Brush>::ColorFormat>, I: ImageBrush> Render<C> for Image<'_, I> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        _style: &C,
        offset: crate::primitives::Point,
    ) {
        let origin = self.origin + offset;
        let rectangle = Rectangle::new(Point::new(0, 0), self.image.size());
        render_target.fill(origin, self.image, None, &rectangle);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        _style: &C,
        offset: crate::primitives::Point,
        domain: &super::AnimationDomain,
    ) {
        let origin = offset + Point::interpolate(source.origin, target.origin, domain.factor);
        let rectangle = Rectangle::new(Point::new(0, 0), target.image.size());
        render_target.fill(origin, target.image, None, &rectangle);
    }
}
