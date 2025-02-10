use core::time::Duration;

use crate::{
    primitives::Point,
    render::{AnimationDomain, CharacterRender},
    Animation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animate<T, U> {
    subtree: T,
    /// Length of the animation
    animation: Animation,
    /// The time at which this frame was generated
    frame_time: Duration,
    value: U,
    /// This is true if the animation is the result of a partially-completed join operation.
    /// If this is true, the source animation / duration will be used
    /// if the values are equal to avoid animations cancelling.
    is_partial: bool,
}

impl<T, U: PartialEq + Clone> Animate<T, U> {
    #[must_use]
    pub fn new(subtree: T, animation: Animation, frame_time: Duration, value: U) -> Self {
        Self {
            subtree,
            animation,
            frame_time,
            value,
            is_partial: false,
        }
    }
}

impl<C, T: CharacterRender<C>, U: PartialEq + Clone> CharacterRender<C> for Animate<T, U> {
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
        let should_animate;
        let (end_time, duration) = if source.value != target.value {
            let duration = target.animation.duration();
            should_animate = true;
            (target.frame_time + duration, duration)
        } else if source.is_partial {
            // continue source animation
            let duration = source.animation.duration();
            should_animate = true;
            (source.frame_time + duration, duration)
        } else {
            // no animation
            should_animate = false;
            (domain.app_time, Duration::from_secs(0))
        };

        let subdomain = if !should_animate {
            domain
        } else if end_time == Duration::from_secs(0) || domain.app_time >= end_time {
            // animation has already completed or there was zero duration
            // use the parent domain to animate subtree
            &AnimationDomain {
                factor: 255,
                app_time: domain.app_time,
            }
        } else {
            // compute factor
            let diff = end_time.saturating_sub(domain.app_time);
            let factor = 255u128.saturating_sub(
                (diff.as_millis() * 255)
                    .checked_div(duration.as_millis())
                    .unwrap_or(0),
            ) as u8;
            &AnimationDomain {
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
            subdomain,
        );
    }

    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let should_animate;
        let (end_time, duration) = if source.value != target.value {
            let duration = target.animation.duration();
            should_animate = true;
            (target.frame_time + duration, duration)
        } else if source.is_partial {
            // continue source animation
            let duration = source.animation.duration();
            should_animate = true;
            (source.frame_time + duration, duration)
        } else {
            // no animation
            should_animate = false;
            (domain.app_time, Duration::from_secs(0))
        };

        let new_duration;
        let is_partial;
        let subdomain;
        if !should_animate {
            is_partial = false;
            new_duration = Duration::from_secs(0);
            subdomain = domain.clone();
        } else if duration == Duration::from_secs(0) || domain.app_time >= end_time {
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
            let factor = 255u128.saturating_sub(
                (new_duration.as_millis() * 255)
                    .checked_div(duration.as_millis())
                    .unwrap_or(0),
            ) as u8;
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

    impl<C: PixelColor, T: EmbeddedGraphicsRender<C>, U: PartialEq + Clone>
        EmbeddedGraphicsRender<C> for Animate<T, U>
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
            let should_animate;
            let (end_time, duration) = if source.value != target.value {
                let duration = target.animation.duration();
                should_animate = true;
                (target.frame_time + duration, duration)
            } else if source.is_partial {
                // continue source animation
                let duration = source.animation.duration();
                should_animate = true;
                (source.frame_time + duration, duration)
            } else {
                // no animation
                should_animate = false;
                (domain.app_time, Duration::from_secs(0))
            };

            let subdomain = if !should_animate {
                domain
            } else if end_time == Duration::from_secs(0) || domain.app_time >= end_time {
                // animation has already completed or there was zero duration
                // use the parent domain to animate subtree
                &AnimationDomain {
                    factor: 255,
                    app_time: domain.app_time,
                }
            } else {
                // compute factor
                let diff = end_time.saturating_sub(domain.app_time);
                let factor = 255u128.saturating_sub(
                    (diff.as_millis() * 255)
                        .checked_div(duration.as_millis())
                        .unwrap_or(0),
                ) as u8;
                &AnimationDomain {
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
                subdomain,
            );
        }

        fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
            let should_animate;
            let (end_time, duration) = if source.value != target.value {
                let duration = target.animation.duration();
                should_animate = true;
                (target.frame_time + duration, duration)
            } else if source.is_partial {
                // continue source animation
                let duration = source.animation.duration();
                should_animate = true;
                (source.frame_time + duration, duration)
            } else {
                // no animation
                should_animate = false;
                (domain.app_time, Duration::from_secs(0))
            };

            let new_duration;
            let is_partial;
            let subdomain;
            if !should_animate {
                is_partial = false;
                new_duration = Duration::from_secs(0);
                subdomain = domain.clone();
            } else if duration == Duration::from_secs(0) || domain.app_time >= end_time {
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
                let factor = 255u128.saturating_sub(
                    (new_duration.as_millis() * 255)
                        .checked_div(duration.as_millis())
                        .unwrap_or(0),
                ) as u8;
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
}
