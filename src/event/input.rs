use core::cell::Cell;

use super::cursor::ComponentPath;
use super::keyboard::KeyboardInput;

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

#[derive(Debug)]
pub struct Input<'a> {
    pub active_groups: Cell<Groups>,
    pub keyboards: heapless::Vec<(Groups, &'a KeyboardInput), 8>,
    pub groups: heapless::LinearMap<Groups, &'a GroupData, 8>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct InputRef<'a>(Option<&'a Input<'a>>);

#[derive(Debug)]
pub struct DeactivationGuard<'a> {
    input: InputRef<'a>,
    groups: Groups,
}

impl Group {
    pub const fn new(group: usize) -> Self {
        match Self::try_new(group) {
            Some(g) => g,
            None => panic!("Group must be in range 0..8."),
        }
    }
    pub const fn try_new(group: usize) -> Option<Self> {
        if group < 8 {
            Some(Self(group as u8))
        } else {
            None
        }
    }
}

impl<'a> Input<'a> {
    pub fn new() -> Self {
        Self {
            active_groups: Cell::new(Groups::ZERO),
            keyboards: heapless::Vec::new(),
            groups: heapless::LinearMap::new(),
        }
    }

    pub const fn as_ref(&self) -> InputRef<'_> {
        InputRef(Some(self))
    }

    pub fn keyboard_input(
        &self,
        keyboard: &'a KeyboardInput,
        key: super::keyboard::Key,
        button_state: super::keyboard::ButtonState,
        timestamp: core::time::Duration,
    ) -> Option<super::Event> {
        let &(groups, _) = self.keyboards.iter().find(|(_, k)| *k == keyboard)?;
        let mut event = keyboard.input(key, button_state, timestamp)?;
        event.groups = groups;
        Some(super::Event::Keyboard(event))
    }
    pub fn activate(&self, groups: Groups) {
        self.active_groups.update(|g| g | groups);
    }
    pub fn deactivate(&self, groups: Groups) {
        self.active_groups.update(|g| g & !groups);
    }
    pub fn scoped_deactivate(&self, groups: Groups) -> DeactivationGuard<'_> {
        self.deactivate(groups);
        DeactivationGuard {
            input: self.as_ref(),
            groups,
        }
    }
    pub fn traverse(
        &self,
        groups: Groups,
        event: super::keyboard::KeyboardEventKind,
        max: usize,
        f: impl FnMut(usize) -> super::EventResult,
    ) -> super::EventResult {
        let groups = groups & self.active_groups.get();
        // TODO: traverse multiple groups with one event
        let Some((_, group)) = self.groups.iter().find(|&(&g, _)| (g & groups) != 0) else {
            return super::EventResult::default();
        };
        group.focused_path.traverse(event, max, f)
    }
    pub fn leaf_move(&self, focus: &mut FocusState, groups: Groups) -> super::EventResult {
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
    pub fn is_focused_any(&self, groups: Groups) -> bool {
        self.groups
            .iter()
            .filter(|g| (*g.0 & groups) != 0)
            .any(|g| g.1.focused_path.is_focused())
    }
    pub fn focus(&self, groups: Groups) {
        let groups = groups & self.active_groups.get();
        for (&g, &data) in self.groups.iter() {
            if (g & groups) != Groups::ZERO {
                data.focused_path.focus()
            }
        }
    }
    pub fn blur(&self, groups: Groups) {
        let groups = groups & self.active_groups.get();
        for (&g, &data) in self.groups.iter() {
            if (g & groups) != Groups::ZERO {
                data.focused_path.blur()
            }
        }
    }
    pub fn add_group(
        &mut self,
        group: Group,
        data: &'a mut GroupData,
    ) -> Result<(), CapacityExceededError> {
        let groups = group.into();
        *self.active_groups.get_mut() |= groups;
        self.groups
            .insert(groups, data)
            .map(|_| ())
            .map_err(|_| CapacityExceededError)
    }
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
}

impl<'a> InputRef<'a> {
    pub const fn empty() -> Self {
        Self(None)
    }

    pub fn keyboard_input(
        self,
        keyboard: &'a KeyboardInput,
        key: super::keyboard::Key,
        button_state: super::keyboard::ButtonState,
        timestamp: core::time::Duration,
    ) -> Option<super::Event> {
        self.0
            .and_then(|i| i.keyboard_input(keyboard, key, button_state, timestamp))
    }

    pub fn activate(self, groups: Groups) {
        if let Some(i) = self.0 {
            i.activate(groups);
        }
    }
    pub fn deactivate(self, groups: Groups) {
        if let Some(i) = self.0 {
            i.deactivate(groups);
        }
    }
    pub fn scoped_deactivate(self, groups: Groups) -> DeactivationGuard<'a> {
        self.deactivate(groups);
        DeactivationGuard {
            input: self,
            groups,
        }
    }
    pub fn traverse(
        self,
        groups: Groups,
        event: super::keyboard::KeyboardEventKind,
        max: usize,
        f: impl FnMut(usize) -> super::EventResult,
    ) -> super::EventResult {
        if let Some(i) = self.0 {
            i.traverse(groups, event, max, f)
        } else {
            super::EventResult::default()
        }
    }
    pub fn leaf_move(self, focus: &mut FocusState, groups: Groups) -> super::EventResult {
        if let Some(i) = self.0 {
            i.leaf_move(focus, groups)
        } else {
            super::EventResult::default()
        }
    }
    pub fn is_focused_any(self, groups: Groups) -> bool {
        self.0.is_some_and(|i| i.is_focused_any(groups))
    }
    pub fn focus(self, groups: Groups) {
        if let Some(i) = self.0 {
            i.focus(groups);
        }
    }
    pub fn blur(self, groups: Groups) {
        if let Some(i) = self.0 {
            i.blur(groups);
        }
    }
}

impl FocusState {
    pub const fn new(groups: Groups) -> Self {
        Self {
            groups,
            focused: Groups::ZERO,
        }
    }
    pub fn should_focus(&self, groups: Groups) -> bool {
        self.is_member_of_any(groups) && !self.is_focused_all(groups)
    }
    pub fn should_blur(&self, groups: Groups) -> bool {
        self.is_focused_any(groups)
    }
    pub fn is_member_of_any(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.groups & groups) != 0
    }
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
    pub fn is_focused_all(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.focused & groups) == groups
    }
    pub fn is_focused_any(&self, groups: Groups) -> bool {
        let groups = groups & self.groups;
        (self.focused & groups) != 0
    }
}

impl GroupData {
    pub const fn new() -> Self {
        Self {
            focused_path: ComponentPath::new(),
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

impl Groups {
    pub const ZERO: Self = Groups(0);

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl FromIterator<Group> for Groups {
    fn from_iter<T: IntoIterator<Item = Group>>(iter: T) -> Self {
        let mut groups = Groups::ZERO;
        for group in iter {
            groups |= group.into();
        }
        groups
    }
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new(Groups(1))
    }
}

impl core::ops::BitOr for Groups {
    type Output = Groups;
    fn bitor(self, rhs: Self) -> Self::Output {
        Groups(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for Groups {
    type Output = Groups;
    fn bitand(self, rhs: Self) -> Self::Output {
        Groups(self.0 & rhs.0)
    }
}

impl core::ops::BitXor for Groups {
    type Output = Groups;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Groups(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for Groups {
    type Output = Groups;
    fn not(self) -> Self::Output {
        Groups(!self.0)
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
        Self(1u8.unbounded_shl(value.0 as u32))
    }
}

impl PartialEq<usize> for Groups {
    fn eq(&self, other: &usize) -> bool {
        self.0 as usize == *other
    }
}
