use crate::{
    primitives::{geometry::Rectangle, Interpolate as _, Point, Size},
    render::{Animate, AnimatedJoin, Capsule, Offset, Render},
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
    fn render(
        &self,
        render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = C>,
        style: &C,
        offset: crate::primitives::Point,
    ) {
        let clip = render_target
            .set_clip_rect(Rectangle::new(offset + self.inner.offset, self.scroll_size));
        self.inner.render(render_target, style, offset);
        let _ = render_target.set_clip_rect(clip);
    }

    fn render_animated(
        render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: crate::primitives::Point,
        domain: &crate::render::AnimationDomain,
    ) {
        let clip = render_target.set_clip_rect(Rectangle::new(
            offset + target.inner.offset,
            target.scroll_size,
        ));
        Render::render_animated(
            render_target,
            &source.inner,
            &target.inner,
            style,
            offset,
            domain,
        );
        let _ = render_target.set_clip_rect(clip);
    }
}
