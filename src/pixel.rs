use crate::primitives::Point;

pub struct Pixel<C> {
    pub point: Point,
    pub color: C,
}

#[cfg(feature = "embedded-graphics")]
impl<T: embedded_graphics_core::pixelcolor::PixelColor> From<Pixel<T>>
    for embedded_graphics_core::Pixel<T>
{
    fn from(value: Pixel<T>) -> Self {
        embedded_graphics_core::Pixel(value.point.into(), value.color)
    }
}

#[cfg(feature = "embedded-graphics")]
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
        (((u32::from(amount) * u32::from(to)) + (u32::from(255 - amount) * u32::from(from))) / 255) as u16
    }
}

impl Interpolate for i16 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((i32::from(amount) * i32::from(to)) + (i32::from(255 - amount) * i32::from(from))) / 255) as i16
    }
}

// TODO: This isn't correct...close enough for now
impl Interpolate for u32 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        ((u32::from(amount) * to) + (u32::from(255 - amount) * from)) / 255
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
    let mut amount = f32::from(amount) / 255.0;
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
            r: (f32::from(r1) * inverse_amount + f32::from(r2) * amount) as u8,
            g: (f32::from(g1) * inverse_amount + f32::from(g2) * amount) as u8,
            b: (f32::from(b1) * inverse_amount + f32::from(b2) * amount) as u8,
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
mod embedded_graphics_impl {

    use super::Interpolate;
    use embedded_graphics::primitives::PrimitiveStyle;
    use embedded_graphics_core::pixelcolor::{Rgb565, RgbColor};

    impl Interpolate for embedded_graphics_core::pixelcolor::BinaryColor {}

    impl Interpolate for embedded_graphics_core::pixelcolor::Rgb565 {
        fn interpolate(from: Self, to: Self, amount: u8) -> Self {
            if amount == 255 {
                return to;
            }
            let t_fixed = i16::from(amount);

            let r = interpolate_channel(from.r(), to.r(), t_fixed);
            let g = interpolate_channel(from.g(), to.g(), t_fixed);
            let b = interpolate_channel(from.b(), to.b(), t_fixed);
            Rgb565::new(r, g, b)
        }
    }

    #[inline]
    /// Interpolate between two colors, using a u16 between 0 and 256
    fn interpolate_channel(a: u8, b: u8, t: i16) -> u8 {
        (i16::from(a) + ((i16::from(b).wrapping_sub(i16::from(a))).wrapping_mul(t) as u16 >> 8) as i16) as u8
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
}

#[cfg(all(test, feature = "embedded-graphics"))]
mod tests {
    use embedded_graphics_core::pixelcolor::Rgb565;

    use super::Interpolate;

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
