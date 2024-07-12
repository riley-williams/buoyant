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

impl ColorValue for rgb::RGB8 {
    fn interpolate(from: Self, to: Self, mut amount: f32) -> Self {
        amount = amount.clamp(0.0, 1.0);
        let inverse_amount = 1.0 - amount;
        let r = (from.r as f32 * inverse_amount + to.r as f32 * amount) as u8;
        let g = (from.g as f32 * inverse_amount + to.g as f32 * amount) as u8;
        let b = (from.b as f32 * inverse_amount + to.b as f32 * amount) as u8;
        rgb::RGB8 { r, g, b }
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
