use crate::render::{AnimatedJoin, AnimationDomain, ContentShape, IntrinsicShape, Render};
use crate::render_target::RenderTarget;

/// A render tree node that overrides the content shape of its subtree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentShapeOverride<T> {
    pub subtree: T,
    pub shape: ContentShape,
}

impl<T> ContentShapeOverride<T> {
    pub const fn new(subtree: T, shape: ContentShape) -> Self {
        Self { subtree, shape }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for ContentShapeOverride<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.subtree.join_from(&source.subtree, domain);
    }
}

impl<T: Render<Color>, Color> Render<Color> for ContentShapeOverride<T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        self.subtree.render(render_target, style);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            style,
            domain,
        );
    }
}

impl<T: IntrinsicShape> IntrinsicShape for ContentShapeOverride<T> {
    fn content_shape(&self) -> ContentShape {
        self.shape.clone()
    }
}
