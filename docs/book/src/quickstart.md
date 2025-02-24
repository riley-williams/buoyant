# Quickstart

## Embedded graphics simulator

Unless otherwise noted, the examples in this book utilize the [embedded-graphics-simulator](https://github.com/embedded-graphics/simulator).

To run `embedded-graphics-simulator` examples, you'll need to follow the instructions in the
`embedded-graphics-simulator` README to install sdl2.

## Add dependencies

```toml
# Cargo.toml

[dependencies]
buoyant = "0.3"
embedded-graphics-simulator = "0.7.0"
```

## Set up the simulator window

```rust
use buoyant::{
    environment::DefaultEnvironment,
    layout::Layout,
    render::{EmbeddedGraphicsRender, Renderable},
    view::{HStack, RenderExtensions, Spacer, Text},
};
use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

fn main() {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(480, 320));
    let mut window = Window::new("Hello World", &OutputSettings::default());

    let environment = DefaultEnvironment::default();
    let origin = buoyant::primitives::Point::zero();
    let background_color = Rgb888::BLACK;
    let default_color = Rgb888::WHITE;

    display.clear(background_color).unwrap();

    let view = hello_view();
    let layout = view.layout(&display.size().into(), &environment);
    let render_tree = view.render_tree(&layout, origin, &environment);

    render_tree.render(&mut display, &default_color, origin);

    window.show_static(&display);
}

fn hello_view() -> impl Renderable<Rgb888, Renderables: EmbeddedGraphicsRender<Rgb888>> {
    HStack::new((
        Text::str("Hello", &FONT_10X20).foreground_color(Rgb888::GREEN),
        Spacer::default(),
        Text::str("World", &FONT_10X20).foreground_color(Rgb888::YELLOW),
    ))
}
```
