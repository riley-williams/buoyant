use core::time::Duration;

use crate::{
    event::input::{Groups, Input, InputRef},
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
    pub input: InputRef<'a>,
}

impl<'a> DefaultEnvironment<'a> {
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self {
            app_time,
            input: InputRef::DUMMY,
        }
    }

    #[must_use]
    pub fn input(self, input: &'a Input<'a>) -> Self {
        Self {
            input: input.as_ref(),
            ..self
        }
    }

    // Input should always be provided, `as_drawable` is probably fine to not blur subtrees?
    #[must_use]
    pub const fn non_animated() -> Self {
        Self {
            app_time: Duration::new(0, 0),
            input: InputRef::DUMMY,
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

    // When subtree changes, shouldn't it reset instead of blur?
    fn blur(&self, groups: Groups) {
        self.input.blur(groups)
    }
}
