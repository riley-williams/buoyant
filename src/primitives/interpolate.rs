pub trait Interpolate: Sized + PartialEq {
    /// Interpolate between two colors
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount < 127 {
            from
        } else {
            to
        }
    }
}

impl Interpolate for () {
    fn interpolate(_from: Self, _to: Self, _amount: u8) -> Self {}
}

impl Interpolate for u16 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((u32::from(amount) * u32::from(to)) + (u32::from(255 - amount) * u32::from(from))) / 255)
            as Self
    }
}

impl Interpolate for i16 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((i32::from(amount) * i32::from(to)) + (i32::from(255 - amount) * i32::from(from))) / 255)
            as Self
    }
}

// TODO: This isn't correct...close enough for now
impl Interpolate for u32 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        ((Self::from(amount) * to) + (Self::from(255 - amount) * from)) / 255
    }
}

impl Interpolate for f32 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let amount = Self::from(amount) / 255.0;
        from * (1.0 - amount) + to * amount
    }
}

impl Interpolate for f64 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let amount = Self::from(amount) / 255.0;
        from * (1.0 - amount) + to * amount
    }
}

impl Interpolate for char {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount < 127 {
            from
        } else {
            to
        }
    }
}

impl Interpolate for fixed::types::U18F14 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        to * (Self::from(amount) / 255) + from * (Self::from(255 - amount) / 255)
    }
}

impl Interpolate for fixed::types::I18F14 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        from + (to - from) * (Self::from(amount) / 255)
    }
}

#[cfg(feature = "crossterm")]
impl Interpolate for crossterm::style::Colors {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let foreground = interpolate_crossterm_colors(from.foreground, to.foreground, amount);
        let background = interpolate_crossterm_colors(from.background, to.background, amount);

        Self {
            foreground,
            background,
        }
    }
}

#[cfg(feature = "crossterm")]
#[allow(clippy::cast_sign_loss)]
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
    use embedded_graphics_core::pixelcolor::RgbColor;

    macro_rules! impl_interpolate_for_pixelcolor {
        ($color_type:ty) => {
            impl Interpolate for $color_type {
                fn interpolate(from: Self, to: Self, amount: u8) -> Self {
                    if amount == 255 {
                        return to;
                    }
                    let t_fixed = i16::from(amount);

                    let r = interpolate_channel(from.r(), to.r(), t_fixed);
                    let g = interpolate_channel(from.g(), to.g(), t_fixed);
                    let b = interpolate_channel(from.b(), to.b(), t_fixed);
                    <$color_type>::new(r, g, b)
                }
            }
        };
    }

    impl_interpolate_for_pixelcolor!(embedded_graphics_core::pixelcolor::Rgb555);
    impl_interpolate_for_pixelcolor!(embedded_graphics_core::pixelcolor::Rgb565);
    impl_interpolate_for_pixelcolor!(embedded_graphics_core::pixelcolor::Rgb666);
    impl_interpolate_for_pixelcolor!(embedded_graphics_core::pixelcolor::Rgb888);

    impl Interpolate for embedded_graphics_core::pixelcolor::BinaryColor {}

    /// Interpolate between two colors, using an i16 between 0 and 256
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn interpolate_channel(a: u8, b: u8, t: i16) -> u8 {
        (i16::from(a).saturating_add(
            ((i16::from(b).wrapping_sub(i16::from(a))).wrapping_mul(t) as u16 >> 8) as i16,
        )) as u8
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

#[cfg(test)]
mod tests {
    use super::Interpolate;
    use fixed::types::{I18F14, U18F14};
    use paste::paste;

    fn fp_interpolate<T: fixed::traits::Fixed>(start: T, end: T, amount: f32) -> T {
        T::from_num(
            start.to_num::<f32>() + ((end.to_num::<f32>() - start.to_num::<f32>()) * amount),
        )
    }

    macro_rules! test_fixed_i_interpolate_approx_fp {
        ($type:ty) => {
            paste! {
                #[test]
                #[expect(non_snake_case)]
                fn [<interpolate_ $type _approximates_fp>]() {
                    for start in [$type::from_num(-100.123), $type::from_num(0.0), $type::from_num(12.0), $type::from_num(87654)] {
                        for end in [$type::from_num(-6644.7), $type::from_num(22.3), $type::from_num(0.0), $type::from_num(1.0)] {
                            for amount in 0u8..=255u8 {
                                let precision: f32 = (start.to_num::<f32>() - end.to_num::<f32>()).abs() / 2.0f32.powf(8.0);
                                let expected = fp_interpolate(start, end, f32::from(amount) / 255.0);
                                let interpolation = <$type>::interpolate(start, end, amount);
                                let diff = interpolation.abs_diff(expected);
                                assert!(diff <= precision,
                                    "Interpolation of {} and {} at {} ({}): expected {}, got {}, diff: {} (max dev. {})",
                                    start, end, amount, f32::from(amount) / 255.0, expected, interpolation, diff, precision
                                );
                            }
                            // The start and end should be exact
                            assert_eq!(<$type>::interpolate(start, end, 0), start);
                            assert_eq!(<$type>::interpolate(start, end, 255), end);
                        }
                    }
                }
            }
        };
    }

    macro_rules! test_fixed_u_interpolate_approx_fp {
        ($type:ty) => {
            paste! {
                #[test]
                #[expect(non_snake_case)]
                fn [<interpolate_ $type _approximates_fp>]() {
                    for start in [$type::from_num(100.123), $type::from_num(0.0), $type::from_num(12.0), $type::from_num(87654)] {
                        for end in [$type::from_num(6644.7), $type::from_num(22.3), $type::from_num(0.0), $type::from_num(1.0)] {
                            for amount in 0u8..=255u8 {
                                let precision: f32 = (start.to_num::<f32>() - end.to_num::<f32>()).abs() / 2.0f32.powf(8.0);
                                let expected = fp_interpolate(start, end, f32::from(amount) / 255.0);
                                let interpolation = <$type>::interpolate(start, end, amount);
                                let diff = interpolation.abs_diff(expected);
                                assert!(diff <= precision,
                                    "Interpolation of {} and {} at {} ({}): expected {}, got {}, diff: {} (max dev. {})",
                                    start, end, amount, f32::from(amount) / 255.0, expected, interpolation, diff, precision
                                );
                            }
                            // The start and end should be exact
                            assert_eq!(<$type>::interpolate(start, end, 0), start);
                            assert_eq!(<$type>::interpolate(start, end, 255), end);
                        }
                    }
                }
            }
        };
    }

    test_fixed_u_interpolate_approx_fp!(U18F14);
    test_fixed_i_interpolate_approx_fp!(I18F14);

    macro_rules! test_fp_ends {
        ($type:ty) => {
            paste! {
                #[test]
                #[expect(clippy::float_cmp, reason = "Should match exactly at start and end")]
                fn [<interpolate_ $type _ends>]() {
                    for start in [100.123, 0.0, 12.0, 87654.0] {
                        for end in [6644.7, 22.3, 0.0, 1.0] {
                            // The start and end should exactly match
                            assert_eq!(<$type>::interpolate(start, end, 0), start);
                            assert_eq!(<$type>::interpolate(start, end, 255), end);
                        }
                    }
                }
            }
        };
    }

    test_fp_ends!(f32);
    test_fp_ends!(f64);
}

#[cfg(all(test, feature = "embedded-graphics"))]
mod rgb_tests {
    use super::Interpolate;
    use embedded_graphics::{
        pixelcolor::{Rgb555, Rgb565, Rgb666, Rgb888},
        prelude::RgbColor,
    };
    use paste::paste;

    #[test]
    fn interpolate_rgb() {
        let start = Rgb565::new(0, 30, 10);
        let end = Rgb565::new(10, 20, 20);
        assert_eq!(Rgb565::interpolate(start, end, 0), start);
        assert_eq!(Rgb565::interpolate(start, end, 128), Rgb565::new(5, 25, 15));
        assert_eq!(Rgb565::interpolate(start, end, 255), end);
    }

    /// Interpolate using trivially correct floating point math
    #[expect(clippy::cast_sign_loss)]
    fn fp_interpolate<T: RgbColor>(start: T, end: T, amount: f32) -> (u8, u8, u8) {
        let r = (f32::from(start.r()) * (1.0 - amount) + f32::from(end.r()) * amount) as u8;
        let g = (f32::from(start.g()) * (1.0 - amount) + f32::from(end.g()) * amount) as u8;
        let b = (f32::from(start.b()) * (1.0 - amount) + f32::from(end.b()) * amount) as u8;
        (r, g, b)
    }

    macro_rules! test_rgb_interpolate_approx_fp {
        ($type:ty, $start:expr, $end:expr) => {
            paste! {
                #[test]
                #[expect(non_snake_case)]
                fn [<interpolate_ $type _approximates_fp>]() {
                    for amount in 0u8..=255u8 {
                        let expected = fp_interpolate($start, $end, f32::from(amount) / 255.0);
                        let interpolation = <$type>::interpolate($start, $end, amount);
                        assert!(interpolation.r().abs_diff(expected.0) <= 1);
                        assert!(interpolation.g().abs_diff(expected.1) <= 1);
                        assert!(interpolation.b().abs_diff(expected.2) <= 1);
                    }
                }
            }
        };
    }

    test_rgb_interpolate_approx_fp!(Rgb555, Rgb555::new(0, 255, 127), Rgb555::new(255, 0, 200));
    test_rgb_interpolate_approx_fp!(Rgb565, Rgb565::new(0, 255, 127), Rgb565::new(255, 0, 200));
    test_rgb_interpolate_approx_fp!(Rgb666, Rgb666::new(0, 255, 127), Rgb666::new(255, 0, 200));
    test_rgb_interpolate_approx_fp!(Rgb888, Rgb888::new(0, 255, 127), Rgb888::new(255, 0, 200));

    macro_rules! test_interpolate_start_end_match {
        ($type:ty) => {
            paste! {
                #[test]
                #[expect(non_snake_case)]
                fn [<interpolate_ $type _fills_range>]() {
                    let start = <$type>::BLACK;
                    let end = <$type>::WHITE;
                    let interpolation = <$type>::interpolate(start, end, 0);
                    assert_eq!(interpolation, start);

                    let interpolation = <$type>::interpolate(start, end, 255);
                    assert_eq!(interpolation, end);
                }
            }
        };
    }

    test_interpolate_start_end_match!(Rgb555);
    test_interpolate_start_end_match!(Rgb565);
    test_interpolate_start_end_match!(Rgb666);
    test_interpolate_start_end_match!(Rgb888);
}
