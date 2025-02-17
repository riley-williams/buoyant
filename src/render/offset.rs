use crate::{pixel::Interpolate as _, primitives::Point};

use super::{AnimationDomain, CharacterRender, CharacterRenderTarget};

/// A render tree item that offsets its children by a fixed amount.
/// The offset is animated, resulting in all children moving in unison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset<T> {
    pub offset: Point,
    pub subtree: T,
}

impl<T> Offset<T> {
    /// Create a new offset render tree item
    pub fn new(offset: Point, subtree: T) -> Self {
        Self { offset, subtree }
    }
}

impl<T: CharacterRender<C>, C> CharacterRender<C> for Offset<T> {
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        style: &C,
        offset: Point,
    ) {
        self.subtree
            .render(render_target, style, self.offset + offset);
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &super::AnimationDomain,
    ) {
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            style,
            Point::interpolate(source.offset, target.offset, domain.factor) + offset,
            domain,
        );
    }

    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let subtree = T::join(source.subtree, target.subtree, domain);
        let offset = Point::interpolate(source.offset, target.offset, domain.factor);
        Self { offset, subtree }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics_core::draw_target::DrawTarget;

    use crate::pixel::Interpolate;
    use crate::primitives::Point;
    use crate::render::{AnimationDomain, EmbeddedGraphicsRender};

    use super::Offset;

    impl<T: EmbeddedGraphicsRender<C>, C: PixelColor> EmbeddedGraphicsRender<C> for Offset<T> {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            self.subtree
                .render(render_target, style, self.offset + offset);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &super::AnimationDomain,
        ) {
            T::render_animated(
                render_target,
                &source.subtree,
                &target.subtree,
                style,
                Point::interpolate(source.offset, target.offset, domain.factor) + offset,
                domain,
            );
        }

        fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
            let subtree = T::join(source.subtree, target.subtree, domain);
            let offset = Point::interpolate(source.offset, target.offset, domain.factor);
            Self { offset, subtree }
        }
    }
}
