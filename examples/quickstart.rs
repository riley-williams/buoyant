//! # Example: Quickstart
//!
//! This example renders a simple "Hello World" message using the ``embedded_graphics_simulator``.
//!
//! To run this example using the ``embedded_graphics_simulator``, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use buoyant::{
    environment::DefaultEnvironment,
    layout::Layout,
    render::{Render as _, Renderable as _},
    render_target::EmbeddedGraphicsRenderTarget,
    view::{padding::Edges, HStack, Spacer, Text, View, ViewExt as _},
};
use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const DEFAULT_COLOR: Rgb888 = Rgb888::WHITE;

fn main() {
    let size = Size::new(480, 320);
    let mut window = Window::new("Hello World", &OutputSettings::default());
    let display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size);
    let mut target = EmbeddedGraphicsRenderTarget::new(display, BACKGROUND_COLOR);

    target.target.clear(BACKGROUND_COLOR).unwrap();

    let environment = DefaultEnvironment::default();
    let origin = buoyant::primitives::Point::zero();

    let view = hello_view();
    let layout = view.layout(&size.into(), &environment);
    let render_tree = view.render_tree(&layout, origin, &environment);

    render_tree.render(&mut target, &DEFAULT_COLOR, origin);

    window.show_static(&target.target);
}

fn hello_view() -> impl View<Rgb888> {
    HStack::new((
        Text::new("Hello", &FONT_10X20).foreground_color(Rgb888::GREEN),
        Spacer::default(),
        Text::new("World", &FONT_10X20).foreground_color(Rgb888::YELLOW),
    ))
    .padding(Edges::All, 20)
}
