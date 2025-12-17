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
    // Well, we may drop `input` if use a bit of unsafe code to have empty `Input`
    // in `static` - no actual `Cell`s will be present, so no data race is possible.
    #[must_use]
    pub const fn new(app_time: Duration, input: &'a Input<'a>) -> Self {
        Self {
            app_time,
            input: Some(input),
        }
    }

    // Input should always be provided, `as_drawable` is probably fine to not blur subtrees?
    #[must_use]
    pub(crate) const fn non_animated() -> Self {
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
