use core::sync::atomic::{AtomicU8, Ordering};

use super::cursor::ComponentPath;
use super::keyboard::KeyboardInput;

static DUMMY_ACTIVE_GROUPS: AtomicU8 = AtomicU8::new(0);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group(u8);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Groups(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapacityExceededError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoGroupError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusState {
    pub groups: Groups,
    pub focused: Groups,
}

#[derive(Default, Debug)]
pub struct GroupData {
    focused_path: ComponentPath,
}

#[derive(Default, Debug)]
pub struct Input<'a> {
    // Maybe be just `Cell<u8>` if ref would have `&'a Option<Cell<u8>>`.
    active_groups: AtomicU8,
    keyboards: heapless::Vec<(Groups, &'a KeyboardInput), 8>,
    groups: heapless::LinearMap<Groups, &'a GroupData, 8>,
}

#[derive(Debug, Clone, Copy)]
pub struct InputRef<'a> {
    active_groups: &'a AtomicU8,
    groups: &'a heapless::linear_map::LinearMapView<Groups, &'a GroupData>,
}

#[derive(Debug)]
pub struct Deactivation {
    groups: Groups,
}

#[derive(Debug)]
pub struct DeactivationGuard<'a> {
    input: InputRef<'a>,
    groups: Groups,
}

/// A set of applied modifiers.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interaction(pub(crate) u32);

// Note: we may report which group exactly is <pressed>, but I can't think of a use case.
impl Interaction {
    pub(crate) const PRESSED: u32 = 1 << 0;
    pub(crate) const FOCUSED: u32 = 1 << 1;
    pub(crate) const CLICKED: u32 = 1 << 2;
    pub(crate) const LONG_PRESSED: u32 = 1 << 3;

    pub(crate) const fn new() -> Self {
        Self(0)
    }
    pub(crate) fn with(self, on: bool, modifier: u32) -> Self {
        Self(self.0 | if on { modifier } else { 0 })
    }

    #[must_use]
    pub fn is_pressed(self) -> bool {
        (self.0 & Self::PRESSED) != 0
    }
    #[must_use]
    pub fn is_focused(self) -> bool {
        (self.0 & Self::FOCUSED) != 0
    }
    #[must_use]
    pub fn is_clicked(self) -> bool {
        (self.0 & Self::CLICKED) != 0
    }
    #[must_use]
    pub fn is_long_pressed(self) -> bool {
        (self.0 & Self::LONG_PRESSED) != 0
    }
}

impl Group {
    /// Create a new group
    /// # Panics
    /// If group is not in range `0..8`.
    #[must_use]
    pub const fn new(group: usize) -> Self {
        match Self::try_new(group) {
            Some(g) => g,
            None => panic!("Group must be in range 0..8."),
        }
    }

    /// Create a new group
    #[must_use]
    pub const fn try_new(group: usize) -> Option<Self> {
        if group < 8 {
            Some(Self(group as u8))
        } else {
            None
        }
    }
}

impl<'a> Input<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active_groups: AtomicU8::new(Groups::ZERO.0),
            keyboards: heapless::Vec::new(),
            groups: heapless::LinearMap::new(),
        }
    }

    /// Adds a new group to the input system.
    /// # Errors
    /// If the capacity of the input system is exceeded, an error is returned.
    pub fn add_group(
        &mut self,
        group: Group,
        data: &'a mut GroupData,
    ) -> Result<(), CapacityExceededError> {
        let groups: Groups = group.into();
        *self.active_groups.get_mut() |= groups.0;
        self.groups
            .insert(groups, data)
            .map(|_| ())
            .map_err(|_| CapacityExceededError)
    }

    /// Adds a new keyboard to the input system.
    /// # Errors
    /// If the capacity of the input system is exceeded, an error is returned.
    pub fn add_keyboard(
        &mut self,
        groups: Groups,
        keyboard: &'a KeyboardInput,
    ) -> Result<(), CapacityExceededError> {
        if groups.is_empty() {
            return Ok(());
        }
        if let Some((g, _)) = self.keyboards.iter_mut().find(|(_, k)| *k == keyboard) {
            *g |= groups;
            return Ok(());
        }
        self.keyboards
            .push((groups, keyboard))
            .map_err(|_| CapacityExceededError)
    }
    /// Processes a keyboard input state.
    pub fn keyboard_input(
        &self,
        keyboard: &'a KeyboardInput,
        key: super::keyboard::Key,
        button_state: super::keyboard::ButtonState,
        timestamp: core::time::Duration,
    ) -> Option<super::Event> {
        let &(groups, _) = self.keyboards.iter().find(|(_, k)| *k == keyboard)?;
        let mut event = keyboard.input(key, button_state, timestamp)?;
        event.groups = groups & self.active_groups();
        (!event.groups.is_empty()).then_some(super::Event::Keyboard(event))
    }

    #[inline]
    fn active_groups(&self) -> Groups {
        Groups(self.active_groups.load(Ordering::Relaxed))
    }

    pub fn as_ref(&'a self) -> InputRef<'a> {
        InputRef {
            active_groups: &self.active_groups,
            groups: &self.groups,
        }
    }
}

impl Default for InputRef<'_> {
    fn default() -> Self {
        Self::DUMMY
    }
}

impl<'a> InputRef<'a> {
    pub(crate) const DUMMY: InputRef<'static> = InputRef {
        active_groups: &DUMMY_ACTIVE_GROUPS,
        groups: &heapless::linear_map::LinearMap::<_, _, 0>::new(),
    };
    #[inline]
    fn active_groups(&self) -> Groups {
        Groups(self.active_groups.load(Ordering::Relaxed))
    }

    pub fn activate(self, groups: Groups) {
        self.active_groups.fetch_or(groups.0, Ordering::Relaxed);
    }
    pub fn deactivate(self, groups: Groups) -> Deactivation {
        self.active_groups.fetch_and(!groups.0, Ordering::Relaxed);
        Deactivation { groups }
    }
    pub fn traverse(
        self,
        groups: Groups,
        event: super::keyboard::KeyboardEventKind,
        max: usize,
        f: impl FnMut(usize) -> super::EventResult,
    ) -> super::EventResult {
        let groups = groups & self.active_groups();
        // TODO: traverse multiple groups with one event
        let Some((_, group)) = self.groups.iter().find(|&(&g, _)| (g & groups) != 0) else {
            return super::EventResult::default();
        };
        group.focused_path.traverse(event, max, f)
    }
    pub fn leaf_move(self, focus: &mut FocusState, groups: Groups) -> super::EventResult {
        let groups = groups & self.active_groups();

        if !focus.is_member_of_any(groups) {
            return super::EventResult::default();
        }

        // todo: debug assert that there is a sound path there

        if self.is_focused_any(groups) {
            debug_assert!(
                focus.is_focused_any(groups),
                "Included press must transition from focused to unfocused, but it is already unfocused.",
            );

            self.blur(focus.blur(groups));
            super::EventResult::new(false, true)
        } else {
            debug_assert!(
                !focus.is_focused_all(groups),
                "Included press must transition from unfocused to focused, but it is already focused.",
            );

            self.focus(focus.focus(groups));
            super::EventResult::new(true, true)
        }
    }
    pub fn is_focused_any(self, groups: Groups) -> bool {
        let groups = groups & self.active_groups();

        self.groups
            .iter()
            .filter(|g| (*g.0 & groups) != 0)
            .any(|g| g.1.focused_path.is_focused())
    }
    pub fn focus(self, groups: Groups) {
        let groups = groups & self.active_groups();

        for (&g, &data) in self.groups {
            if (g & groups) != Groups::ZERO {
                data.focused_path.focus();
            }
        }
    }
    pub fn blur(self, groups: Groups) {
        let groups = groups & self.active_groups();

        for (&g, &data) in self.groups {
            if (g & groups) != Groups::ZERO {
                data.focused_path.blur();
            }
        }
    }
    pub fn reset(self, groups: Groups) {
        let groups = groups & self.active_groups();

        for (&g, &data) in self.groups {
            if (g & groups) != Groups::ZERO {
                data.focused_path.reset();
            }
        }
    }
}

impl FocusState {
    #[must_use]
    pub const fn new(groups: Groups) -> Self {
        Self {
            groups,
            focused: Groups::ZERO,
        }
    }
    #[must_use]
    pub fn should_focus(&self, groups: Groups) -> bool {
        self.is_member_of_any(groups) && !self.is_focused_all(groups)
    }
    #[must_use]
    pub fn should_blur(&self, groups: Groups) -> bool {
        self.is_focused_any(groups)
    }
    #[must_use]
    pub fn is_member_of_any(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.groups & groups) != 0
    }
    #[must_use]
    pub fn is_member_of_all(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.groups & groups) == groups
    }
    pub fn focus(&mut self, groups: Groups) -> Groups {
        let groups = groups & self.groups;
        self.focused |= groups;
        groups
    }
    pub fn blur(&mut self, groups: Groups) -> Groups {
        let groups = groups & self.groups;
        self.focused &= !groups;
        groups
    }
    #[must_use]
    pub fn is_focused_all(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.focused & groups) == groups
    }
    #[must_use]
    pub fn is_focused_any(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.focused & groups) != 0
    }
}

impl Deactivation {
    pub fn into_guard<'a>(self, input: &'a Input<'a>) -> DeactivationGuard<'a> {
        DeactivationGuard {
            groups: self.groups,
            input: input.as_ref(),
        }
    }
}

impl Drop for DeactivationGuard<'_> {
    fn drop(&mut self) {
        self.input.activate(self.groups);
    }
}

impl core::error::Error for CapacityExceededError {}
impl core::fmt::Display for CapacityExceededError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Capacity exceeded")
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new(Groups(1))
    }
}

impl GroupData {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            focused_path: ComponentPath::new(),
        }
    }
}

impl Groups {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn from_mask(mask: u8) -> Self {
        Self(mask)
    }

    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl FromIterator<Group> for Groups {
    fn from_iter<T: IntoIterator<Item = Group>>(iter: T) -> Self {
        let mut groups = Self::ZERO;
        for group in iter {
            groups |= group.into();
        }
        groups
    }
}

impl core::ops::BitOr for Groups {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for Groups {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitXor for Groups {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for Groups {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl core::ops::BitOrAssign for Groups {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAndAssign for Groups {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl From<Group> for Groups {
    fn from(value: Group) -> Self {
        Self(1u8.unbounded_shl(u32::from(value.0)))
    }
}

impl PartialEq<usize> for Groups {
    fn eq(&self, other: &usize) -> bool {
        self.0 as usize == *other
    }
}
