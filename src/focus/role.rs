use core::ops::{BitAnd, BitOr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Role {
    /// No role, does not match any mask and cannot be focused
    None,
    /// An element that performs an action
    Button,
    /// An element that displays text
    Text,
    /// An element that supports text entry
    TextEntry,
    /// A container which holds other focusable elements, often wrapping focus
    /// within the container.
    Container,
}

impl Role {
    /// The mask corresponding to this role
    #[must_use]
    const fn mask(self) -> RoleSet {
        match self {
            Self::None => RoleSet(0),
            Self::Button => RoleSet(1 << 0),
            Self::Text => RoleSet(1 << 1),
            Self::TextEntry => RoleSet(1 << 2),
            Self::Container => RoleSet(1 << 3),
        }
    }
}

impl BitOr for Role {
    type Output = RoleSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.mask() | rhs.mask()
    }
}

impl BitAnd for Role {
    type Output = RoleSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.mask() & rhs.mask()
    }
}

impl From<Role> for RoleSet {
    fn from(value: Role) -> Self {
        value.mask()
    }
}

/// A set of [`Role`]s
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoleSet(u16);

impl RoleSet {
    /// A set matching all roles
    #[must_use]
    pub const fn any() -> Self {
        Self(u16::MAX)
    }

    /// A set not matching any roles
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns true if this set contains the given role
    #[must_use]
    pub const fn contains(self, role: Role) -> bool {
        self.0 & role.mask().0 != 0
    }
}

impl BitOr for RoleSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for RoleSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
