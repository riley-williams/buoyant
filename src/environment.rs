use core::time::Duration;

use crate::layout::{LayoutDirection, SafeAreaInsets};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    /// The duration since the application started.
    /// This is used to drive animations.
    fn app_time(&self) -> Duration;

    /// The safe area insets, typically used to avoid notches and other screen obstructions.
    fn safe_area_insets(&self) -> &SafeAreaInsets;

    // /// Dark mode / light mode?
    // fn color_scheme(&self) -> &mut ColorScheme;
    // ///
    // /// Arbitrary environment values? Is this even remotely efficient to implement in no_alloc?
    // fn value<T: 'static + ???>(&self) -> &mut T;
}

#[derive(Clone, Debug, Default)]
pub struct DefaultEnvironment {
    pub layout_direction: LayoutDirection,
    pub app_time: Duration,
    pub safe_area_insets: SafeAreaInsets,
}

impl DefaultEnvironment {
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self {
            app_time,
            layout_direction: LayoutDirection::Horizontal, // ::default()
            safe_area_insets: SafeAreaInsets::zero(),
        }
    }

    #[must_use]
    pub fn non_animated() -> Self {
        Self::new(Duration::ZERO)
    }
}

impl LayoutEnvironment for DefaultEnvironment {
    fn layout_direction(&self) -> LayoutDirection {
        self.layout_direction
    }

    fn app_time(&self) -> Duration {
        self.app_time
    }

    fn safe_area_insets(&self) -> &SafeAreaInsets {
        &self.safe_area_insets
    }
}
