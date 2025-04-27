// ANCHOR: all
// ANCHOR: simulator
use buoyant::{
    environment::DefaultEnvironment,
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::{padding::Edges, AsDrawable as _, HStack, Spacer, Text, View, ViewExt as _},
};
use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const DEFAULT_COLOR: Rgb888 = Rgb888::WHITE;

fn main() {
    let mut window = Window::new("Hello World", &OutputSettings::default());
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(480, 320));
    display.clear(BACKGROUND_COLOR);

    hello_view()
        .as_drawable(display.size(), DEFAULT_COLOR)
        .draw(&mut display)
        .unwrap();

    window.show_static(&display);
}

// ANCHOR_END: simulator
// ANCHOR: view
fn hello_view() -> impl View<Rgb888> {
    HStack::new((
        Text::new("Hello", &FONT_10X20).foreground_color(Rgb888::GREEN),
        Spacer::default(),
        Text::new("World", &FONT_10X20).foreground_color(Rgb888::YELLOW),
    ))
    .padding(Edges::All, 20)
}
// ANCHOR_END: view
// ANCHOR_END: all
