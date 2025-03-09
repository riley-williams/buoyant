#![no_std]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_const_for_fn)]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod primitives;
pub mod render;
pub mod render_target;
// #[warn(missing_docs)]
pub mod animation;
pub mod view;
