use crate::primitives::Interpolate;

/// A color with an alpha channel.
pub trait AlphaColor {
    /// Returns the alpha value of the color.
    /// 255 is fully opaque, 0 is fully transparent.
    fn alpha(&self) -> u8 {
        255
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba<C> {
    color: C,
    alpha: u8,
}

impl<C> AlphaColor for Rgba<C> {
    fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl<C: Interpolate> Interpolate for Rgba<C> {
    fn interpolate(from: Self, to: Self, amount: u8) -> Self {
        // FIXME: lazy cast
        Self {
            color: Interpolate::interpolate(from.color, to.color, amount),
            alpha: Interpolate::interpolate(u32::from(from.alpha), u32::from(to.alpha), amount)
                as u8,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics {
    use embedded_graphics::pixelcolor::{
        BinaryColor, Gray2, Gray4, Gray8, Rgb555, Rgb565, Rgb666, Rgb888,
    };

    use crate::color::{AlphaColor, Rgba};

    impl AlphaColor for Rgb888 {}
    impl From<Rgba<Self>> for Rgb888 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Rgb666 {}
    impl From<Rgba<Self>> for Rgb666 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Rgb565 {}
    impl From<Rgba<Self>> for Rgb565 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Rgb555 {}
    impl From<Rgba<Self>> for Rgb555 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for BinaryColor {}
    impl From<Rgba<Self>> for BinaryColor {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Gray2 {}
    impl From<Rgba<Self>> for Gray2 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Gray4 {}
    impl From<Rgba<Self>> for Gray4 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }

    impl AlphaColor for Gray8 {}
    impl From<Rgba<Self>> for Gray8 {
        fn from(value: Rgba<Self>) -> Self {
            value.color
        }
    }
}
