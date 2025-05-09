use core::time::Duration;

use crate::layout::LayoutDirection;

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    /// The duration since the application started.
    /// This is used to drive animations.
    fn app_time(&self) -> Duration;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultEnvironment {
    pub app_time: Duration,
}

impl DefaultEnvironment {
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self { app_time }
    }

    #[must_use]
    pub fn non_animated() -> Self {
        Self {
            app_time: Duration::default(),
        }
    }
}

impl LayoutEnvironment for DefaultEnvironment {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn app_time(&self) -> Duration {
        self.app_time
    }
}
