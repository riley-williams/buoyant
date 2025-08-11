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
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.subtree.join_from(&source.subtree, domain);
        self.offset = Point::interpolate(source.offset, self.offset, domain.factor);
    }
}

impl<T: Render<C>, C> Render<C> for Offset<T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        render_target.with_layer(
            |l| l.offset(self.offset),
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
        let offset = Point::interpolate(source.offset, target.offset, domain.factor);
        render_target.with_layer(
            |l| l.offset(offset),
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
        let mut target = Offset::new(Point::new(100, 50), ());

        target.join_from(&source, &animation_domain(0));

        // At factor 0, should have source's offset
        assert_eq!(target.offset, source.offset);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Offset::new(Point::new(0, 0), ());
        let original_target = Offset::new(Point::new(100, 50), ());
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        // At factor 255, should have target's offset
        assert_eq!(target.offset, original_target.offset);
    }

    #[test]
    fn animated_join_interpolates_offset() {
        let source = Offset::new(Point::new(0, 0), ());
        let original_target = Offset::new(Point::new(100, 50), ());
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(128));

        // At factor 128 (~50%), offset should be interpolated
        assert!(target.offset.x > source.offset.x && target.offset.x < original_target.offset.x);
        assert!(target.offset.y > source.offset.y && target.offset.y < original_target.offset.y);
    }
}
