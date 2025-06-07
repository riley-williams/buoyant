use crate::primitives::{Interpolate as _, Point};

use super::{AnimatedJoin, AnimationDomain};

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
        if domain.factor == 0 {
            return source;
        }
        Self {
            origin: Point::interpolate(source.origin, target.origin, domain.factor),
            image: target.image,
        }
    }
}

// This is an implementation that uses the more generic brush
//
// use crate::primitives::geometry::Rectangle;
// use crate::render_target::{Brush, ImageBrush, RenderTarget};
// impl<C: From<<I as Brush>::ColorFormat>, I: ImageBrush> Render<C> for Image<'_, I> {
//     fn render(
//         &self,
//         render_target: &mut impl RenderTarget<ColorFormat = C>,
//         _style: &C,
//         offset: crate::primitives::Point,
//     ) {
//         let origin = self.origin + offset;
//         let rectangle = Rectangle::new(Point::new(0, 0), self.image.size());
//         render_target.fill(origin, self.image, None, &rectangle);
//     }
//
//     fn render_animated(
//         render_target: &mut impl RenderTarget<ColorFormat = C>,
//         source: &Self,
//         target: &Self,
//         _style: &C,
//         offset: crate::primitives::Point,
//         domain: &super::AnimationDomain,
//     ) {
//         let origin = offset + Point::interpolate(source.origin, target.origin, domain.factor);
//         let rectangle = Rectangle::new(Point::new(0, 0), target.image.size());
//         render_target.fill(origin, target.image, None, &rectangle);
//     }
// }

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics {
    use embedded_graphics::{draw_target::DrawTargetExt, image::ImageDrawable};

    use crate::{
        primitives::{Interpolate as _, Point},
        render::Render,
        render_target::RenderTarget,
        surface::AsDrawTarget,
    };

    use super::Image;
    impl<I: ImageDrawable> Render<I::Color> for Image<'_, I> {
        fn render(
            &self,
            render_target: &mut impl RenderTarget<ColorFormat = I::Color>,
            _style: &I::Color,
            offset: crate::primitives::Point,
        ) {
            // TODO: Only render a sub-image if the image is clipped?
            let origin = self.origin + offset;
            _ = self.image.draw(
                &mut render_target
                    .raw_surface()
                    .draw_target()
                    .translated(origin.into()),
            );
        }

        fn render_animated(
            render_target: &mut impl RenderTarget<ColorFormat = I::Color>,
            source: &Self,
            target: &Self,
            style: &I::Color,
            offset: crate::primitives::Point,
            domain: &super::AnimationDomain,
        ) {
            let origin = offset + Point::interpolate(source.origin, target.origin, domain.factor);
            if domain.factor == 0 {
                source.render(render_target, style, origin);
            } else {
                target.render(render_target, style, origin);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    // Mock image data for testing
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct MockImageData {
        width: u32,
        height: u32,
    }

    #[test]
    fn animated_join_extremities() {
        let source_image_data = MockImageData {
            width: 10,
            height: 15,
        };

        let target_image_data = MockImageData {
            width: 20,
            height: 20,
        };

        let source = Image::new(Point::new(0, 0), &source_image_data);
        let target = Image::new(Point::new(50, 25), &target_image_data);

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));
        assert_eq!(result.origin, source.origin);
        assert_eq!(result.image, source.image);

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));
        assert_eq!(result.origin, target.origin);
        assert_eq!(result.image, target.image);
    }
}
