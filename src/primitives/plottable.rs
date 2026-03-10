/// A value that can be used as a chart coordinate.
///
/// Types implementing this trait can be used as x or y values in chart marks.
/// Internally, chart coordinates are stored as `i32` to minimize memory usage.
pub trait Plottable: Copy + PartialOrd {
    /// Converts this value to an `i32` chart coordinate.
    fn as_i32(self) -> i32;
}

impl Plottable for i8 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}

impl Plottable for i16 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}

impl Plottable for i32 {
    fn as_i32(self) -> i32 {
        self
    }
}

impl Plottable for u8 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}

impl Plottable for u16 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}

impl Plottable for u32 {
    fn as_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i8_plottable() {
        assert_eq!((-128i8).as_i32(), -128);
        assert_eq!(127i8.as_i32(), 127);
    }

    #[test]
    fn i16_plottable() {
        assert_eq!((-1000i16).as_i32(), -1000);
        assert_eq!(1000i16.as_i32(), 1000);
    }

    #[test]
    fn i32_plottable() {
        assert_eq!((-1_000_000i32).as_i32(), -1_000_000);
        assert_eq!(1_000_000i32.as_i32(), 1_000_000);
    }

    #[test]
    fn u8_plottable() {
        assert_eq!(0u8.as_i32(), 0);
        assert_eq!(255u8.as_i32(), 255);
    }

    #[test]
    fn u16_plottable() {
        assert_eq!(0u16.as_i32(), 0);
        assert_eq!(65535u16.as_i32(), 65535);
    }

    #[test]
    fn u32_plottable() {
        assert_eq!(0u32.as_i32(), 0);
        assert_eq!(1000u32.as_i32(), 1000);
    }
}
