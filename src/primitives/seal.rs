use core::ops::{Deref, DerefMut};

/// A [`Seal`] is a wrapper around a mutable reference that can be "broken" to allow mutation.
/// This is used to determine if a view tree needs to be re-computed due to changes in the underlying data.
///
/// # Examples
///
/// Reading doesn't break the seal:
///
/// ```
/// # use buoyant::primitives::Seal;
/// fn read_only_operation(value: &mut Seal<i32>) {
///     println!("Value: {}", *value);
/// }
/// ```
///
/// Writing breaks the seal:
///
/// ```
/// # use buoyant::primitives::Seal;
/// fn mutating_operation(value: &mut Seal<i32>) {
///     *value.as_mut() += 10;
/// }
/// ```
///
/// Conditional operations may avoid breaking the seal if no mutation occurs:
///
/// ```
/// # use buoyant::primitives::Seal;
/// fn conditional_operation(value: &mut Seal<i32>, should_modify: bool) {
///     if should_modify {
///         *value.as_mut() = 100;
///     } else {
///         println!("Value: {}", *value);
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Seal<'a, T: ?Sized> {
    value: &'a mut T,
    is_broken: bool,
}

impl<T> core::fmt::Display for Seal<'_, T>
where
    T: ?Sized + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, T: ?Sized> Seal<'a, T> {
    #[must_use]
    pub(crate) const fn new(value: &'a mut T) -> Self {
        Self {
            value,
            is_broken: false,
        }
    }

    /// Mark the seal as broken, triggering a re-computation of the view tree.
    ///
    /// This may be necessary if the underlying data uses interior mutability or
    /// if the the view state is not a pure function of the data.
    pub const fn break_seal(&mut self) {
        self.is_broken = true;
    }

    /// Check if the seal was broken.
    #[must_use]
    pub const fn is_broken(&self) -> bool {
        self.is_broken
    }
}

impl<T: ?Sized> AsRef<T> for Seal<'_, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> AsMut<T> for Seal<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        self.is_broken = true;
        self.value
    }
}

impl<T: ?Sized> Deref for Seal<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: ?Sized> DerefMut for Seal<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_broken = true;
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Seal;

    #[test]
    fn test_ref_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert_eq!(seal.as_ref(), &42);
        assert!(!seal.is_broken);
        *(seal.as_mut()) = 43;
        assert_eq!(seal.value, &43);
        assert!(seal.is_broken);
    }

    #[test]
    fn test_deref_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert_eq!(*seal, 42);
        assert!(!seal.is_broken);
        *seal = 43;
        assert_eq!(seal.value, &43);
        assert!(seal.is_broken);
    }

    #[test]
    fn test_manually_break_seal() {
        let mut value = 42;
        let mut seal = Seal::new(&mut value);
        assert!(!seal.is_broken);
        seal.break_seal();
        assert!(seal.is_broken);
    }
}
