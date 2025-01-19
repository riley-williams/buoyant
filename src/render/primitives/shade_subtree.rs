use crate::{
    pixel::Interpolate,
    render::{AnimationDomain, CharacterRender, CharacterRenderTarget, EmbeddedGraphicsRender},
};

use embedded_graphics::prelude::PixelColor;
use embedded_graphics_core::draw_target::DrawTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadeSubtree<C, T> {
    style: C,
    subtree: T,
}

impl<C, T> ShadeSubtree<C, T> {
    pub fn new(style: C, subtree: T) -> Self {
        Self { style, subtree }
    }
}

impl<C: PixelColor + Interpolate, T: EmbeddedGraphicsRender<C>> EmbeddedGraphicsRender<C>
    for ShadeSubtree<C, T>
{
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, _: &C) {
        self.subtree.render(render_target, &self.style);
    }

    fn render_animated(
        render_target: &mut impl DrawTarget<Color = C>,
        source: &Self,
        target: &Self,
        _: &C,
        config: &AnimationDomain,
    ) {
        let style = Interpolate::interpolate(source.style, target.style, config.factor);
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            &style,
            config,
        );
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        Self {
            // TODO: This "jumps" to the target style, should be an interpolated intermetiate
            style: target.style,
            subtree: T::join(source.subtree, target.subtree, config),
        }
    }
}

impl<C: Clone, T: CharacterRender<C>> CharacterRender<C> for ShadeSubtree<C, T> {
    fn render(&self, render_target: &mut impl CharacterRenderTarget<Color = C>, _: &C) {
        self.subtree.render(render_target, &self.style);
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        _: &C,
        config: &AnimationDomain,
    ) {
        // TODO: This should be animated, but then I'd need to update all the char types in tests
        // let style = Interpolate::interpolate(source.style, target.style, config.factor);
        // T::render_animated(
        //     render_target,
        //     &source.subtree,
        //     &target.subtree,
        //     &style,
        //     config,
        // );
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            &target.style,
            config,
        );
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        Self {
            // TODO: This "jumps" to the target style, should be an interpolated intermetiate
            style: target.style,
            subtree: T::join(source.subtree, target.subtree, config),
        }
    }
}
