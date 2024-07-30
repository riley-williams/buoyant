#![cfg_attr(not(feature = "std"), no_std)]
#![feature(type_alias_impl_trait)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

pub mod environment;
pub mod font;
pub mod layout;
pub mod pixel;
pub mod primitives;
pub mod render;
pub mod render_target;
pub mod view;
