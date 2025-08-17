use crate::{
    primitives::Interpolate,
    render::{AnimationDomain, Render, RenderTarget},
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HintBackground<T, C> {
    pub subtree: T,
    pub color: C,
}

impl<T, C> HintBackground<T, C> {
    pub const fn new(subtree: T, color: C) -> Self {
        Self { subtree, color }
    }
}

impl<T: AnimatedJoin, C: Interpolate + Copy> AnimatedJoin for HintBackground<T, C> {
    fn join_from(&mut self, source: &Self, config: &AnimationDomain) {
        self.color = Interpolate::interpolate(source.color, self.color, config.factor);
        self.subtree.join_from(&source.subtree, config);
    }
}

impl<T: Render<C>, C: Interpolate + Copy> Render<C> for HintBackground<T, C> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let color = self.color;
        render_target.with_layer(
            |l| l.hint_background(color),
            |render_target| {
                self.subtree.render(render_target, style);
            },
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        domain: &AnimationDomain,
    ) {
        let color = Interpolate::interpolate(source.color, target.color, domain.factor);
        render_target.with_layer(
            |l| l.hint_background(color),
            |render_target| {
                T::render_animated(
                    render_target,
                    &source.subtree,
                    &target.subtree,
                    style,
                    domain,
                );
            },
        );
    }
}
