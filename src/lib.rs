#![no_std]
#![feature(type_alias_impl_trait)]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod pixel;
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
    fn duration(&self) -> core::time::Duration {
        match self {
            Animation::Linear(duration) => *duration,
        }
    }

    fn with_duration(self, duration: core::time::Duration) -> Self {
        match self {
            Animation::Linear(_) => Animation::Linear(duration),
        }
    }
}
