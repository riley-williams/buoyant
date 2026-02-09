//! Simple counter view for basic workflow testing.

use buoyant::view::prelude::*;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use serde::Serialize;

/// Colors shared with other view modules.
pub mod color {
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::{RgbColor, WebColors};

    pub const FOREGROUND: Rgb888 = Rgb888::WHITE;
    pub const ACCENT: Rgb888 = Rgb888::CSS_LIGHT_SKY_BLUE;
    pub const SECONDARY: Rgb888 = Rgb888::CSS_DIM_GRAY;
}

/// State for the simple counter view.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CounterState {
    pub value: i32,
}

/// A simple counter view with increment and decrement buttons.
pub fn counter_view(state: &CounterState) -> impl View<Rgb888, CounterState> + use<> {
    let value_str: &'static str = Box::leak(format!("{}", state.value).into_boxed_str());

    VStack::new((
        Text::new("Simple Counter", &FONT_10X20).foreground_color(color::ACCENT),
        HStack::new((
            Button::new(
                |s: &mut CounterState| s.value -= 1,
                |_| {
                    Text::new("-", &FONT_10X20)
                        .padding(Edges::All, 12)
                        .background_color(color::SECONDARY, Rectangle)
                },
            ),
            Text::new(value_str, &FONT_10X20)
                .foreground_color(color::FOREGROUND)
                .padding(Edges::Horizontal, 20),
            Button::new(
                |s: &mut CounterState| s.value += 1,
                |_| {
                    Text::new("+", &FONT_10X20)
                        .padding(Edges::All, 12)
                        .background_color(color::SECONDARY, Rectangle)
                },
            ),
        ))
        .with_spacing(16),
    ))
    .with_spacing(24)
    .padding(Edges::All, 20)
}
