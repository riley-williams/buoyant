use crate::primitives::Point;

pub struct Pixel<C> {
    pub point: Point,
    pub color: C,
}

impl<T: embedded_graphics_core::pixelcolor::PixelColor> From<Pixel<T>>
    for embedded_graphics_core::Pixel<T>
{
    fn from(value: Pixel<T>) -> Self {
        embedded_graphics_core::Pixel(value.point.into(), value.color)
    }
}

impl<T: embedded_graphics_core::pixelcolor::PixelColor> From<embedded_graphics_core::Pixel<T>>
    for Pixel<T>
{
    fn from(value: embedded_graphics_core::Pixel<T>) -> Self {
        Pixel {
            point: value.0.into(),
            color: value.1,
        }
    }
}

pub trait Interpolate: Copy + PartialEq {
    /// Interpolate between two colors
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount < 127 {
            from
        } else {
            to
        }
    }
}

impl Interpolate for u16 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((amount as u32 * to as u32) + ((255 - amount) as u32 * from as u32)) / 255) as u16
    }
}

impl Interpolate for i16 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((amount as i32 * to as i32) + ((255 - amount) as i32 * from as i32)) / 255) as i16
    }
}

// TODO: This isn't correct...close enough for now
impl Interpolate for u32 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        ((amount as u32 * to) + ((255 - amount) as u32 * from)) / 255
    }
}

#[cfg(feature = "crossterm")]
impl Interpolate for crossterm::style::Colors {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let foreground = interpolate_crossterm_colors(from.foreground, to.foreground, amount);
        let background = interpolate_crossterm_colors(from.background, to.background, amount);

        crossterm::style::Colors {
            foreground,
            background,
        }
    }
}

#[cfg(feature = "crossterm")]
fn interpolate_crossterm_colors(
    from: Option<crossterm::style::Color>,
    to: Option<crossterm::style::Color>,
    amount: u8,
) -> Option<crossterm::style::Color> {
    let mut amount = amount as f32 / 255.0;
    amount = amount.clamp(0.0, 1.0);
    let inverse_amount = 1.0 - amount;
    match (from, to) {
        (
            Some(crossterm::style::Color::Rgb {
                r: r1,
                g: g1,
                b: b1,
            }),
            Some(crossterm::style::Color::Rgb {
                r: r2,
                g: g2,
                b: b2,
            }),
        ) => Some(crossterm::style::Color::Rgb {
            r: (r1 as f32 * inverse_amount + r2 as f32 * amount) as u8,
            g: (g1 as f32 * inverse_amount + g2 as f32 * amount) as u8,
            b: (b1 as f32 * inverse_amount + b2 as f32 * amount) as u8,
        }),
        (Some(c1), Some(c2)) => {
            if amount < 0.5 {
                Some(c1)
            } else {
                Some(c2)
            }
        }
        (Some(c1), None) => Some(c1),
        (None, Some(c2)) => Some(c2),
        _ => None,
    }
}

impl Interpolate for embedded_graphics_core::pixelcolor::BinaryColor {}

use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics_core::pixelcolor::{Rgb565, RgbColor};

impl Interpolate for embedded_graphics_core::pixelcolor::Rgb565 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let t_fixed = amount as i16;

        let r = interpolate_channel(from.r(), to.r(), t_fixed);
        let g = interpolate_channel(from.g(), to.g(), t_fixed);
        let b = interpolate_channel(from.b(), to.b(), t_fixed);
        Rgb565::new(r, g, b)
    }
}

impl<C: embedded_graphics::prelude::PixelColor + Interpolate> Interpolate for PrimitiveStyle<C> {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let mut style = embedded_graphics::primitives::PrimitiveStyleBuilder::new();
        style = match (from.fill_color, to.fill_color) {
            (Some(from), Some(to)) => style.fill_color(C::interpolate(from, to, amount)),
            (Some(from), None) => style.fill_color(from),
            (None, Some(to)) => style.fill_color(to),
            (None, None) => style,
        };

        style = match (from.stroke_color, to.stroke_color) {
            (Some(from), Some(to)) => style.stroke_color(C::interpolate(from, to, amount)),
            (Some(from), None) => style.stroke_color(from),
            (None, Some(to)) => style.stroke_color(to),
            (None, None) => style,
        };

        style.build()
    }
}

#[inline]
/// Interpolate between two colors, using a u16 between 0 and 256
fn interpolate_channel(a: u8, b: u8, t: i16) -> u8 {
    (a as i16 + (((b as i16).wrapping_sub(a as i16)).wrapping_mul(t) as u16 >> 8) as i16) as u8
}

#[cfg(test)]
mod tests {
    use embedded_graphics_core::pixelcolor::Rgb565;

    use super::Interpolate;

    #[ignore]
    #[test]
    fn interpolate_rgb() {
        let start = Rgb565::new(0, 30, 10);
        let end = Rgb565::new(10, 20, 20);
        assert_eq!(Rgb565::interpolate(start, end, 0), start);
        assert_eq!(Rgb565::interpolate(start, end, 128), Rgb565::new(5, 25, 15));
        // TODO: Fix interpolation
        assert_eq!(Rgb565::interpolate(start, end, 255), end);
    }
}
