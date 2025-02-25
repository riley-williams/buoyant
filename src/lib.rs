#![no_std]
#![allow(clippy::cast_sign_loss, clippy::missing_panics_doc)]
#![deny(clippy::missing_const_for_fn)]
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
            Animation::Linear(duration) => *duration,
        }
    }

    const fn with_duration(self, duration: core::time::Duration) -> Self {
        match self {
            Animation::Linear(_) => Animation::Linear(duration),
        }
    }
}
