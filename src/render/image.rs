use core::marker::PhantomData;

use crate::primitives::{Interpolate as _, Point};

use super::{AnimatedJoin, AnimationDomain};

/// Render mode marker for rendering an image in its original colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Original;

/// Render mode marker for rendering an image as a template, replacing
/// white pixels with the foreground color and rendering black pixels
/// transparent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Template;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a, T: ?Sized, Mode = Original> {
    pub origin: Point,
    pub image: &'a T,
    pub _mode: PhantomData<Mode>,
}

impl<T: ?Sized, Mode> Clone for Image<'_, T, Mode> {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            image: self.image,
            _mode: PhantomData,
        }
    }
}

impl<'a, T: ?Sized, Mode> Image<'a, T, Mode> {
    pub const fn new(origin: Point, image: &'a T) -> Self {
        Self {
            origin,
            image,
            _mode: PhantomData,
        }
    }
}

impl<T: ?Sized, Mode> AnimatedJoin for Image<'_, T, Mode> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        // image content jumps
        self.origin = Point::interpolate(source.origin, self.origin, domain.factor);
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
    use core::marker::PhantomData;
    use embedded_graphics::{
        Pixel,
        draw_target::{DrawTarget, DrawTargetExt as _},
        geometry::Dimensions,
        image::{ImageDrawable, ImageDrawableExt},
        pixelcolor::{GrayColor, PixelColor},
        primitives::PointsIter,
    };

    use crate::{
        primitives::{Interpolate, Point, geometry::Rectangle},
        render::{ContentShape, IntrinsicShape, Render},
        render_target::{
            RenderTarget,
            surface::{AsDrawTarget, Visibility},
        },
    };

    use super::Image;

    struct TemplatedTarget<'a, T: DrawTarget, C> {
        target: &'a mut T,
        color: T::Color,
        background_color: T::Color,
        _template_color: PhantomData<C>,
    }

    impl<T: DrawTarget<Color: Interpolate>, C: GrayColor> TemplatedTarget<'_, T, C> {
        fn template_color(
            color: C,
            background: T::Color,
            foreground: T::Color,
        ) -> Option<T::Color> {
            let luma = color.luma();
            if luma == 0 {
                None
            } else {
                // Gray2 and Gray4 use their native ranges, rather than 0..=255.
                // This math is expected to be mostly inlined despite not being const
                let alpha = (u16::from(luma) * 255 / u16::from(C::WHITE.luma())) as u8;
                Some(Interpolate::interpolate(background, foreground, alpha))
            }
        }
    }

    impl<T: DrawTarget, C> Dimensions for TemplatedTarget<'_, T, C> {
        fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
            self.target.bounding_box()
        }
    }

    impl<T: DrawTarget<Color: Interpolate>, C: GrayColor> DrawTarget for TemplatedTarget<'_, T, C> {
        type Color = C;

        type Error = T::Error;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
        {
            let pixels = pixels.into_iter().filter_map(|p| {
                Self::template_color(p.1, self.background_color, self.color)
                    .map(|color| Pixel(p.0, color))
            });
            self.target.draw_iter(pixels)
        }

        fn fill_contiguous<I>(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            colors: I,
        ) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Self::Color>,
        {
            self.draw_iter(
                area.points()
                    .zip(colors)
                    .map(|(pos, color)| Pixel(pos, color)),
            )
        }

        fn fill_solid(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            color: Self::Color,
        ) -> Result<(), Self::Error> {
            if let Some(color) = Self::template_color(color, self.background_color, self.color) {
                self.target.fill_solid(area, color)?;
            }
            Ok(())
        }

        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            if let Some(color) = Self::template_color(color, self.background_color, self.color) {
                self.target.clear(color)?;
            }
            Ok(())
        }
    }

    impl<I: ImageDrawable> Render<I::Color> for Image<'_, I, super::Original> {
        fn render(
            &self,
            render_target: &mut impl RenderTarget<ColorFormat = I::Color>,
            _style: &I::Color,
        ) {
            draw_image(
                &mut render_target.raw_surface(self.origin).draw_target(),
                self.image,
            );
        }

        fn render_animated(
            render_target: &mut impl RenderTarget<ColorFormat = I::Color>,
            source: &Self,
            target: &Self,
            _style: &I::Color,
            domain: &super::AnimationDomain,
        ) {
            let origin = Point::interpolate(source.origin, target.origin, domain.factor);
            draw_image(
                &mut render_target.raw_surface(origin).draw_target(),
                target.image,
            );
        }
    }

    /// Draws `image` at the origin of an image-local target.
    ///
    /// The image is classified against the clip rect once here, rather than
    /// once per drawing call issued by the image decoder. Images that are fully
    /// visible, which is the common case, are drawn through a surface that only
    /// applies a translation.
    fn draw_image<I: ImageDrawable>(target: &mut impl DrawTarget<Color = I::Color>, image: &I) {
        let bounds = Rectangle::new(Point::zero(), image.size().into());

        match Visibility::classify(&target.bounding_box().into(), &bounds) {
            Visibility::Full => {
                _ = image.draw(target);
            }
            Visibility::Clipped(visible) => {
                let offset = visible.origin.into();
                _ = image
                    .sub_image(&visible.into())
                    .draw(&mut target.translated(offset));
            }
            Visibility::Hidden => (),
        }
    }

    impl<I, TargetColor> Render<TargetColor> for Image<'_, I, super::Template>
    where
        I: ImageDrawable,
        I::Color: GrayColor,
        TargetColor: PixelColor + Interpolate,
    {
        fn render(
            &self,
            render_target: &mut impl RenderTarget<ColorFormat = TargetColor>,
            style: &TargetColor,
        ) {
            draw_template(render_target, self.image, self.origin, *style);
        }

        fn render_animated(
            render_target: &mut impl RenderTarget<ColorFormat = TargetColor>,
            source: &Self,
            target: &Self,
            style: &TargetColor,
            domain: &super::AnimationDomain,
        ) {
            let origin = Point::interpolate(source.origin, target.origin, domain.factor);
            draw_template(render_target, target.image, origin, *style);
        }
    }

    /// Draws `image` as a template with its top left corner at `origin` in the
    /// local coordinate space.
    ///
    /// Classified against the clip rect once, as in [`draw_image`].
    fn draw_template<I, TargetColor>(
        render_target: &mut impl RenderTarget<ColorFormat = TargetColor>,
        image: &I,
        origin: Point,
        color: TargetColor,
    ) where
        I: ImageDrawable,
        I::Color: GrayColor,
        TargetColor: PixelColor + Interpolate,
    {
        // Without a background hint, nonzero pixels use the foreground color,
        // matching the fallback used for antialiased text.
        let background_color = render_target.background_hint().unwrap_or(color);
        let color = Interpolate::interpolate(background_color, color, render_target.alpha());
        let mut surface = render_target.raw_surface(origin);
        let mut draw_target = surface.draw_target();
        draw_image(
            &mut TemplatedTarget::<_, I::Color> {
                color,
                target: &mut draw_target,
                background_color,
                _template_color: PhantomData,
            },
            image,
        );
    }

    impl<I: ImageDrawable, Mode> IntrinsicShape for Image<'_, I, Mode> {
        fn content_shape(&self) -> ContentShape {
            let size = self.image.size().into();
            Rectangle::new(self.origin, size).into()
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

        let source: Image<'_, MockImageData, Original> =
            Image::new(Point::new(0, 0), &source_image_data);
        let original_target: Image<'_, MockImageData, Original> =
            Image::new(Point::new(50, 25), &target_image_data);

        let mut target = original_target.clone();
        target.join_from(&source, &animation_domain(0));
        assert_eq!(target.origin, source.origin);
        assert_eq!(target.image, target.image);

        let mut target = original_target.clone();
        target.join_from(&source, &animation_domain(255));
        assert_eq!(target.origin, original_target.origin);
        assert_eq!(target.image, original_target.image);
    }
}
