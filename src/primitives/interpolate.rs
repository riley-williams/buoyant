use fixed_macro::fixed;

pub trait Interpolate: Sized + PartialEq {
    /// Interpolate between two colors
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount < 127 { from } else { to }
    }
}

impl Interpolate for () {
    fn interpolate(_from: Self, _to: Self, _amount: u8) -> Self {}
}

impl Interpolate for u8 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (((u16::from(amount) * u16::from(to)) + (u16::from(255 - amount) * u16::from(from))) / 255)
            as Self
    }
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

impl<A: Interpolate, B: Interpolate> Interpolate for (A, B) {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        (
            A::interpolate(from.0, to.0, amount),
            B::interpolate(from.1, to.1, amount),
        )
    }
}

impl Interpolate for char {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        if amount < 127 { from } else { to }
    }
}

impl Interpolate for fixed::types::U18F14 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        let factor = Self::from(amount) / 255;
        to * factor + from * (fixed!(1:U18F14) - factor)
    }
}

impl Interpolate for fixed::types::I18F14 {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        from + (to - from) * (Self::from(amount) / 255)
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
                    for start in [$type::from_num(-100.123), $type::from_num(0.0), $type::from_num(1.0), $type::from_num(12.0), $type::from_num(87654)] {
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
                    for start in [$type::from_num(100.123), $type::from_num(0.0), $type::from_num(1.0), $type::from_num(12.0), $type::from_num(87654)] {
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

    #[expect(clippy::cast_precision_loss)]
    fn fp_interpolate_int<T>(start: T, end: T, amount: f64) -> i64
    where
        T: Into<i64> + Copy,
    {
        let start_f = start.into() as f64;
        let end_f = end.into() as f64;
        let result = start_f * (1.0 - amount) + end_f * amount;
        result.round() as i64
    }

    macro_rules! test_integer_interpolate {
        ($type:ty, $test_values:expr, $max_error:expr) => {
            paste! {
                #[test]
                fn [<interpolate_ $type _approximates_fp>]() {
                    let test_values: &[$type] = $test_values;
                    for &start in test_values {
                        for &end in test_values {
                            for amount in 0u8..=255u8 {
                                let expected = fp_interpolate_int(start, end, f64::from(amount) / 255.0) as $type;
                                let interpolation = <$type>::interpolate(start, end, amount);
                                let diff = if interpolation >= expected {
                                    interpolation - expected
                                } else {
                                    expected - interpolation
                                };
                                assert!(
                                    diff <= $max_error,
                                    "Interpolation of {} and {} at {} ({}): expected {}, got {}, diff: {}",
                                    start, end, amount, f32::from(amount) / 255.0, expected, interpolation, diff
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

    test_integer_interpolate!(u8, &[0, 1, 50, 100, 200, 255], 1);
    test_integer_interpolate!(u16, &[0, 1, 100, 1000, 10000, 32767, 65535], 2);
    test_integer_interpolate!(i16, &[-32767, -1000, -1, 0, 1, 1000, 32767], 2);
    test_integer_interpolate!(
        u32,
        &[0, 1, 100, 1000, 100_000, 1_000_000, 16_777_216],
        1000
    );
}
