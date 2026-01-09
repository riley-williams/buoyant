use crate::{
    primitives::{Interpolate, geometry::Rectangle},
    render_target::RenderTarget,
};

use super::{AnimatedJoin, AnimationDomain, Render};

/// A render tree node that clips its children
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clipped<T> {
    pub subtree: T,
    pub clip_rect: Rectangle,
}

impl<T> Clipped<T> {
    pub const fn new(subtree: T, clip_rect: Rectangle) -> Self {
        Self { subtree, clip_rect }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for Clipped<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.subtree.join_from(&source.subtree, domain);
        self.clip_rect = Rectangle::interpolate(
            source.clip_rect.clone(),
            self.clip_rect.clone(),
            domain.factor,
        );
    }
}

impl<T: Render<C>, C: Interpolate + Copy> Render<C> for Clipped<T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        render_target.with_layer(
            |l| l.clip(&self.clip_rect),
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
        domain: &super::AnimationDomain,
    ) {
        let clip_rect = Rectangle::interpolate(
            source.clip_rect.clone(),
            target.clip_rect.clone(),
            domain.factor,
        );
        render_target.with_layer(
            |l| l.clip(&clip_rect),
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
