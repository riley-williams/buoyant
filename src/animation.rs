use core::time::Duration;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animation {
    pub duration: Duration,
    pub curve: Curve,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Curve {
    Linear,
    /// Quadratic ease in
    EaseIn,
    /// Quadratic ease out
    EaseOut,
    /// Quadratic ease in and out
    EaseInOut,
    /// Ease out with a bounce effect
    EaseOutBounce,
}

impl Animation {
    /// Constructs a new animation with a linear curve.
    #[must_use]
    pub const fn linear(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::Linear,
        }
    }

    /// Constructs a new animation with a quadratic ease-in curve.
    ///
    /// The animation will start slow and speed up.
    #[must_use]
    pub const fn ease_in(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::EaseIn,
        }
    }

    /// Constructs a new animation with a quadratic ease-out curve.
    ///
    /// The animation will start fast and slow down.
    #[must_use]
    pub const fn ease_out(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::EaseOut,
        }
    }

    /// Constructs a new animation with a quadratic ease-in and ease-out curve.
    ///
    /// The animation will begin and end slowly, with a fast middle section.
    #[must_use]
    pub const fn ease_in_out(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::EaseInOut,
        }
    }

    /// Constructs a new animation with a bouncy ease-out curve.
    ///
    /// The animation will bounce at the end, staying within the bounds of the start and end points.
    #[must_use]
    pub const fn ease_out_bounce(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::EaseOutBounce,
        }
    }

    #[must_use]
    pub const fn with_duration(self, duration: Duration) -> Self {
        Self { duration, ..self }
    }
}

impl Curve {
    /// Computes the animation factor for a given time offset.
    ///
    /// This calculation is expected to occur only once per animation node
    /// in the view rendering, and thus has considerable compute headroom
    #[must_use]
    pub fn factor(&self, time: Duration, duration: Duration) -> u8 {
        match self {
            Self::Linear => (time.as_millis() * 255)
                .checked_div(duration.as_millis())
                .unwrap_or(255)
                .min(255) as u8,
            Self::EaseIn => {
                let x = (time.as_millis() * 256)
                    .checked_div(duration.as_millis())
                    .unwrap_or(255) as u64;
                let x_2 = (x * x) >> 8;
                x_2.min(255) as u8
            }
            Self::EaseOut => {
                let duration_ms = duration.as_millis();
                let x = (duration_ms.saturating_sub(time.as_millis()) * 256)
                    .checked_div(duration_ms)
                    .unwrap_or(255) as u64;
                let x_2 = (x * x) >> 8;
                255u8 - x_2.min(255) as u8
            }
            Self::EaseInOut => {
                let x = (time.as_millis() * 256)
                    .checked_div(duration.as_millis())
                    .unwrap_or(255) as i64;
                if x < 128 {
                    ((x * x) >> 7).min(255).try_into().unwrap_or(255)
                } else {
                    (255 - (((256 - x) * (256 - x)) >> 7))
                        .min(255)
                        .try_into()
                        .unwrap_or(255)
                }
            }
            Self::EaseOutBounce => {
                // Use 1024-scale (4x) for better precision
                let x = (time.as_millis() * 1024)
                    .checked_div(duration.as_millis())
                    .unwrap_or(1024)
                    .min(1024) as i32;

                // n1 = 7.5625, d1 = 2.75
                // Boundaries scaled to 1024
                // 1/d1 = 1/2.75 = 0.364 -> 372
                // 2/d1 = 2/2.75 = 0.727 -> 745
                // 2.5/d1 = 2.5/2.75 = 0.909 -> 931

                let result = if x < 372 {
                    // x < 1/d1
                    // n1 * x * x where n1 = 7.5625
                    // n1 in 1024-scale: 7.5625 * 1024 = 7744
                    let x_sq = (x * x) >> 10; // x^2 / 1024 to normalize
                    ((7744 * x_sq) >> 10) >> 2 // Apply n1, then convert from 1024 to 256 scale
                } else if x < 745 {
                    // x < 2/d1
                    // n1 * (x - 1.5/d1)^2 + 0.75
                    let offset = 559; // 1.5/d1 * 1024 = 559
                    let adjusted_x = x - offset;
                    let x_sq = (adjusted_x * adjusted_x) >> 10;
                    192 + (((7744 * x_sq) >> 10) >> 2) // 0.75 * 256 = 192
                } else if x < 931 {
                    // x < 2.5/d1
                    // n1 * (x - 2.25/d1)^2 + 0.9375
                    let offset = 838; // 2.25/d1 * 1024 = 838
                    let adjusted_x = x - offset;
                    let x_sq = (adjusted_x * adjusted_x) >> 10;
                    240 + (((7744 * x_sq) >> 10) >> 2) // 0.9375 * 256 = 240
                } else {
                    // n1 * (x - 2.625/d1)^2 + 0.984375
                    let offset = 977; // 2.625/d1 * 1024 = 977
                    let adjusted_x = x - offset;
                    let x_sq = (adjusted_x * adjusted_x) >> 10;
                    252 + (((7744 * x_sq) >> 10) >> 2) // 0.984375 * 256 = 252
                };

                result.clamp(0, 255) as u8
            }
        }
    }

    /// Computes the animation factor for a given time offset.
    ///
    /// This calculation is expected to occur only once per animation node
    /// in the view rendering, and thus has considerable compute headroom.
    #[must_use]
    #[allow(dead_code)]
    fn factor_f32(self, time: Duration, duration: Duration) -> f32 {
        match self {
            Self::Linear => time.as_secs_f32() / duration.as_secs_f32(),
            Self::EaseIn => {
                let x = time.as_secs_f32() / duration.as_secs_f32();
                x * x
            }
            Self::EaseOut => {
                let x = time.as_secs_f32() / duration.as_secs_f32();
                1.0 - (1.0 - x) * (1.0 - x)
            }
            Self::EaseInOut => {
                let x = time.as_secs_f32() / duration.as_secs_f32();
                if x < 0.5 {
                    2.0 * x * x
                } else {
                    let y = -2.0 * x + 2.0;
                    1.0 - y * y / 2.0
                }
            }
            Self::EaseOutBounce => {
                let x = time.as_secs_f32() / duration.as_secs_f32();
                let n1 = 7.5625;
                let d1 = 2.75;

                if x < 1.0 / d1 {
                    n1 * x * x
                } else if x < 2.0 / d1 {
                    let adjusted_x = x - 1.5 / d1;
                    n1 * adjusted_x * adjusted_x + 0.75
                } else if x < 2.5 / d1 {
                    let adjusted_x = x - 2.25 / d1;
                    n1 * adjusted_x * adjusted_x + 0.9375
                } else {
                    let adjusted_x = x - 2.625 / d1;
                    n1 * adjusted_x * adjusted_x + 0.984_375
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::*;

    fn factor(animation: &Animation, time: u64) -> u8 {
        animation
            .curve
            .factor(Duration::from_millis(time), animation.duration)
    }

    #[expect(clippy::cast_sign_loss)]
    fn factor_f32(animation: &Animation, time: u64) -> u8 {
        (animation
            .curve
            .factor_f32(Duration::from_millis(time), animation.duration)
            * 255.0)
            .clamp(0.0, 255.0) as u8
    }

    #[test]
    fn linear_factor_approximates_f32() {
        let animation = Animation::linear(Duration::from_millis(500));
        for time in 0..512 {
            let f32_factor = factor_f32(&animation, time);
            let u8_factor = factor(&animation, time);
            assert!(f32_factor.abs_diff(u8_factor) <= 1);
        }
    }

    #[test]
    fn ease_in_factor_approximates_f32() {
        let animation = Animation::ease_in(Duration::from_millis(500));
        for time in 0..512 {
            let f32_factor = factor_f32(&animation, time);
            let u8_factor = factor(&animation, time);
            assert!(f32_factor.abs_diff(u8_factor) <= 1);
        }
    }

    #[test]
    fn ease_out_factor_approximates_f32() {
        let animation = Animation::ease_out(Duration::from_millis(500));
        for time in 0..512 {
            let f32_factor = factor_f32(&animation, time);
            let u8_factor = factor(&animation, time);
            assert!(f32_factor.abs_diff(u8_factor) <= 2);
        }
    }

    #[test]
    fn ease_in_out_factor_approximates_f32() {
        let animation = Animation::ease_in_out(Duration::from_millis(500));
        for time in 0..512 {
            let f32_factor = factor_f32(&animation, time);
            let u8_factor = factor(&animation, time);
            assert!(f32_factor.abs_diff(u8_factor) <= 2);
        }
    }

    #[test]
    fn linear_animation_factor_bounds() {
        let animation = Animation::linear(Duration::from_millis(100));
        assert_eq!(factor(&animation, 0), 0);
        assert_eq!(factor(&animation, 50), 127);
        assert_eq!(factor(&animation, 100), 255);
        assert_eq!(factor(&animation, 101), 255);
        assert_eq!(factor(&animation, 1500), 255);
    }

    #[test]
    fn ease_in_animation_factor_bounds() {
        let animation = Animation::ease_in(Duration::from_millis(100));
        assert_eq!(factor(&animation, 0), 0);
        assert_eq!(factor(&animation, 100), 255);
        assert_eq!(factor(&animation, 101), 255);
        assert_eq!(factor(&animation, 1500), 255);
    }

    #[test]
    fn ease_out_animation_factor_bounds() {
        let animation = Animation::ease_out(Duration::from_millis(100));
        assert_eq!(factor(&animation, 0), 0);
        assert_eq!(factor(&animation, 100), 255);
        assert_eq!(factor(&animation, 101), 255);
        assert_eq!(factor(&animation, 1500), 255);
    }

    #[test]
    fn ease_out_bounce_factor_approximates_f32() {
        let animation = Animation::ease_out_bounce(Duration::from_millis(500));
        for time in 0..512 {
            let f32_factor = factor_f32(&animation, time);
            let u8_factor = factor(&animation, time);
            assert!(
                f32_factor.abs_diff(u8_factor) <= 3,
                "Expected {u8_factor} to be nearly {f32_factor} at t={time}"
            );
        }
    }

    #[test]
    fn ease_out_bounce_animation_factor_bounds() {
        let animation = Animation::ease_out_bounce(Duration::from_millis(100));
        assert_eq!(factor(&animation, 0), 0);
        assert_eq!(factor(&animation, 100), 255);
        assert_eq!(factor(&animation, 101), 255);
        assert_eq!(factor(&animation, 1500), 255);
    }
}
