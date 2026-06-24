mod role;

use core::fmt;

pub use role::{Role, RoleSet};

use crate::event::Event;

/// The direction to search when acquiring focus
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FocusDirection {
    /// Search forward (towards the end)
    #[default]
    Forward,
    /// Search backward (towards the beginning)
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusAction {
    /// Move focus to the next element
    Next,
    /// Move focus to the previous element
    Previous,
    /// Obtain the selected container
    ///
    /// If the current container does not match the requested role mask,
    /// the direction will be used to obtain the nearest matching element.
    Focus(FocusDirection),
    /// Exit the selected container.
    ///
    /// Typically associated with the user pressing a "back" or "menu" button.
    Blur,
    /// Perform the focused element's primary action. If the currently focused element
    /// does not match the requested role set, this does nothing.
    Select,
    /// Inform the focused tree that it is being terminated and should perform
    /// any necessary cleanup.
    Teardown,
}

impl FocusAction {
    #[must_use]
    pub fn into_event(self, group: FocusGroup) -> Event {
        Event::Focus {
            action: self,
            group,
        }
    }
}

/// A trait for focus tree types that can be initialized to either the first or last element.
///
/// This is roughly equivalent to `Default` but supports bidirectional navigation
pub trait DefaultFocus {
    /// Returns a focus tree initialized to the first element.
    fn default_first() -> Self;

    /// Returns a focus tree initialized to the last element.
    fn default_last() -> Self;
}

impl DefaultFocus for () {
    fn default_first() -> Self {}
    fn default_last() -> Self {}
}

/// A group identifying a set of related elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FocusGroup(u8);

/// A set of focus groups.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FocusGroupSet(u8);

impl FocusGroup {
    /// Creates a focus group with the specified group index (0-7)
    #[must_use]
    pub const fn new(i: u8) -> Option<Self> {
        if i < 8 { Some(Self(0b1 << i)) } else { None }
    }

    /// Creates a focus group with the specified group index (0-7)
    #[must_use]
    pub const fn new_unchecked(i: u8) -> Self {
        debug_assert!(i < 8, "Focus group index must be between 0 and 7");
        Self(0b1 << i)
    }

    /// Returns the underlying group index (0-7) for this focus group
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    /// Returns a focus group set containing only this focus group
    #[must_use]
    pub const fn set(self) -> FocusGroupSet {
        FocusGroupSet(self.0)
    }
}

impl Default for FocusGroup {
    fn default() -> Self {
        Self::new_unchecked(0)
    }
}

impl FocusGroupSet {
    /// Creates a focus group set matching no focus groups.
    #[must_use]
    pub const fn new_none() -> Self {
        Self(0b0000_0000)
    }

    /// Creates a focus group set matching any focus group.
    #[must_use]
    pub const fn new_any() -> Self {
        Self(0b1111_1111)
    }

    /// Returns true if this set contains the specified focus group
    #[must_use]
    pub const fn contains(self, group: FocusGroup) -> bool {
        (self.0 & group.0) != 0
    }

    /// Returns true if this set contains no focus groups.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns a copy of this set with the specified focus group removed.
    #[must_use]
    pub const fn without(self, group: FocusGroup) -> Self {
        Self(self.0 & !group.0)
    }
}

impl Default for FocusGroupSet {
    fn default() -> Self {
        Self::new_none()
    }
}

impl core::ops::BitOr for FocusGroup {
    type Output = FocusGroupSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        FocusGroupSet(self.0 | rhs.0)
    }
}

impl From<FocusGroup> for FocusGroupSet {
    fn from(group: FocusGroup) -> Self {
        group.set()
    }
}

impl core::ops::BitOr<FocusGroup> for FocusGroupSet {
    type Output = Self;

    fn bitor(self, rhs: FocusGroup) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOr for FocusGroupSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for FocusGroupSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

pub static GROUP_0: FocusGroup = FocusGroup(0b1 << 0);
pub static GROUP_1: FocusGroup = FocusGroup(0b1 << 1);
pub static GROUP_2: FocusGroup = FocusGroup(0b1 << 2);
pub static GROUP_3: FocusGroup = FocusGroup(0b1 << 3);
pub static GROUP_4: FocusGroup = FocusGroup(0b1 << 4);
pub static GROUP_5: FocusGroup = FocusGroup(0b1 << 5);
pub static GROUP_6: FocusGroup = FocusGroup(0b1 << 6);
pub static GROUP_7: FocusGroup = FocusGroup(0b1 << 7);

/// Behavior when focus reaches the boundary of the bounded region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundaryBehavior {
    /// Wrap focus around to the other side when reaching the end
    #[default]
    Wrap,
    /// Stop movement at the boundaries (focus stays on the current element)
    Stop,
}

impl fmt::Display for FocusGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FocusGroup({})", self.index())
    }
}
impl fmt::Debug for FocusGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl fmt::Display for FocusGroupSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FocusGroupSet({:08b})", self.0)
    }
}
impl fmt::Debug for FocusGroupSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_indices() {
        for i in 0..8 {
            let group = FocusGroup::new(i).unwrap();
            let unchecked_group = FocusGroup::new_unchecked(i);
            assert_eq!(group, unchecked_group);
            assert_eq!(group.index(), i);
        }
    }

    #[test]
    fn group_contains_all_groups() {
        assert_eq!(FocusGroupSet::new_any(), FocusGroupSet::new_any());

        for i in 0..8 {
            let group = FocusGroup::new(i).unwrap();
            assert!(FocusGroupSet::new_any().contains(group));
        }
    }
}
