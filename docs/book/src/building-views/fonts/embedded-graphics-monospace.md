# Embedded Graphics Fonts

The `embedded-graphics` crate provides a selection of fixed-width fonts. These bitmapped
fonts are easy to use and render quickly, making them ideal for text in low-resource applications.

![Embedded Graphics Fonts](./images/monospace-fonts.png)

```rust,no_run
# extern crate buoyant;
# extern crate embedded_graphics;
# extern crate embedded_graphics_simulator;
# use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
# use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
# 
# const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_DARK_SLATE_GRAY;
# const DEFAULT_COLOR: Rgb888 = Rgb888::WHITE;
# 
# fn main() {
#     let mut window = Window::new("Example", &OutputSettings::default());
#     let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(480, 320));
# 
#     display.clear(BACKGROUND_COLOR).unwrap();
# 
#     view()
#         .as_drawable(display.size(), DEFAULT_COLOR, &mut ())
#         .draw(&mut display)
#         .unwrap();
# 
#     window.show_static(&display);
# }
# 
use buoyant::view::prelude::*;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X10, FONT_9X15};

fn view() -> impl View<Rgb888, ()> {
    VStack::new((
        Text::new("Small (6x10)", &FONT_6X10)
            .foreground_color(Rgb888::CSS_PALE_GREEN),
        Text::new("Medium (9x15)", &FONT_9X15)
            .foreground_color(Rgb888::CSS_LIGHT_SKY_BLUE),
        Text::new("Large (10x20)", &FONT_10X20)
            .foreground_color(Rgb888::CSS_LIGHT_CORAL),
    ))
    .with_spacing(20)
    .with_alignment(HorizontalAlignment::Center)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, 20)
}
```
