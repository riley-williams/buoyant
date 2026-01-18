use crate::{
    event::EventResult,
    primitives::{
        Point,
        geometry::{self, Shape as _},
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FocusStateChange {
    /// Focus sucessfully moved to a new element
    Focused {
        /// The content shape of the focused element
        shape: ContentShape,
        result: EventResult,
    },
    /// All elements in this subtree have been exhausted
    Exhausted,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ContentShape {
    /// The focused element has no content shape
    #[default]
    Empty,
    Rectangle(geometry::Rectangle),
    RoundedRectangle(geometry::RoundedRectangle),
    Circle(geometry::Circle),
}

impl ContentShape {
    /// Returns the bounding rectangle of this content shape, if any.
    #[must_use]
    pub fn bounding_box(&self) -> Option<geometry::Rectangle> {
        match self {
            Self::Empty => None,
            Self::Rectangle(rect) => Some(rect.clone()),
            Self::RoundedRectangle(rrect) => Some(rrect.bounding_box()),
            Self::Circle(circle) => Some(circle.bounding_box()),
        }
    }

    /// Offsets the shape
    #[must_use]
    pub fn offset(self, offset: Point) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Rectangle(rect) => Self::Rectangle(rect.offset(offset)),
            Self::RoundedRectangle(rrect) => Self::RoundedRectangle(rrect.offset(offset)),
            Self::Circle(circle) => Self::Circle(circle.offset(offset)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusEvent {
    pub action: FocusAction,
    pub roles: RoleMask,
}

impl FocusEvent {
    /// Creates a new focus event with the given action and role mask
    #[must_use]
    pub fn new(action: FocusAction, roles: RoleMask) -> Self {
        Self { action, roles }
    }
}

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
    /// Perform the focused element's primary action
    Select,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Role {
    /// No role, does not match any mask and cannot be focused
    #[default]
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
    pub fn mask(&self) -> RoleMask {
        match self {
            Self::None => RoleMask(0b0000),
            Self::Button => RoleMask(0b0001),
            Self::Text => RoleMask(0b0010),
            Self::TextEntry => RoleMask(0b0100),
            Self::Container => RoleMask(0b1000),
        }
    }

    /// Returns true if this role matches the given mask
    #[must_use]
    pub fn matches(&self, mask: RoleMask) -> bool {
        let role_mask = self.mask();
        (role_mask.0 & mask.0) != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoleMask(u16);

impl RoleMask {
    #[must_use]
    pub fn any() -> Self {
        Self(u16::MAX)
    }

    #[must_use]
    pub fn contains(self, role: Role) -> bool {
        role.matches(self)
    }
}

/// A trait for focus tree types that can be initialized to either the first or last element.
///
/// This is roughly equivalent to `Default` but support bidirectional navigation
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
