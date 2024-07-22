use crate::primitives::Point;

pub struct Pixel<C: PixelColor> {
    pub point: Point,
    pub color: C,
}

#[cfg(feature = "embedded-graphics")]
impl<T: embedded_graphics_core::pixelcolor::PixelColor + PixelColor> From<Pixel<T>>
    for embedded_graphics_core::Pixel<T>
{
    fn from(value: Pixel<T>) -> Self {
        embedded_graphics_core::Pixel(value.point.into(), value.color)
    }
}

#[cfg(feature = "embedded-graphics")]
impl<T: embedded_graphics_core::pixelcolor::PixelColor + PixelColor>
    From<embedded_graphics_core::Pixel<T>> for Pixel<T>
{
    fn from(value: embedded_graphics_core::Pixel<T>) -> Self {
        Pixel {
            point: value.0.into(),
            color: value.1,
        }
    }
}

pub trait PixelColor: Clone + Copy + PartialEq {
    /// Interpolate between two colors
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        if amount < 0.5 {
            from
        } else {
            to
        }
    }
}

impl PixelColor for char {}
impl PixelColor for () {}

#[cfg(feature = "crossterm")]
impl PixelColor for crossterm::style::Colors {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
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
    mut amount: f32,
) -> Option<crossterm::style::Color> {
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

#[cfg(feature = "embedded-graphics")]
impl PixelColor for embedded_graphics_core::pixelcolor::BinaryColor {}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics_core::pixelcolor::{Rgb565, RgbColor};

#[cfg(feature = "embedded-graphics")]
impl PixelColor for embedded_graphics_core::pixelcolor::Rgb565 {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        let t_fixed = (amount * 256.0) as i16;

        let r = interpolate_channel(from.r(), to.r(), t_fixed);
        let g = interpolate_channel(from.g(), to.g(), t_fixed);
        let b = interpolate_channel(from.b(), to.b(), t_fixed);
        Rgb565::new(r, g, b)
    }
}

#[cfg(feature = "embedded-graphics")]
#[inline]
/// Interpolate between two colors, using a u16 between 0 and 256
fn interpolate_channel(a: u8, b: u8, t: i16) -> u8 {
    (a as i16 + (((b as i16).wrapping_sub(a as i16)).wrapping_mul(t) as u16 >> 8) as i16) as u8
}

#[cfg(feature = "embedded-graphics")]
#[cfg(test)]
mod tests {
    use embedded_graphics_core::pixelcolor::Rgb565;

    use super::PixelColor;

    #[test]
    fn interpolate_rgb() {
        let start = Rgb565::new(0, 30, 10);
        let end = Rgb565::new(10, 20, 20);
        assert_eq!(Rgb565::interpolate(start, end, 0.0), start);
        assert_eq!(Rgb565::interpolate(start, end, 0.5), Rgb565::new(5, 25, 15));
        assert_eq!(Rgb565::interpolate(start, end, 1.0), end);
    }
}
