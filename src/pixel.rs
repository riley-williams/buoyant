pub trait ColorValue: Clone + Copy + PartialEq {
    /// Interpolate between two colors
    fn interpolate(from: Self, to: Self, amount: f32) -> Self;
}

impl ColorValue for char {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        if amount < 0.5 {
            from
        } else {
            to
        }
    }
}

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
impl ColorValue for CrosstermColorSymbol {
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
impl ColorValue for embedded_graphics::pixelcolor::BinaryColor {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        if amount < 0.5 {
            from
        } else {
            to
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

#[cfg(feature = "embedded-graphics")]
impl ColorValue for embedded_graphics::pixelcolor::Rgb565 {
    fn interpolate(from: Self, to: Self, amount: f32) -> Self {
        let x = ((amount * 255.0) as u32).clamp(0, 255) as u16;
        let r = from.r() as u16 * x + to.r() as u16 * (255 - x);
        let g = from.g() as u16 * x + to.g() as u16 * (255 - x);
        let b = from.b() as u16 * x + to.b() as u16 * (255 - x);
        Rgb565::new((r / 255) as u8, (g / 255) as u8, (b / 255) as u8)
    }
}

#[cfg(feature = "embedded-graphics")]
#[cfg(test)]
mod tests {
    use embedded_graphics::pixelcolor::Rgb565;

    use super::ColorValue;

    #[test]
    fn interpolate_rgb() {
        let start = Rgb565::new(0, 30, 100);
        let end = Rgb565::new(10, 20, 200);
        assert_eq!(
            Rgb565::interpolate(start, end, 0.0),
            Rgb565::new(0, 30, 100)
        );
        assert_eq!(
            Rgb565::interpolate(start, end, 0.1),
            Rgb565::new(1, 29, 110)
        );
        assert_eq!(
            Rgb565::interpolate(start, end, 1.0),
            Rgb565::new(10, 20, 200)
        );
    }
}
