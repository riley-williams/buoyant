/// Manages a source and target render tree that can be swapped without copying.
#[derive(Debug)]
pub struct Trees<T> {
    tree_a: T,
    tree_b: T,
    source_is_a: bool,
}

impl<T> Trees<T> {
    pub fn new(source: T, target: T) -> Self {
        Self {
            tree_a: source,
            tree_b: target,
            source_is_a: true,
        }
    }

    /// Returns a reference to the source tree.
    #[must_use]
    pub fn source(&self) -> &T {
        if self.source_is_a {
            &self.tree_a
        } else {
            &self.tree_b
        }
    }

    /// Returns a reference to the target tree.
    #[must_use]
    pub fn target(&self) -> &T {
        if self.source_is_a {
            &self.tree_b
        } else {
            &self.tree_a
        }
    }

    /// Returns a mutable reference to the source tree.
    #[must_use]
    pub fn source_mut(&mut self) -> &mut T {
        if self.source_is_a {
            &mut self.tree_a
        } else {
            &mut self.tree_b
        }
    }

    /// Returns a mutable reference to the target tree.
    #[must_use]
    pub fn target_mut(&mut self) -> &mut T {
        if self.source_is_a {
            &mut self.tree_b
        } else {
            &mut self.tree_a
        }
    }

    /// Returns an tuple of mutable references to the trees.
    ///
    /// # Example
    ///
    /// ```
    /// # use buoyant::app::Trees;
    /// let mut trees = Trees::new(1, 2);
    ///
    /// let (source, target) = trees.both_mut();
    ///
    /// assert_eq!(*source, 1);
    /// assert_eq!(*target, 2);
    /// ```
    #[must_use]
    pub fn both_mut(&mut self) -> (&mut T, &mut T) {
        if self.source_is_a {
            (&mut self.tree_a, &mut self.tree_b)
        } else {
            (&mut self.tree_b, &mut self.tree_a)
        }
    }

    /// Swaps the source and target trees.
    ///
    /// This operation does not copy any data.
    pub(crate) fn swap(&mut self) {
        self.source_is_a = !self.source_is_a;
    }
}
