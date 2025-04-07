//! Color pixel formats and theme colors
//!
//! The display color format can be configured by changing the feature.
//!
//! You'll notice the colors don't look right when switching to rgb565.
//! This is partially because the rgb order changes in `::new()`, but
//! also has to do with how `::new` clips values.
//!

use embedded_graphics::prelude::RgbColor;

/// The color format used by the display
#[cfg(feature = "rgb888")]
pub type ColorFormat = embedded_graphics::pixelcolor::Rgb888;

#[cfg(all(feature = "rgb565", not(feature = "rgb888")))]
pub type ColorFormat = embedded_graphics::pixelcolor::Rgb565;

pub const GREEN: ColorFormat = ColorFormat::new(20, 200, 50);
pub const RED: ColorFormat = ColorFormat::new(255, 0, 0);
pub const YELLOW: ColorFormat = ColorFormat::new(255, 255, 0);
pub const BLUE: ColorFormat = ColorFormat::new(100, 210, 255);
pub const BLACK: ColorFormat = ColorFormat::new(0, 0, 0);
pub const GREY: ColorFormat = ColorFormat::new(150, 150, 150);
pub const WHITE: ColorFormat = ColorFormat::WHITE;

pub const BACKGROUND: ColorFormat = BLACK;
pub const SECONDARY_BACKGROUND: ColorFormat = ColorFormat::new(50, 50, 50);
pub const CONTENT: ColorFormat = WHITE;
pub const SECONDARY_CONTENT: ColorFormat = ColorFormat::new(200, 200, 200);
