use crate::render::{AnimationDomain, Render};

use embedded_graphics::{prelude::PixelColor, primitives::PrimitiveStyle};
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

impl<C: PixelColor, T: Render<C>> Render<C> for ShadeSubtree<PrimitiveStyle<C>, T> {
    fn render(&self, render_target: &mut impl DrawTarget<Color = C>, _: &PrimitiveStyle<C>) {
        self.subtree.render(render_target, &self.style);
    }

    fn render_animated(
        render_target: &mut impl DrawTarget<Color = C>,
        source: &Self,
        _: &PrimitiveStyle<C>,
        target: &Self,
        _: &PrimitiveStyle<C>,
        config: &AnimationDomain,
    ) {
        T::render_animated(
            render_target,
            &source.subtree,
            &source.style,
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
