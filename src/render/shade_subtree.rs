use crate::{
    primitives::{Interpolate, Point},
    render::{AnimationDomain, Render, RenderTarget},
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadeSubtree<C, T> {
    pub style: C,
    pub subtree: T,
}

impl<C, T> ShadeSubtree<C, T> {
    pub const fn new(style: C, subtree: T) -> Self {
        Self { style, subtree }
    }
}

impl<C: Clone + Interpolate, T: AnimatedJoin> AnimatedJoin for ShadeSubtree<C, T> {
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        Self {
            style: Interpolate::interpolate(source.style, target.style, config.factor),
            subtree: T::join(source.subtree, target.subtree, config),
        }
    }
}

impl<C: Interpolate, T: Render<C>> Render<C> for ShadeSubtree<C, T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, _: &C, offset: Point) {
        self.subtree.render(render_target, &self.style, offset);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        _: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        let style = Interpolate::interpolate(source.style, target.style, domain.factor);
        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            &style,
            offset,
            domain,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Point;
    use crate::render::Circle;
    use core::time::Duration;

    fn animation_domain(factor: u8) -> AnimationDomain {
        AnimationDomain::new(factor, Duration::from_millis(100))
    }

    // Test color type that implements Interpolate
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestColor(u16);

    impl Interpolate for TestColor {
        fn interpolate(source: Self, target: Self, factor: u8) -> Self {
            Self(u16::interpolate(source.0, target.0, factor))
        }
    }

    #[test]
    fn animated_join_at_start() {
        let circle = Circle {
            origin: Point::new(0, 0),
            diameter: 10,
        };
        let target_circle = Circle {
            origin: Point::new(20, 20),
            diameter: 30,
        };

        let source = ShadeSubtree::new(TestColor(50), circle);
        let target = ShadeSubtree::new(TestColor(150), target_circle);

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(0));

        assert_eq!(result.style, source.style);
        assert_eq!(result.subtree, source.subtree);
    }

    #[test]
    fn animated_join_at_end() {
        let circle = Circle {
            origin: Point::new(0, 0),
            diameter: 10,
        };
        let target_circle = Circle {
            origin: Point::new(20, 20),
            diameter: 30,
        };

        let source = ShadeSubtree::new(TestColor(50), circle);
        let target = ShadeSubtree::new(TestColor(150), target_circle);

        let result = AnimatedJoin::join(source.clone(), target.clone(), &animation_domain(255));

        assert_eq!(result.style, target.style);
        assert_eq!(result.subtree, target.subtree);
    }
}
