use core::time::Duration;

use crate::{
    animation::Animation,
    render::{AnimationDomain, Render},
    render_target::RenderTarget,
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Animate<T, U: Clone> {
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

impl<T, U: PartialEq + Clone> Animate<T, U> {
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

impl<T: AnimatedJoin, U: PartialEq + Clone> AnimatedJoin for Animate<T, U> {
    #[expect(clippy::useless_let_if_seq)]
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        let (end_time, duration) = if source.value != self.value {
            let duration = self.animation.duration;
            (self.frame_time + duration, duration)
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

        self.subtree.join_from(&source.subtree, &subdomain);
        self.animation = self.animation.clone().with_duration(new_duration);
        self.frame_time = domain.app_time;
        self.is_partial = is_partial;
    }
}

impl<C, T: Render<C>, U: PartialEq + Clone> Render<C> for Animate<T, U> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        self.subtree.render(render_target, style);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
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
            render_target.report_active_animation();
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
            &subdomain,
        );
    }
}
