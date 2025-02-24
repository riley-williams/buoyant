use crate::primitives::Interpolate;

use super::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposedDimensions {
    pub width: ProposedDimension,
    pub height: ProposedDimension,
}

impl ProposedDimensions {
    #[must_use]
    pub fn resolve_most_flexible(self, minimum: u16, ideal: u16) -> Dimensions {
        Dimensions {
            width: self.width.resolve_most_flexible(minimum, ideal),
            height: self.height.resolve_most_flexible(minimum, ideal),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProposedDimension {
    /// An exactly sized offer
    Exact(u16),
    /// A request for the most compact size a view can manage
    Compact,
    /// An offer of infinite size
    Infinite,
}

impl From<Size> for ProposedDimensions {
    fn from(size: Size) -> Self {
        ProposedDimensions {
            width: ProposedDimension::Exact(size.width),
            height: ProposedDimension::Exact(size.height),
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<embedded_graphics_core::geometry::Size> for ProposedDimensions {
    fn from(size: embedded_graphics_core::geometry::Size) -> Self {
        ProposedDimensions {
            width: ProposedDimension::Exact(size.width as u16),
            height: ProposedDimension::Exact(size.height as u16),
        }
    }
}

impl ProposedDimension {
    /// Returns the most flexible dimension within the proposal
    /// Magic size of 10 points is applied to views that have no implicit size
    #[must_use]
    pub fn resolve_most_flexible(self, minimum: u16, ideal: u16) -> Dimension {
        match self {
            ProposedDimension::Compact => Dimension(ideal),
            ProposedDimension::Exact(d) => Dimension(d.max(minimum)),
            ProposedDimension::Infinite => Dimension::infinite(),
        }
    }
}

impl From<u16> for ProposedDimension {
    fn from(value: u16) -> Self {
        ProposedDimension::Exact(value)
    }
}

impl core::ops::Add<u16> for ProposedDimension {
    type Output = ProposedDimension;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            ProposedDimension::Compact => ProposedDimension::Compact,
            ProposedDimension::Exact(d) => ProposedDimension::Exact(d + rhs),
            ProposedDimension::Infinite => ProposedDimension::Infinite,
        }
    }
}

impl core::ops::Sub<u16> for ProposedDimension {
    type Output = ProposedDimension;

    fn sub(self, rhs: u16) -> Self::Output {
        match self {
            ProposedDimension::Compact => ProposedDimension::Compact,
            ProposedDimension::Exact(d) => ProposedDimension::Exact(d.saturating_sub(rhs)),
            ProposedDimension::Infinite => ProposedDimension::Infinite,
        }
    }
}

impl core::ops::Mul<u16> for ProposedDimension {
    type Output = ProposedDimension;

    fn mul(self, rhs: u16) -> Self::Output {
        match self {
            ProposedDimension::Compact => ProposedDimension::Compact,
            ProposedDimension::Exact(d) => ProposedDimension::Exact(d.saturating_mul(rhs)),
            ProposedDimension::Infinite => ProposedDimension::Infinite,
        }
    }
}

impl core::ops::Div<u16> for ProposedDimension {
    type Output = ProposedDimension;

    fn div(self, rhs: u16) -> Self::Output {
        match self {
            ProposedDimension::Compact => ProposedDimension::Compact,
            ProposedDimension::Exact(d) => ProposedDimension::Exact(d.saturating_div(rhs)),
            ProposedDimension::Infinite => ProposedDimension::Infinite,
        }
    }
}

/// The dimension of a single axis
/// `u16::MAX` is treated as infinity, and this type mostly exists to prevent accidental panics from
/// operations overflowing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension(pub u16);

impl Dimension {
    #[must_use]
    pub const fn infinite() -> Self {
        Self(u16::MAX)
    }

    #[must_use]
    pub const fn is_infinite(self) -> bool {
        self.0 == u16::MAX
    }
}

impl From<Dimension> for u16 {
    fn from(value: Dimension) -> Self {
        value.0
    }
}

impl From<Dimension> for i16 {
    fn from(value: Dimension) -> Self {
        value.0.try_into().unwrap_or(i16::MAX)
    }
}

impl From<Dimension> for u32 {
    fn from(value: Dimension) -> Self {
        u32::from(value.0)
    }
}

impl From<Dimension> for i32 {
    fn from(value: Dimension) -> Self {
        i32::from(value.0)
    }
}

impl From<Dimension> for f32 {
    fn from(value: Dimension) -> Self {
        f32::from(value.0)
    }
}

impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Self(value)
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
        Self(self.0.saturating_add(rhs))
    }
}

impl core::ops::Sub<u16> for Dimension {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_sub(rhs))
    }
}

impl core::ops::Mul<u16> for Dimension {
    type Output = Self;
    fn mul(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_mul(rhs))
    }
}

impl core::ops::Div<u16> for Dimension {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        Self(self.0.saturating_div(rhs))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: Dimension,
    pub height: Dimension,
}

impl Dimensions {
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
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
    pub fn area(self) -> u16 {
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

impl Interpolate for Dimensions {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Dimensions {
            width: Dimension::interpolate(from.width, to.width, amount),
            height: Dimension::interpolate(from.height, to.height, amount),
        }
    }
}

impl Interpolate for Dimension {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        Dimension(
            (((u32::from(amount) * u32::from(to.0))
                + (u32::from(255 - amount) * u32::from(from.0)))
                / 255) as u16,
        )
    }
}

#[cfg(feature = "embedded-graphics")]
impl From<Dimensions> for embedded_graphics_core::geometry::Size {
    fn from(value: Dimensions) -> Self {
        embedded_graphics_core::geometry::Size::new(
            u32::from(value.width.0),
            u32::from(value.height.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Interpolate as _;

    use super::ProposedDimension;

    #[test]
    fn interpolate_dimensions() {
        let from = super::Dimensions::new(10, 20);
        let to = super::Dimensions::new(20, 30);
        let result = super::Dimensions::interpolate(from, to, 128);
        assert_eq!(result.width.0, 15);
        assert_eq!(result.height.0, 25);
    }

    #[test]
    fn interpolate_dimension_min() {
        let from = super::Dimension::from(10);
        let to = super::Dimension::from(30);
        let result = super::Dimension::interpolate(from, to, 0);
        assert_eq!(result.0, 10);
    }

    #[test]
    fn interpolate_dimension_max() {
        let from = super::Dimension::from(10);
        let to = super::Dimension::from(30);
        let result = super::Dimension::interpolate(from, to, 255);
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
        assert!(ProposedDimension::Exact(u16::MAX) < ProposedDimension::Infinite);
    }
}
