use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

/// A transparent wrapper around a value that tracks whether the inner value
/// has may have changed since the last reset.
///
/// Common operations like Eq, Ord, and Hash are implemented by delegating to the inner value.
#[derive(Debug)]
pub struct Tracked<'a, T> {
    value: &'a mut T,
    changed: &'a mut bool,
}

impl<'a, T> Tracked<'a, T> {
    /// Creates a new tracked value.
    pub(crate) fn new(value: &'a mut T, flag: &'a mut bool) -> Self {
        Self {
            value,
            changed: flag,
        }
    }

    /// Returns true if the value has been changed since the last reset.
    #[must_use]
    pub fn has_changed(&self) -> bool {
        *self.changed
    }

    /// Manually marks the value as changed.
    pub fn mark_changed(&mut self) {
        *self.changed = true;
    }
}

impl<T> Deref for Tracked<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> DerefMut for Tracked<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        *self.changed = true;
        self.value
    }
}

impl<T> AsRef<T> for Tracked<'_, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

impl<T> AsMut<T> for Tracked<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        *self.changed = true;
        self.value
    }
}

impl<T: PartialEq> PartialEq for Tracked<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Tracked<'_, T> {}

impl<T: PartialOrd> PartialOrd for Tracked<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Tracked<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: Hash> Hash for Tracked<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
