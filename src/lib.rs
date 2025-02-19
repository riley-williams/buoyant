#![no_std]
#![feature(type_alias_impl_trait)]
#![warn(missing_debug_implementations)]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod primitives;
pub mod render;
pub mod render_target;
#[warn(missing_docs, rustdoc::missing_doc_code_examples)]
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
