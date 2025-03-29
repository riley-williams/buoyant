use crate::primitives::{Interpolate as _, Point};
use crate::render_target::{Image as DrawImage, ImageBrush, RenderTarget};

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
// FIXME: Implement Render for Image
impl<C: From<<T as crate::render_target::Image>::ColorFormat>, T: DrawImage> Render<C>
    for Image<'_, T>
{
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        _style: &C,
        offset: crate::primitives::Point,
    ) {
        let origin = self.origin + offset;
        let rectangle = crate::render_target::geometry::Rectangle::new(
            crate::render_target::geometry::Point::new(0, 0),
            (self.image.width(), self.image.height()),
        );
        let brush = ImageBrush::new(self.image);
        render_target.fill(origin.into(), &brush, None, &rectangle);
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
        let rectangle = crate::render_target::geometry::Rectangle::new(
            crate::render_target::geometry::Point::new(0, 0),
            (target.image.width(), target.image.height()),
        );
        let brush = ImageBrush::new(target.image);
        render_target.fill(origin.into(), &brush, None, &rectangle);
    }
}
