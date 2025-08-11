use crate::{
    primitives::{geometry::Rectangle, Interpolate as _, Point, Size},
    render::{Animate, AnimatedJoin, Capsule, Offset, Render},
    render_target::RenderTarget,
};

// This hacks together scroll functionality from existing primitives, but
// a bespoke implementation will eventually replace it
type ScrolInner<T> = Offset<Animate<(Offset<T>, Option<Capsule>, Option<Capsule>), bool>>;

/// This is just a metadata structure that allows [`ScrollView`] to mutate its offset and scroll bars
/// without recomputing a new view tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollMetadata<T> {
    pub scroll_size: Size,
    pub inner_size: Size,
    pub(crate) inner: ScrolInner<T>,
}

impl<T> ScrollMetadata<T> {
    pub(crate) fn new(scroll_size: Size, inner_size: Size, inner: ScrolInner<T>) -> Self {
        Self {
            scroll_size,
            inner_size,
            inner,
        }
    }

    pub fn offset(&self) -> Point {
        self.inner.subtree.subtree.0.offset
    }

    pub fn offset_mut(&mut self) -> &mut Point {
        &mut self.inner.subtree.subtree.0.offset
    }

    pub fn offset_node_mut(&mut self) -> &mut Offset<T> {
        &mut self.inner.subtree.subtree.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner.subtree.subtree.0.subtree
    }

    pub fn set_bars(&mut self, horizontal: Option<Capsule>, vertical: Option<Capsule>) {
        self.inner.subtree.subtree.1 = horizontal;
        self.inner.subtree.subtree.2 = vertical;
    }
}

impl<T: AnimatedJoin> AnimatedJoin for ScrollMetadata<T> {
    fn join_from(&mut self, source: &Self, domain: &crate::render::AnimationDomain) {
        self.scroll_size = Size::interpolate(source.scroll_size, self.scroll_size, domain.factor);
        self.inner_size = Size::interpolate(source.inner_size, self.inner_size, domain.factor);
        self.inner.join_from(&source.inner, domain);
    }
}

impl<T: Render<C>, C: Copy> Render<C> for ScrollMetadata<T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        render_target.with_layer(
            |l| l.clip(&Rectangle::new(self.inner.offset, self.scroll_size)),
            |target| {
                self.inner.render(target, style);
            },
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        domain: &crate::render::AnimationDomain,
    ) {
        render_target.with_layer(
            |l| l.clip(&Rectangle::new(target.inner.offset, target.scroll_size)),
            |render_target| {
                Render::render_animated(render_target, &source.inner, &target.inner, style, domain);
            },
        );
    }
}
