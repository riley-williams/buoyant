use crate::{
    primitives::{Interpolate as _, Point},
    render_target::RenderTarget,
};

use super::{AnimatedJoin, AnimationDomain, Render};

/// A render tree item that offsets its children by a fixed amount.
/// The offset is animated, resulting in all children moving in unison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset<T> {
    pub offset: Point,
    pub subtree: T,
}

impl<T> Offset<T> {
    /// Create a new offset render tree item
    pub const fn new(offset: Point, subtree: T) -> Self {
        Self { offset, subtree }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for Offset<T> {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let subtree = T::join(source.subtree, target.subtree, domain);
        let offset = Point::interpolate(source.offset, target.offset, domain.factor);
        Self { offset, subtree }
    }
}

impl<T: Render<C>, C> Render<C> for Offset<T> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        self.subtree
            .render(render_target, style, self.offset + offset);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
}
