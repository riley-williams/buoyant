#![no_std]
#![cfg_attr(test, allow(unused))]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod primitives;
pub mod render;
pub mod render_target;
pub mod view;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Animation {
    Linear(core::time::Duration),
}

impl Animation {
    const fn duration(&self) -> core::time::Duration {
        match self {
            Self::Linear(duration) => *duration,
        }
    }

    const fn with_duration(self, duration: core::time::Duration) -> Self {
        match self {
            Self::Linear(_) => Self::Linear(duration),
        }
    }
}
