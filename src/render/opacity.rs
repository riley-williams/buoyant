use crate::{
    primitives::Interpolate,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::RenderTarget,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opacity<T> {
    pub subtree: T,
    pub opacity: u8,
}

impl<T> Opacity<T> {
    pub const fn new(subtree: T, opacity: u8) -> Self {
        Self { subtree, opacity }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for Opacity<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.subtree.join_from(&source.subtree, domain);
        self.opacity = Interpolate::interpolate(source.opacity, self.opacity, domain.factor);
    }
}

impl<T: Render<C>, C: Interpolate + Copy> Render<C> for Opacity<T> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        if self.opacity == 0 {
            return;
        }
        render_target.with_layer(
            |l| l.opacity(self.opacity),
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
        let opacity = Interpolate::interpolate(source.opacity, target.opacity, domain.factor);
        if opacity == 0 {
            return;
        }
        render_target.with_layer(
            |l| l.opacity(opacity),
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
        let source = Opacity::new((), 0);
        let mut target = Opacity::new((), 200);

        target.join_from(&source, &animation_domain(0));

        // At factor 0, should have source's opacity
        assert_eq!(target.opacity, source.opacity);
    }

    #[test]
    fn animated_join_at_end() {
        let source = Opacity::new((), 0);
        let original_target = Opacity::new((), 200);
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(255));

        // At factor 255, should have target's opacity
        assert_eq!(target.opacity, original_target.opacity);
    }

    #[test]
    fn animated_join_interpolates_opacity() {
        let source = Opacity::new((), 50);
        let original_target = Opacity::new((), 200);
        let mut target = original_target.clone();

        target.join_from(&source, &animation_domain(128));

        assert_eq!(
            target.opacity,
            Interpolate::interpolate(source.opacity, original_target.opacity, 128)
        );
    }
}
