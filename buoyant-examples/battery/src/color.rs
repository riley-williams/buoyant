//! Color pixel formats and theme colors

use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

/// The color format used by the display
pub type ColorFormat = Rgb888;

pub const GREEN: ColorFormat = Rgb888::new(20, 200, 50);
pub const RED: ColorFormat = Rgb888::new(255, 0, 0);
pub const YELLOW: ColorFormat = Rgb888::new(255, 255, 0);
pub const BLUE: ColorFormat = Rgb888::new(100, 210, 255);
pub const BLACK: ColorFormat = Rgb888::new(0, 0, 0);
pub const GREY: ColorFormat = Rgb888::new(150, 150, 150);
pub const WHITE: ColorFormat = Rgb888::WHITE;

pub const BACKGROUND: ColorFormat = BLACK;
pub const SECONDARY_BACKGROUND: ColorFormat = Rgb888::new(50, 50, 50);
pub const CONTENT: ColorFormat = Rgb888::new(255, 255, 255);
pub const SECONDARY_CONTENT: ColorFormat = Rgb888::new(200, 200, 200);
