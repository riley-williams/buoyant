//! View modifiers are used to alter the layout and rendering of views.
//!
//! These are not intended to be used directly, but rather via the methods in [`ViewExt`]
//!
//! [`ViewExt`]: crate::view::ViewExt

mod animated;
#[allow(missing_docs)]
pub mod aspect_ratio;
mod background;
mod erase_captures;
mod fixed_frame;
mod fixed_size;
mod flex_frame;
mod foreground_color;
mod geometry_group;
mod hidden;
mod offset;
mod overlay;
#[allow(missing_docs)]
pub mod padding;
mod priority;

pub(crate) use animated::Animated;
pub(crate) use aspect_ratio::AspectRatio;
pub(crate) use background::BackgroundView;
pub(crate) use erase_captures::EraseCaptures;
pub(crate) use fixed_frame::FixedFrame;
pub(crate) use fixed_size::FixedSize;
pub(crate) use flex_frame::FlexFrame;
pub(crate) use foreground_color::ForegroundStyle;
pub(crate) use geometry_group::GeometryGroup;
pub(crate) use hidden::Hidden;
pub(crate) use offset::Offset;
pub(crate) use overlay::OverlayView;
pub(crate) use padding::Padding;
pub(crate) use priority::Priority;
