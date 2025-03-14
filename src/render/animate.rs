use core::time::Duration;

use crate::{
    animation::Animation,
    primitives::Point,
    render::{AnimationDomain, CharacterRender},
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Animate<T, U> {
    pub subtree: T,
    /// Length of the animation
    pub animation: Animation,
    /// The time at which this frame was generated
    pub frame_time: Duration,
    pub value: U,
    /// This is true if the animation is the result of a partially-completed join operation.
    /// If this is true, the source animation / duration will be used
    /// if the values are equal to avoid animations cancelling.
    pub is_partial: bool,
}

impl<T, U: PartialEq> Animate<T, U> {
    #[must_use]
    pub const fn new(subtree: T, animation: Animation, frame_time: Duration, value: U) -> Self {
        Self {
            subtree,
            animation,
            frame_time,
            value,
            is_partial: false,
        }
    }
}

impl<T: AnimatedJoin, U: PartialEq> AnimatedJoin for Animate<T, U> {
    #[expect(clippy::useless_let_if_seq)]
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let (end_time, duration) = if source.value != target.value {
            let duration = target.animation.duration;
            (target.frame_time + duration, duration)
        } else if source.is_partial {
            // continue source animation
            let duration = source.animation.duration;
            (source.frame_time + duration, duration)
        } else {
            // no animation
            (domain.app_time, Duration::from_secs(0))
        };

        let new_duration;
        let is_partial;
        let subdomain;
        if end_time == Duration::from_secs(0) || domain.app_time >= end_time {
            // animation has already completed or there was zero duration
            is_partial = false;
            new_duration = Duration::from_secs(0);
            subdomain = AnimationDomain {
                factor: 255,
                app_time: domain.app_time,
            };
        } else {
            is_partial = true;
            new_duration = end_time.saturating_sub(domain.app_time);
            // compute factor
            let diff = duration.saturating_sub(end_time.saturating_sub(domain.app_time));
            let factor = source.animation.curve.factor(diff, duration);
            subdomain = AnimationDomain {
                factor,
                app_time: domain.app_time,
            };
        }

        Self {
            animation: target.animation.with_duration(new_duration),
            subtree: T::join(source.subtree, target.subtree, &subdomain),
            frame_time: domain.app_time,
            value: target.value,
            is_partial,
        }
    }
}

impl<C, T: CharacterRender<C>, U: PartialEq> CharacterRender<C> for Animate<T, U> {
    fn render(
        &self,
        render_target: &mut impl crate::render::CharacterRenderTarget<Color = C>,
        style: &C,
        offset: Point,
    ) {
        self.subtree.render(render_target, style, offset);
    }

    fn render_animated(
        render_target: &mut impl crate::render::CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        let (end_time, duration) = if source.value != target.value {
            let duration = target.animation.duration;
            (target.frame_time + duration, duration)
        } else if source.is_partial {
            // continue source animation
            let duration = source.animation.duration;
            (source.frame_time + duration, duration)
        } else {
            // no animation
            (domain.app_time, Duration::from_secs(0))
        };

        let subdomain = if end_time == Duration::from_secs(0) || domain.app_time >= end_time {
            // animation has already completed or there was zero duration
            AnimationDomain {
                factor: 255,
                app_time: domain.app_time,
            }
        } else {
            // compute factor
            let diff = duration.saturating_sub(end_time.saturating_sub(domain.app_time));
            let factor = source.animation.curve.factor(diff, duration);
            AnimationDomain {
                factor,
                app_time: domain.app_time,
            }
        };

        T::render_animated(
            render_target,
            &source.subtree,
            &target.subtree,
            style,
            offset,
            &subdomain,
        );
    }
}

// TODO: This implementation should always be exactly the same as the character render implementation.

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use core::time::Duration;

    use crate::{
        primitives::Point,
        render::{AnimationDomain, EmbeddedGraphicsRender},
    };

    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics_core::draw_target::DrawTarget;

    use super::Animate;

    impl<C: PixelColor, T: EmbeddedGraphicsRender<C>, U: PartialEq> EmbeddedGraphicsRender<C>
        for Animate<T, U>
    {
        fn render(&self, render_target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            self.subtree.render(render_target, style, offset);
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            let (end_time, duration) = if source.value != target.value {
                let duration = target.animation.duration;
                (target.frame_time + duration, duration)
            } else if source.is_partial {
                // continue source animation
                let duration = source.animation.duration;
                (source.frame_time + duration, duration)
            } else {
                // no animation
                (domain.app_time, Duration::from_secs(0))
            };

            let subdomain = if end_time == Duration::from_secs(0) || domain.app_time >= end_time {
                // animation has already completed or there was zero duration
                AnimationDomain {
                    factor: 255,
                    app_time: domain.app_time,
                }
            } else {
                // compute factor
                let diff = duration.saturating_sub(end_time.saturating_sub(domain.app_time));
                let factor = source.animation.curve.factor(diff, duration);
                AnimationDomain {
                    factor,
                    app_time: domain.app_time,
                }
            };

            T::render_animated(
                render_target,
                &source.subtree,
                &target.subtree,
                style,
                offset,
                &subdomain,
            );
        }
    }
}
