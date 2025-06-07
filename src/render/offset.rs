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

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    #[test]
    fn animated_join_at_start() {
        let source = Offset::new(Point::new(0, 0), ());
        let target = Offset::new(Point::new(100, 50), ());

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        // At factor 0, should have source's offset
        assert_eq!(result.offset, source.offset);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Offset::new(Point::new(0, 0), ());
        let target = Offset::new(Point::new(100, 50), ());

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        // At factor 255, should have target's offset
        assert_eq!(result.offset, target.offset);
    }

    #[test]
    fn animated_join_interpolates_offset() {
        let source = Offset::new(Point::new(0, 0), ());
        let target = Offset::new(Point::new(100, 50), ());

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(128));

        // At factor 128 (~50%), offset should be interpolated
        assert!(result.offset.x > source.offset.x && result.offset.x < target.offset.x);
        assert!(result.offset.y > source.offset.y && result.offset.y < target.offset.y);
    }
}
