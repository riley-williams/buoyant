#![no_std]
#![warn(missing_debug_implementations)]
#![deny(clippy::missing_const_for_fn)]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod primitives;
pub mod render;
pub mod render_target;
#[warn(missing_docs)]
pub mod view;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Animation {
    Linear(core::time::Duration),
}

impl Animation {
    const fn duration(&self) -> core::time::Duration {
        match self {
            Animation::Linear(duration) => *duration,
        }
    }

    const fn with_duration(self, duration: core::time::Duration) -> Self {
        match self {
            Animation::Linear(_) => Animation::Linear(duration),
        }
    }
}
