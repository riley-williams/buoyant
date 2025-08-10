#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod animation;
pub mod color;
pub mod environment;
pub mod event;
pub mod font;
pub mod image;
pub mod layout;
pub mod primitives;
pub mod render;
pub mod render_loop;
pub mod render_target;
pub mod surface;
pub mod transition;
#[warn(missing_docs)]
pub mod view;
