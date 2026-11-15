use fixed::{traits::ToFixed, types::I18F14};
use fixed_macro::fixed;

use crate::primitives::Interpolate;

type FixedPoint = I18F14;

const DEGREES_PER_RADIAN: FixedPoint = fixed!(57.295_779_513: I18F14);

/// A geometric angle, stored in degrees.
///
/// Angles are measured from the positive x-axis (3 o'clock), increasing clockwise
/// on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Angle {
    degrees: FixedPoint,
}

impl Angle {
    /// Creates an angle from a value in degrees.
    #[must_use]
    pub fn degrees(degrees: impl ToFixed) -> Self {
        Self {
            degrees: degrees.saturating_to_fixed(),
        }
    }

    /// Creates an angle from a value in radians.
    #[must_use]
    pub fn radians(radians: impl ToFixed) -> Self {
        Self {
            degrees: radians
                .saturating_to_fixed::<FixedPoint>()
                .saturating_mul(DEGREES_PER_RADIAN),
        }
    }

    /// An angle of zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            degrees: fixed!(0: I18F14),
        }
    }

    /// Returns the angle in degrees.
    #[must_use]
    pub fn to_degrees(self) -> f32 {
        self.degrees.to_num()
    }

    /// Returns the angle in radians.
    #[must_use]
    pub fn to_radians(self) -> f32 {
        self.to_degrees().to_radians()
    }
}

impl Interpolate for Angle {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            degrees: FixedPoint::interpolate(from.degrees, to.degrees, amount),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Angle> for embedded_graphics::geometry::Angle {
    fn from(value: Angle) -> Self {
        Self::from_degrees(value.to_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degrees_roundtrip() {
        assert_eq!(Angle::degrees(90).to_degrees(), 90.0);
        assert_eq!(Angle::degrees(-45.5).to_degrees(), -45.5);
    }

    #[test]
    fn radians_converts_to_degrees() {
        let angle = Angle::radians(core::f32::consts::PI);
        assert!((angle.to_degrees() - 180.0).abs() < 0.01);
        assert!((angle.to_radians() - core::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn zero_is_default() {
        assert_eq!(Angle::zero(), Angle::default());
        assert_eq!(Angle::zero(), Angle::degrees(0));
    }

    #[test]
    fn out_of_range_saturates() {
        assert_eq!(Angle::degrees(i64::MAX).degrees, FixedPoint::MAX);
        assert_eq!(Angle::radians(i64::MIN).degrees, FixedPoint::MIN);
    }

    #[test]
    fn interpolates_linearly() {
        let from = Angle::degrees(0);
        let to = Angle::degrees(180);
        assert_eq!(Angle::interpolate(from, to, 0), from);
        assert_eq!(Angle::interpolate(from, to, 255), to);
        assert!((Angle::interpolate(from, to, 128).to_degrees() - 90.35).abs() < 0.01);
    }

    #[test]
    fn interpolation_takes_the_long_way_round() {
        // Angles are interpolated as plain numbers, so 350° -> 10° sweeps back through 180°
        let midpoint = Angle::interpolate(Angle::degrees(350), Angle::degrees(10), 128);
        assert!((midpoint.to_degrees() - 180.0).abs() < 1.0);
    }
}
