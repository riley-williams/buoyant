# Manual View Lifecycle

While the `AsDrawable` trait is useful for quickly rendering a view, you can also manually
manage the layout and rendering stages of a view.

Looking back at the simple Hello World example, we can replace the `AsDrawable` trait usage
with a manual view lifecycle.

```rust,no_run
# extern crate buoyant;
# extern crate embedded_graphics;
# extern crate embedded_graphics_simulator;
use buoyant::{
    environment::DefaultEnvironment,
    layout::Layout,
    render::{Render as _, Renderable as _},
    render_target::EmbeddedGraphicsRenderTarget,
    view::prelude::*,
};
use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const DEFAULT_COLOR: Rgb888 = Rgb888::WHITE;

fn main() {
    let size = Size::new(480, 320);
    let mut window = Window::new("Hello World", &OutputSettings::default());
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size);
    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);

    target.display_mut().clear(BACKGROUND_COLOR).unwrap();

    let environment = DefaultEnvironment::default();
    let origin = buoyant::primitives::Point::zero();

    let view = hello_view();
    let layout = view.layout(&size.into(), &environment);
    let render_tree = view.render_tree(&layout, origin, &environment);

    render_tree.render(&mut target, &DEFAULT_COLOR, origin);

    window.show_static(target.display());
}

fn hello_view() -> impl View<Rgb888> {
    HStack::new((
        Text::new("Hello", &FONT_10X20).foreground_color(Rgb888::GREEN),
        Spacer::default(),
        Text::new("World", &FONT_10X20).foreground_color(Rgb888::YELLOW),
    ))
    .padding(Edges::All, 20)
}
```

## Layout

```rust,ignore
let layout = view.layout(&size.into(), &environment);
```

The layout call resolves the sizes of all the views. It is a bug to try to reuse the layout
after mutating the view, and Buoyant may panic if you do so.

## Render Tree

```rust,ignore
let render_tree = view.render_tree(&layout, origin, &environment);
```

The render tree is a minimal snapshot of the view. It holds a copy of the resolved positions,
sizes, colors, etc. of all the elements that are actually rendered to the screen.
Relational elements like `Padding`, `Frame`s, alignment, and so on have been stripped.

## Rendering

```rust,ignore
render_tree.render(&mut display, &DEFAULT_COLOR, origin);
```

Here, the snapshot is finally rendered to the display buffer. A default color, similar to SwiftUI's
foreground color, is passed in. This is used for elements that don't have a color set.

## Why?

For just rendering a static view, this feels like (and is) a lot of boilerplate from Buoyant.
However, as you'll see in the next section, having multiple snapshots allows you to create
incredibly powerful animation between them with next to no effort.
