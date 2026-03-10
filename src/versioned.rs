/// Tracks whether a data source has been modified.
///
/// Types implementing this trait provide a lightweight version identifier
/// that changes whenever the underlying data is modified. This enables
/// efficient change detection without comparing full data contents.
///
/// # Examples
///
/// ```
/// use buoyant::versioned::{Versioned, Generational};
///
/// let mut data = Generational::new(vec![1, 2, 3]);
/// let v1 = data.version();
///
/// data.get_mut().push(4);
/// let v2 = data.version();
///
/// assert_ne!(v1, v2);
/// ```
pub trait Versioned {
    /// A lightweight identifier for the current state of the data.
    type Version: PartialEq + Clone + 'static;

    /// Returns the current version identifier.
    fn version(&self) -> Self::Version;
}

/// A value that tracks modifications via a generation counter.
///
/// Each call to [`get_mut`](Self::get_mut) or [`set`](Self::set) increments
/// the generation, providing a cheap way to detect changes without comparing
/// the underlying data.
///
/// For interior mutability, wrap in `Cell<Generational<T>>` or
/// `RefCell<Generational<T>>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generational<T> {
    value: T,
    generation: u32,
}

impl<T> Generational<T> {
    /// Creates a new `Generational` value with generation 0.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            generation: 0,
        }
    }

    /// Returns a shared reference to the inner value.
    #[must_use]
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the inner value, incrementing the generation.
    pub fn get_mut(&mut self) -> &mut T {
        self.generation = self.generation.wrapping_add(1);
        &mut self.value
    }

    /// Replaces the inner value, incrementing the generation.
    pub fn set(&mut self, value: T) {
        self.generation = self.generation.wrapping_add(1);
        self.value = value;
    }

    /// Returns the current generation counter.
    #[must_use]
    pub const fn generation(&self) -> u32 {
        self.generation
    }
}

impl<T> Versioned for Generational<T> {
    type Version = u32;

    fn version(&self) -> u32 {
        self.generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_zero() {
        let g = Generational::new(42);
        assert_eq!(*g.get(), 42);
        assert_eq!(g.generation(), 0);
    }

    #[test]
    fn get_does_not_change_generation() {
        let g = Generational::new(42);
        let _ = g.get();
        assert_eq!(g.generation(), 0);
    }

    #[test]
    fn get_mut_increments_generation() {
        let mut g = Generational::new(42);
        *g.get_mut() = 100;
        assert_eq!(*g.get(), 100);
        assert_eq!(g.generation(), 1);
    }

    #[test]
    fn set_increments_generation() {
        let mut g = Generational::new(42);
        g.set(100);
        assert_eq!(*g.get(), 100);
        assert_eq!(g.generation(), 1);
    }

    #[test]
    fn versioned_trait() {
        let mut g = Generational::new(42);
        let v1 = g.version();
        g.set(100);
        let v2 = g.version();
        assert_ne!(v1, v2);
    }

    #[test]
    fn generation_wraps() {
        let mut g = Generational {
            value: 0,
            generation: u32::MAX,
        };
        g.set(1);
        assert_eq!(g.generation(), 0);
    }
}
