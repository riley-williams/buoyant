use fixed::traits::ToFixed;
use fixed_macro::fixed;

use crate::primitives::{Interpolate, Point};

pub type ScaleFactor = fixed::types::U18F14;

/// A type which can be transformed from one coordinate space to another.
pub trait CoordinateSpaceTransform {
    /// Converts an object in transform's coordinate space to the global coordinate space.
    #[must_use]
    fn applying(&self, transform: &LinearTransform) -> Self;

    /// Converts an object in global coordinate space to the transform's coordinate space.
    #[must_use]
    fn applying_inverse(&self, transform: &LinearTransform) -> Self;
}

/// A transformation from one coordinate space to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearTransform {
    pub offset: Point,
    pub scale: ScaleFactor,
}

impl LinearTransform {
    /// Creates a new [`LinearTransform`] with the given offset and scale.
    #[must_use]
    pub fn new(offset: Point, scale: impl ToFixed) -> Self {
        Self {
            offset,
            scale: scale.saturating_to_fixed(),
        }
    }

    /// Creates a new [`LinearTransform`] with the given offset and no scaling.
    #[must_use]
    pub const fn new_offset(offset: Point) -> Self {
        Self {
            offset,
            scale: fixed!(1: U18F14),
        }
    }

    #[must_use]
    pub const fn identity() -> Self {
        Self {
            offset: Point::new(0, 0),
            scale: fixed!(1: U18F14),
        }
    }

    /// Applies another transform to this one
    #[must_use]
    pub fn applying(&self, other: &Self) -> Self {
        let offset = Point {
            x: (other.offset.x * self.scale.cast_signed()).to_num::<i32>() + self.offset.x,
            y: (other.offset.y * self.scale.cast_signed()).to_num::<i32>() + self.offset.y,
        };
        Self {
            offset,
            scale: self.scale * other.scale,
        }
    }

    #[must_use]
    pub fn inverse(&self) -> Self {
        let inv_scale = self.scale.recip().cast_signed();
        let offset = Point {
            x: (self.offset.x * inv_scale).to_num::<i32>(),
            y: (self.offset.y * inv_scale).to_num::<i32>(),
        };
        Self {
            offset: -offset,
            scale: inv_scale.to_num(),
        }
    }
}

impl Default for LinearTransform {
    /// Returns an identity transform with no scaling and zero offset.
    fn default() -> Self {
        Self {
            offset: Point::new(0, 0),
            scale: fixed!(1: U18F14),
        }
    }
}

impl From<Point> for LinearTransform {
    /// Converts a `Point` into a `LinearTransform` with the point as the origin and a scale of 1.0.
    fn from(offset: Point) -> Self {
        Self::new_offset(offset)
    }
}

impl Interpolate for LinearTransform {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            offset: Point::interpolate(from.offset, to.offset, amount),
            scale: ScaleFactor::interpolate(from.scale, to.scale, amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Point;

    #[test]
    fn interpolate_transform() {
        let from = LinearTransform::new(Point::new(10, 20), 1.0);
        let to = LinearTransform::new(Point::new(30, 40), 3.0);
        let interpolated = LinearTransform::interpolate(from.clone(), to.clone(), 128);
        assert_eq!(interpolated.offset, Point::new(20, 30));
        assert!(
            interpolated.scale.abs_diff(ScaleFactor::from_num(2.0)) < ScaleFactor::from_num(0.01)
        );
    }

    #[test]
    fn transformed_point() {
        let transform = LinearTransform::new(Point::new(10, 20), 2.0);
        // "local" point in the coordinate space of the transform
        let point = Point::new(20, 30);
        let transformed_point = point.applying(&transform);
        assert_eq!(transformed_point, Point::new(50, 80));
    }

    #[test]
    fn identity_transform() {
        let point = Point::new(5, 10);
        let transform = LinearTransform::default();
        let transformed_point = point.applying(&transform);
        assert_eq!(transformed_point, point);
        let transform = transform.clone().applying(&transform);
        let transformed_point = point.applying(&transform);
        assert_eq!(transformed_point, point);
    }

    #[test]
    fn offset_transform() {
        let point = Point::new(1, 1);
        let transform = LinearTransform::new_offset(Point::new(5, 10));
        let transformed_point = point.applying(&transform);
        assert_eq!(transformed_point, Point::new(6, 11));

        let transform = transform.clone().applying(&transform);
        let transformed_point = point.applying(&transform);
        assert_eq!(transformed_point, Point::new(11, 21));
    }

    #[test]
    fn point_applying_inverse_transform() {
        // This works with these numbers, but there are inherent limitations in precision
        // because of the result of each transform is truncated to an integer. Speed is probably better?
        let point = Point::new(10, 20);
        let transform = LinearTransform::new(Point::new(5, 10), 2.0);
        let transformed_point = point.applying(&transform).applying_inverse(&transform);
        assert_eq!(transformed_point, point);

        let transformed_point = point.applying(&transform).applying(&transform.inverse());
        assert_eq!(transformed_point, point);
    }

    #[test]
    fn transform_with_inverse_cancels() {
        // This works with these numbers, but there are inherent limitations in precision
        // because of the result of each transform is truncated to an integer. Speed is probably better?
        let point = Point::new(128, -12);
        let transform1 = LinearTransform::new(Point::new(20, 0), 2);
        let transform2 = LinearTransform::new(Point::new(120, -12), 0.5);

        let transformed_point = point
            .applying(&transform1)
            .applying(&transform2)
            .applying_inverse(&transform2)
            .applying_inverse(&transform1);
        assert_eq!(transformed_point, point);
    }
}
