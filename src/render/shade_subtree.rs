use crate::{
    primitives::{Interpolate, Point},
    render::{AnimationDomain, CharacterRender, CharacterRenderTarget},
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadeSubtree<C, T> {
    pub style: C,
    pub subtree: T,
}

impl<C, T> ShadeSubtree<C, T> {
    pub const fn new(style: C, subtree: T) -> Self {
        Self { style, subtree }
    }
}

impl<C: Clone + Interpolate, T: AnimatedJoin> AnimatedJoin for ShadeSubtree<C, T> {
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        Self {
            style: Interpolate::interpolate(source.style, target.style, config.factor),
            subtree: T::join(source.subtree, target.subtree, config),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::{
        primitives::{Interpolate, Point},
        render::EmbeddedGraphicsRender,
    };

    use super::{AnimationDomain, ShadeSubtree};
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics_core::draw_target::DrawTarget;

    impl<C: PixelColor + Interpolate, T: EmbeddedGraphicsRender<C>> EmbeddedGraphicsRender<C>
        for ShadeSubtree<C, T>
    {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, _: &C, offset: Point) {
            self.subtree.render(render_target, &self.style, offset);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            _: &C,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            let style = Interpolate::interpolate(source.style, target.style, domain.factor);
            T::render_animated(
                render_target,
                &source.subtree,
                &target.subtree,
                &style,
                offset,
                domain,
            );
        }
    }
}

impl<C: Interpolate, T: CharacterRender<C>> CharacterRender<C> for ShadeSubtree<C, T> {
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        _: &C,
        offset: Point,
    ) {
        self.subtree.render(render_target, &self.style, offset);
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        _: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        let style = Interpolate::interpolate(source.style, target.style, domain.factor);
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            &style,
            offset,
            domain,
        );
    }
}
