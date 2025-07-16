# Animated Render Loops

## Animating Between Render Trees

To animate between two render trees, you can use the `render_animated()` method:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use std::time::Duration;
#
# use buoyant::{
#     environment::DefaultEnvironment,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::prelude::*,
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
#
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
#
# let mut captures = ();
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let mut view_state = source_view.build_state(&mut ());
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment, &mut captures, &mut view_state);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment, &mut captures, &mut view_state);

Render::render_animated(
    &mut target,
    &source_render_tree,
    &target_render_tree,
    &Rgb888::BLACK,
    Point::zero(),
    &AnimationDomain::top_level(app_time),
);
#
# /// This is just a tribute to the greatest view in the world.
# fn view() -> impl View<Rgb888, ()> {
#     EmptyView // Couldn't remember
# }
```

## Joining Trees

Generally, all animations in Buoyant are interruptible. In the same way you can animate
rendering between two trees, you can also join two trees to form a new one. This allows
you to continuously merge and generate new trees to create a smooth animated render loop.

Render tree types conform to `AnimatedJoin`, which allows you to create a joined tree
at a specific point in time. With some exceptions, the result of rendering the joined tree
is the same as rendering the two trees with `render_animated()`.

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use std::time::Duration;
#
# use buoyant::{
#     environment::DefaultEnvironment,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::prelude::*,
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
#
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
#
# let mut captures = ();
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let mut view_state = source_view.build_state(&mut ());
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment, &mut captures, &mut view_state);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment, &mut captures, &mut view_state);

// Join two trees
let joined_tree = AnimatedJoin::join(
    source_render_tree,
    target_render_tree,
    &AnimationDomain::top_level(app_time),
);

// Calling render on the joined tree produces the same result as
// the render_animated call above
joined_tree.render(&mut target, &Rgb888::BLACK, Point::zero());
#
# fn view() -> impl View<Rgb888, ()> {
#     EmptyView
# }
```

Joining trees encodes information about the partially completed animation, which allows multiple
staggered animations to occur in a render loop.

## Creating a Render Loop

Buoyant on its own does not track whether state has changed, and you are responsible
for managing the view and render tree lifecycle in response to state changes.

Here's a minimal example that demonstrates the render loop pattern:

```rust,no_run
# extern crate buoyant;
# extern crate embedded_graphics;
# extern crate embedded_graphics_simulator;
use std::time::{Duration, Instant};

use buoyant::{
    environment::DefaultEnvironment,
    primitives::{Point, Size},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::EmbeddedGraphicsRenderTarget,
    view::prelude::*,
};
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct AppState {
    counter: u32,
}

fn main() {
    let size = Size::new(200, 100);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size.into());
    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
    let mut window = Window::new("Render Loop Example", &OutputSettings::default());
    let app_start = Instant::now();

    let mut captures = AppState::default();
    let env = DefaultEnvironment::new(app_start.elapsed());
    let mut view = root_view(&captures);
    let mut state = view.build_state(&mut captures);
    let layout = view.layout(&size.into(), &env, &mut captures, &mut state);

    let mut source_tree = view.render_tree(
        &layout,
        Point::zero(),
        &env,
        &mut captures,
        &mut state,
    );

    let mut target_tree = view.render_tree(
        &layout,
        Point::zero(),
        &env,
        &mut captures,
        &mut state,
    );

    let mut rebuild_view = true;
    'running: loop {
        target.display_mut().clear(Rgb888::BLACK).unwrap();

        // Render, animating between the source and target trees
        Render::render_animated(
            &mut target,
            &source_tree,
            &target_tree,
            &Rgb888::WHITE,
            Point::zero(),
            &AnimationDomain::top_level(app_start.elapsed()),
        );

        window.update(target.display());

        if rebuild_view {
            rebuild_view = false;
            // Join source and target trees at the current time
            let time = app_start.elapsed();
            source_tree = AnimatedJoin::join(
                source_tree,
                target_tree,
                &AnimationDomain::top_level(time),
            );

            view = root_view(&captures);
            let env = DefaultEnvironment::new(time);
            let layout = view.layout(&size.into(), &env, &mut captures, &mut state);

            target_tree = view.render_tree(
                &layout,
                Point::zero(),
                &env,
                &mut captures,
                &mut state,
            );
        }

        for event in window.events() {
            // handle view events
            // TODO: set rebuild_view = true on event
            if let embedded_graphics_simulator::SimulatorEvent::Quit = event {
                break 'running;
            }
        }
    }
}

fn root_view(state: &AppState) -> impl View<Rgb888, AppState> {
    let counter = state.counter; // make sure the closure doesn't capture state
    Button::new(
        |state: &mut AppState| state.counter += 1,
        move |_| {
            Text::new_fmt::<32>(
                format_args!("Counter: {}", counter),
                &embedded_graphics::mono_font::ascii::FONT_10X20,
            )
            .foreground_color(Rgb888::WHITE)
            .padding(Edges::All, 10)
        }
    )
}
```

This loop will animate between the source and target trees, creating a new target tree when
the state changes. The source tree is joined with the original target tree to create a new
source tree that continues the animation from where it left off.
