use core::fmt;
use core::sync::atomic::{AtomicU8, Ordering};

use super::cursor::ComponentPath;
use super::keyboard::KeyboardInput;

static DUMMY_ACTIVE_GROUPS: AtomicU8 = AtomicU8::new(0);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group(u8);

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
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
    groups: &'a heapless::LinearMap<Groups, &'a GroupData, 8>,
}

#[derive(Debug, Clone)]
pub struct Deactivation {
    activate: Groups,
    reset: Groups,
    deactivate: Groups,
}

#[derive(Debug)]
pub struct DeactivationGuard<'a> {
    input: InputRef<'a>,
    activate: Groups,
    reset: Groups,
    deactivate: Groups,
}

/// A set of applied modifiers.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interaction(pub(crate) u32);

pub trait InputExtension {
    fn into_guard(self, input: InputRef<'_>) -> DeactivationGuard<'_>;
    fn take_guard<'a>(&mut self, input: InputRef<'a>) -> DeactivationGuard<'a>;
}

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

    pub fn activate(&mut self, groups: Groups) {
        self.as_ref().activate(groups)
    }
    #[must_use]
    pub fn deactivate(&self, groups: Groups) -> Deactivation {
        self.as_ref().deactivate(groups)
    }

    #[inline]
    fn active_groups(&self) -> Groups {
        Groups(self.active_groups.load(Ordering::Relaxed))
    }

    pub const fn as_ref(&'a self) -> InputRef<'a> {
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

impl InputRef<'_> {
    pub(crate) const DUMMY: InputRef<'static> = InputRef {
        active_groups: &DUMMY_ACTIVE_GROUPS,
        groups: &heapless::linear_map::LinearMap::<_, _, 8>::new(),
    };
    #[inline]
    #[must_use]
    pub fn active_groups(&self) -> Groups {
        Groups(self.active_groups.load(Ordering::Relaxed))
    }

    pub fn activate(self, groups: Groups) {
        self.active_groups.fetch_or(groups.0, Ordering::Relaxed);
    }
    #[must_use]
    pub fn deactivate(self, groups: Groups) -> Deactivation {
        self.active_groups.fetch_and(!groups.0, Ordering::Relaxed);
        Deactivation {
            activate: groups,
            reset: Groups::ZERO,
            deactivate: Groups::ZERO,
        }
    }
    /// Deactivates `from` groups and activates `to` groups, returning a
    /// [`Deactivation`] guard that will reverse it. `from` and `to` must be
    /// disjunctive, else behaviour is unspecified.
    #[must_use]
    pub fn replace(self, from: Groups, to: Groups) -> Deactivation {
        debug_assert_eq!(
            from & to,
            Groups::ZERO,
            "`from` and `to` groups should be disjunct"
        );
        let active = self.active_groups();
        // remove currently inactive from `from` to not enable them in drop
        // remove already active from `to` to not disable them in drop
        let (from, deactivate) = (from & active, to & !active);
        // We are not doing multithreading, it is atomic to have the dummy input ref,
        // it is fine to race there. Proper solution is a compare exchage loop.
        self.active_groups.fetch_and(!from.0, Ordering::Relaxed);
        self.active_groups.fetch_or(to.0, Ordering::Relaxed);
        Deactivation {
            activate: from,
            reset: to,
            deactivate,
        }
    }
    pub fn traverse(
        self,
        groups: Groups,
        event: super::keyboard::KeyboardEventKind,
        initial: u8,
        i: impl Fn(u8, super::keyboard::KeyboardEventKind) -> (bool, u8),
        f: &mut dyn FnMut(usize) -> super::EventResult,
    ) -> super::EventResult {
        let groups = groups & self.active_groups();
        // TODO: traverse multiple groups with one event
        let Some((_, group)) = self.groups.iter().find(|&(&g, _)| (g & groups) != 0) else {
            return super::EventResult::default();
        };
        group.focused_path.traverse(event, initial, i, f)
    }
    #[inline(never)]
    pub fn traverse_linear(
        self,
        groups: Groups,
        event: super::keyboard::KeyboardEventKind,
        max: usize,
        f: &mut dyn FnMut(usize) -> super::EventResult,
    ) -> super::EventResult {
        let groups = groups & self.active_groups();
        // TODO: traverse multiple groups with one event
        let Some((_, group)) = self.groups.iter().find(|&(&g, _)| (g & groups) != 0) else {
            return super::EventResult::default();
        };
        group.focused_path.traverse_linear(event, max, f)
    }
    pub fn leaf_move(self, focus: &mut FocusState, groups: Groups) -> super::EventResult {
        let groups = groups & self.active_groups();

        if !focus.is_member_of_any(groups) {
            return super::EventResult::default();
        }

        if self.is_focused_any(groups) {
            debug_assert!(
                focus.is_focused_any(groups),
                "Current phase is focused->blurred, but the leaf is already blurred, not focused.
State is corrupted.
Event Groups: {groups:?}
",
            );

            self.blur(focus.blur(groups));
            super::EventResult::new(false, true)
        } else {
            debug_assert!(
                !focus.is_focused_all(groups),
                "Current phase is blurred->focused, but the leaf is already focused, not blurred.
State is corrupted.
Event Groups: {groups:?}
",
            );

            self.focus(focus.focus(groups));
            super::EventResult::new(true, true)
        }
    }
    #[must_use]
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

    #[must_use]
    pub const fn contains(self, group: Group) -> bool {
        (self.0 & (1 << group.0)) != 0
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

impl Deactivation {
    #[allow(clippy::must_use_candidate)]
    pub fn into_guard<'a>(self, input: impl Into<InputRef<'a>>) -> DeactivationGuard<'a> {
        DeactivationGuard {
            activate: self.activate,
            reset: self.reset,
            deactivate: self.deactivate,
            input: input.into(),
        }
    }
}

impl<'a> From<&'a Input<'a>> for InputRef<'a> {
    fn from(value: &'a Input<'a>) -> Self {
        value.as_ref()
    }
}

impl Drop for DeactivationGuard<'_> {
    fn drop(&mut self) {
        self.input.activate(self.activate);
        self.input.reset(self.reset);
        _ = self.input.deactivate(self.deactivate);
    }
}

impl InputExtension for Deactivation {
    fn into_guard(self, input: InputRef<'_>) -> DeactivationGuard<'_> {
        Deactivation::into_guard(self, input)
    }
    fn take_guard<'a>(&mut self, input: InputRef<'a>) -> DeactivationGuard<'a> {
        let g = Deactivation::into_guard(self.clone(), input);
        self.activate = Groups::ZERO;
        self.reset = Groups::ZERO;
        self.deactivate = Groups::ZERO;
        g
    }
}

impl InputExtension for Option<Deactivation> {
    fn into_guard(self, input: InputRef<'_>) -> DeactivationGuard<'_> {
        let activate @ reset @ deactivate = Groups::ZERO;
        self.map(|this| Deactivation::into_guard(this, input))
            .unwrap_or_else(|| DeactivationGuard {
                input,
                activate,
                reset,
                deactivate,
            })
    }
    fn take_guard<'a>(&mut self, input: InputRef<'a>) -> DeactivationGuard<'a> {
        self.take().into_guard(input)
    }
}

impl core::error::Error for CapacityExceededError {}
impl core::fmt::Display for CapacityExceededError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Capacity exceeded")
    }
}

// TODO: document that buttons and stuff have group 0 by default (1 is the mask).
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

impl fmt::Display for Groups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("Groups(0b")?;
            for i in (0..8).rev() {
                let bit = (self.0 >> i) & 1;
                f.write_str(if bit == 1 { "1" } else { "0" })?;
            }
            return f.write_str(")");
        }

        f.write_str("Groups({ ")?;

        let mut this = *self;

        if !this.is_empty() {
            let first = this.0.trailing_zeros();
            this.0 &= !(1 << first);
            write!(f, "{}", first)?;
        }

        for g in (1..8).map(Group::new).filter(|g| this.contains(*g)) {
            write!(f, ", {}", g.0)?;
        }

        f.write_str(" })")
    }
}

impl fmt::Debug for Groups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<A> InputExtension for Option<(A, Deactivation)> {
    fn into_guard(self, input: InputRef<'_>) -> DeactivationGuard<'_> {
        self.map(|(_, it)| it).into_guard(input)
    }
    fn take_guard<'a>(&mut self, input: InputRef<'a>) -> DeactivationGuard<'a> {
        self.take().into_guard(input)
    }
}
