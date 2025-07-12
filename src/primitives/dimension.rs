use crate::primitives::Interpolate;

use super::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposedDimensions {
    pub width: ProposedDimension,
    pub height: ProposedDimension,
}

impl ProposedDimensions {
    #[must_use]
    pub fn new(width: impl Into<ProposedDimension>, height: impl Into<ProposedDimension>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }

    /// A proposal with compact width and height
    #[must_use]
    pub const fn compact() -> Self {
        Self {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Compact,
        }
    }

    /// A proposal with infinite width and height
    #[must_use]
    pub const fn infinite() -> Self {
        Self {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Infinite,
        }
    }

    #[must_use]
    pub fn resolve_most_flexible(self, minimum: u32, ideal: u32) -> Dimensions {
        Dimensions {
            width: self.width.resolve_most_flexible(minimum, ideal),
            height: self.height.resolve_most_flexible(minimum, ideal),
        }
    }

    /// Determines if a given size fits within the proposal along the given axes
    ///
    /// Compact and infinite proposals always return true for any size
    #[must_use]
    pub const fn contains(&self, size: Dimensions, horizontal: bool, vertical: bool) -> bool {
        (!horizontal
            || match self.width {
                ProposedDimension::Exact(d) => size.width.0 <= d,
                ProposedDimension::Compact | ProposedDimension::Infinite => true,
            })
            && (!vertical
                || match self.height {
                    ProposedDimension::Exact(d) => size.height.0 <= d,
                    ProposedDimension::Compact | ProposedDimension::Infinite => true,
                })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProposedDimension {
    /// An exactly sized offer
    Exact(u32),
    /// A request for the most compact size a view can manage
    Compact,
    /// An offer of infinite size
    Infinite,
}

impl From<Size> for ProposedDimensions {
    fn from(size: Size) -> Self {
        Self {
            width: ProposedDimension::Exact(size.width),
            height: ProposedDimension::Exact(size.height),
        }
    }
}

impl From<Dimensions> for ProposedDimensions {
    fn from(dimensions: Dimensions) -> Self {
        Self {
            width: ProposedDimension::Exact(dimensions.width.0),
            height: ProposedDimension::Exact(dimensions.height.0),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Size> for ProposedDimensions {
    fn from(size: embedded_graphics_core::geometry::Size) -> Self {
        Self {
            width: ProposedDimension::Exact(size.width),
            height: ProposedDimension::Exact(size.height),
        }
    }
}

impl ProposedDimension {
    /// Returns the most flexible dimension within the proposal
    /// Magic size of 10 points is applied to views that have no implicit size
    #[must_use]
    pub fn resolve_most_flexible(self, minimum: u32, ideal: u32) -> Dimension {
        match self {
            Self::Compact => Dimension(ideal),
            Self::Exact(d) => Dimension(d.max(minimum)),
            Self::Infinite => Dimension::infinite(),
        }
    }
}

impl From<u32> for ProposedDimension {
    fn from(value: u32) -> Self {
        Self::Exact(value)
    }
}

impl core::ops::Add<u32> for ProposedDimension {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Self::Compact => Self::Compact,
            Self::Exact(d) => Self::Exact(d + rhs),
            Self::Infinite => Self::Infinite,
        }
    }
}

impl core::ops::Sub<u32> for ProposedDimension {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        match self {
            Self::Compact => Self::Compact,
            Self::Exact(d) => Self::Exact(d.saturating_sub(rhs)),
            Self::Infinite => Self::Infinite,
        }
    }
}

impl core::ops::Mul<u32> for ProposedDimension {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        match self {
            Self::Compact => Self::Compact,
            Self::Exact(d) => Self::Exact(d.saturating_mul(rhs)),
            Self::Infinite => Self::Infinite,
        }
    }
}

impl core::ops::Div<u32> for ProposedDimension {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        match self {
            Self::Compact => Self::Compact,
            Self::Exact(d) => Self::Exact(d.saturating_div(rhs)),
            Self::Infinite => Self::Infinite,
        }
    }
}

/// The dimension of a single axis
/// `u16::MAX` is treated as infinity, and this type mostly exists to prevent accidental panics from
/// operations overflowing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension(pub u32);

impl Dimension {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn infinite() -> Self {
        Self(u32::MAX)
    }

    #[must_use]
    pub const fn is_infinite(self) -> bool {
        self.0 == u32::MAX
    }
}

impl From<Dimension> for u32 {
    fn from(value: Dimension) -> Self {
        value.0
    }
}

impl From<Dimension> for i32 {
    fn from(value: Dimension) -> Self {
        value.0 as Self
    }
}

impl From<Dimension> for f32 {
    #[expect(clippy::cast_precision_loss)]
    fn from(value: Dimension) -> Self {
        value.0 as Self
    }
}

impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u32> for Dimension {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for Dimension {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl core::ops::Add for Dimension {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl core::ops::Sub for Dimension {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl core::ops::Mul for Dimension {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_mul(rhs.0))
    }
}

impl core::ops::Div for Dimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_div(rhs.0))
    }
}

impl core::ops::AddAssign for Dimension {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign for Dimension {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl core::ops::Add<u16> for Dimension {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_add(rhs.into()))
    }
}

impl core::ops::Sub<u16> for Dimension {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_sub(rhs.into()))
    }
}

impl core::ops::Mul<u16> for Dimension {
    type Output = Self;
    fn mul(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_mul(rhs.into()))
    }
}

impl core::ops::Div<u16> for Dimension {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_div(rhs.into()))
    }
}

impl core::ops::AddAssign<u16> for Dimension {
    fn add_assign(&mut self, rhs: u16) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign<u16> for Dimension {
    fn sub_assign(&mut self, rhs: u16) {
        *self = *self - rhs;
    }
}

impl core::ops::Add<u32> for Dimension {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0.saturating_add(rhs))
    }
}

impl core::ops::Sub<u32> for Dimension {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Self(self.0.saturating_sub(rhs))
    }
}

impl core::ops::Mul<u32> for Dimension {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0.saturating_mul(rhs))
    }
}

impl core::ops::Div<u32> for Dimension {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self(self.0.saturating_div(rhs))
    }
}

impl core::ops::AddAssign<u32> for Dimension {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign<u32> for Dimension {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: Dimension,
    pub height: Dimension,
}

impl Dimensions {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width: Dimension(width),
            height: Dimension(height),
        }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self {
            width: Dimension(0),
            height: Dimension(0),
        }
    }

    #[must_use]
    pub const fn infinite() -> Self {
        Self {
            width: Dimension::infinite(),
            height: Dimension::infinite(),
        }
    }

    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    #[must_use]
    pub fn intersection(self, other: Self) -> Self {
        Self {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    #[must_use]
    pub fn intersecting_proposal(self, offer: &ProposedDimensions) -> Self {
        Self {
            width: match offer.width {
                ProposedDimension::Exact(d) => Dimension(self.width.0.min(d)),
                ProposedDimension::Infinite | ProposedDimension::Compact => self.width,
            },
            height: match offer.height {
                ProposedDimension::Exact(d) => Dimension(self.height.0.min(d)),
                ProposedDimension::Infinite | ProposedDimension::Compact => self.height,
            },
        }
    }

    #[must_use]
    pub fn area(self) -> u32 {
        (self.width * self.height).0
    }
}

impl core::ops::Add for Dimensions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl core::ops::Add<Size> for Dimensions {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl From<Size> for Dimensions {
    fn from(value: Size) -> Self {
        Self {
            width: Dimension(value.width),
            height: Dimension(value.height),
        }
    }
}

impl From<Dimensions> for Size {
    fn from(value: Dimensions) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Size> for Dimensions {
    fn from(value: embedded_graphics_core::geometry::Size) -> Self {
        Self {
            width: Dimension(value.width),
            height: Dimension(value.height),
        }
    }
}

impl Interpolate for Dimensions {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self {
            width: Dimension::interpolate(from.width, to.width, amount),
            height: Dimension::interpolate(from.height, to.height, amount),
        }
    }
}

impl Interpolate for Dimension {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Self(((u32::from(amount) * to.0) + (u32::from(255 - amount) * from.0)) / 255)
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Dimensions> for embedded_graphics_core::geometry::Size {
    fn from(value: Dimensions) -> Self {
        Self::new(value.width.0, value.height.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Interpolate as _;

    use super::{Dimension, Dimensions, ProposedDimension, ProposedDimensions};

    #[test]
    fn interpolate_dimensions() {
        let from = Dimensions::new(10, 20);
        let to = Dimensions::new(20, 30);
        let result = Dimensions::interpolate(from, to, 128);
        assert_eq!(result.width.0, 15);
        assert_eq!(result.height.0, 25);
    }

    #[test]
    fn interpolate_dimension_min() {
        let from = Dimension::new(10);
        let to = Dimension::new(30);
        let result = Dimension::interpolate(from, to, 0);
        assert_eq!(result.0, 10);
    }

    #[test]
    fn interpolate_dimension_max() {
        let from = Dimension::new(10);
        let to = Dimension::new(30);
        let result = Dimension::interpolate(from, to, 255);
        assert_eq!(result.0, 30);
    }

    #[test]
    fn proposed_dimension_order() {
        assert_eq!(ProposedDimension::Compact, ProposedDimension::Compact);
        assert_eq!(ProposedDimension::Exact(0), ProposedDimension::Exact(0));
        assert_eq!(ProposedDimension::Exact(10), ProposedDimension::Exact(10));
        assert_eq!(ProposedDimension::Infinite, ProposedDimension::Infinite);
        assert!(ProposedDimension::Compact > ProposedDimension::Exact(0));
        assert!(ProposedDimension::Compact > ProposedDimension::Exact(100));
        assert!(ProposedDimension::Compact < ProposedDimension::Infinite);
        assert!(ProposedDimension::Exact(0) < ProposedDimension::Infinite);
        assert!(ProposedDimension::Exact(u32::MAX) < ProposedDimension::Infinite);
    }

    #[test]
    fn exact_proposed_dimension_contains() {
        let proposal = ProposedDimensions {
            width: ProposedDimension::Exact(10),
            height: ProposedDimension::Exact(20),
        };

        let smaller_size = Dimensions::new(5, 10);
        let equal_size = Dimensions::new(10, 20);
        let larger_size = Dimensions::new(15, 30);
        assert!(proposal.contains(smaller_size, true, true));
        assert!(proposal.contains(equal_size, true, true));
        assert!(!proposal.contains(larger_size, true, true));
        assert!(proposal.contains(Dimensions::new(1, 30), true, false));
        assert!(proposal.contains(Dimensions::new(100, 3), false, true));
        assert!(proposal.contains(Dimensions::new(100, 100), false, false));
    }

    #[test]
    fn proposed_dimension_contains_compact() {
        let proposal = ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Compact,
        };

        let smaller_size = Dimensions::new(5, 10);
        let equal_size = Dimensions::new(10, 20);
        let infinite_size = Dimensions {
            width: Dimension::infinite(),
            height: Dimension::infinite(),
        };
        assert!(proposal.contains(smaller_size, true, true));
        assert!(proposal.contains(equal_size, true, true));
        assert!(proposal.contains(infinite_size, true, true));
    }

    #[test]
    fn proposed_dimension_contains_infinite() {
        let proposal = ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: ProposedDimension::Infinite,
        };

        let fixed_size = Dimensions::new(5, 10);
        let infinite_size = Dimensions {
            width: Dimension::infinite(),
            height: Dimension::infinite(),
        };
        assert!(proposal.contains(fixed_size, true, true));
        assert!(proposal.contains(infinite_size, true, true));
    }
}
