use core::time::Duration;

use crate::{
    event::input::{Groups, Input},
    layout::LayoutDirection,
};

pub trait LayoutEnvironment {
    fn layout_direction(&self) -> LayoutDirection;
    /// The duration since the application started.
    /// This is used to drive animations.
    fn app_time(&self) -> Duration;
    fn blur(&self, groups: Groups);
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultEnvironment<'a> {
    pub app_time: Duration,
    pub input: Option<&'a Input<'a>>,
}

impl<'a> DefaultEnvironment<'a> {
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self {
            app_time,
            input: None,
        }
    }

    #[must_use]
    pub const fn new_with_input(app_time: Duration, input: &'a Input<'a>) -> Self {
        Self {
            app_time,
            input: Some(input),
        }
    }

    #[must_use]
    pub const fn non_animated() -> Self {
        Self {
            app_time: Duration::new(0, 0),
            input: None,
        }
    }
}

impl LayoutEnvironment for DefaultEnvironment<'_> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::default()
    }

    fn app_time(&self) -> Duration {
        self.app_time
    }

    fn blur(&self, groups: Groups) {
        if let Some(input) = self.input {
            input.blur(groups);
        }
    }
}
