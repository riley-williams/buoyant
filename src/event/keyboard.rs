use super::input::Groups;

mod input;

pub use input::KeyboardInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub kind: KeyboardEventKind,
    pub groups: Groups,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardEventKind {
    Click,
    LongPress,

    Cancel,

    Up,
    Down,
    Left,
    Right,

    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Navitating,
    Editing,
}

impl KeyboardEventKind {
    pub fn is_movement(self) -> bool {
        match self {
            Self::Click | Self::LongPress | Self::Cancel | Self::Char(_) => false,

            Self::Up | Self::Down | Self::Left | Self::Right => true,
        }
    }
}

impl KeyboardEvent {
    pub const fn new(kind: KeyboardEventKind) -> Self {
        Self {
            kind,
            groups: Groups::ZERO,
        }
    }
    pub const fn new_with_groups(kind: KeyboardEventKind, groups: Groups) -> Self {
        Self { kind, groups }
    }
}
