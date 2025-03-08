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
}

impl Animation {
    #[must_use]
    pub const fn linear(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::Linear,
        }
    }

    #[must_use]
    pub const fn with_duration(self, duration: Duration) -> Self {
        Self { duration, ..self }
    }
}

impl Curve {
    /// Computes the animation factor for a given time offset.
    #[must_use]
    pub fn factor(&self, time: Duration, duration: Duration) -> u8 {
        match self {
            Curve::Linear => (time.as_millis() * 255)
                .checked_div(duration.as_millis())
                .unwrap_or(255)
                .min(255) as u8,
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

    #[test]
    fn test_animation_factor() {
        let animation = Animation::linear(Duration::from_millis(100));
        assert_eq!(factor(&animation, 0), 0);
        assert_eq!(factor(&animation, 50), 127);
        assert_eq!(factor(&animation, 100), 255);
        assert_eq!(factor(&animation, 150), 255);
        assert_eq!(factor(&animation, 1500), 255);
    }
}
