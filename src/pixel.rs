use crate::primitives::Point;

pub struct Pixel<C: PixelColor> {
    pub point: Point,
    pub color: C,
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

#[cfg(feature = "crossterm")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrosstermColorSymbol {
    pub character: char,
    pub colors: crossterm::style::Colors,
}

#[cfg(feature = "crossterm")]
impl CrosstermColorSymbol {
    pub fn new(character: char) -> Self {
        CrosstermColorSymbol {
            character,
            colors: crossterm::style::Colors {
                foreground: None,
                background: None,
            },
        }
    }

    pub fn with_foreground(mut self, color: crossterm::style::Color) -> Self {
        self.colors.foreground = Some(color);
        self
    }

    pub fn with_background(mut self, color: crossterm::style::Color) -> Self {
        self.colors.background = Some(color);
        self
    }
}

#[cfg(feature = "crossterm")]
use crossterm::style::Stylize;

#[cfg(feature = "crossterm")]
impl From<CrosstermColorSymbol> for crossterm::style::StyledContent<char> {
    fn from(value: CrosstermColorSymbol) -> Self {
        if let Some(fg) = value.colors.foreground {
            if let Some(bg) = value.colors.background {
                value.character.with(fg).on(bg)
            } else {
                value.character.with(fg)
            }
        } else {
            value.character.stylize()
        }
    }
}

#[cfg(feature = "crossterm")]
impl PixelColor for CrosstermColorSymbol {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        let interpolated_char = if amount < 0.5 {
            from.character
        } else {
            to.character
        };

        let foreground =
            interpolate_crossterm_colors(from.colors.foreground, to.colors.foreground, amount);
        let background =
            interpolate_crossterm_colors(from.colors.background, to.colors.background, amount);

        CrosstermColorSymbol {
            character: interpolated_char,
            colors: crossterm::style::Colors {
                foreground,
                background,
            },
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
impl PixelColor for embedded_graphics::pixelcolor::BinaryColor {}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

#[cfg(feature = "embedded-graphics")]
impl PixelColor for embedded_graphics::pixelcolor::Rgb565 {
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
    use embedded_graphics::pixelcolor::Rgb565;

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
