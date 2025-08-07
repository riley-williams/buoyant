use crate::{
    primitives::{Frame, Interpolate, Point},
    render::{AnimatedJoin, Render},
    render_target::RenderTarget,
};

use super::AnimationDomain;

/// A node that tracks a frame and contains a child view.
///
/// This is used to track view frames in event handlers, where only the child frame
/// would otherwise be available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container<T> {
    pub frame: Frame,
    pub child: T,
}

impl<T> Container<T> {
    pub const fn new(frame: Frame, child: T) -> Self {
        Self { frame, child }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for Container<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.frame = Frame::interpolate(source.frame, self.frame, domain.factor);
        self.child.join_from(&source.child, domain);
    }
}

impl<T: Render<Color>, Color> Render<Color> for Container<T> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        style: &Color,
        offset: Point,
    ) {
        self.child.render(render_target, style, offset);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        T::render_animated(
            render_target,
            &source.child,
            &target.child,
            style,
            offset,
            domain,
        );
    }
}
