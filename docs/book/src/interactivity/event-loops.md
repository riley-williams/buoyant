# Event Loops

Extending the animated render loop and hiding the boilerplate:

```rust,no_run
# extern crate buoyant;
# extern crate embedded_graphics;
# extern crate embedded_graphics_simulator;
# use std::time::{Duration, Instant};
#
# use buoyant::{
#     environment::DefaultEnvironment,
#     event::{EventContext, simulator::MouseTracker},
#     primitives::{Point, Size},
#     render::{AnimatedJoin, AnimationDomain, Render},
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::prelude::*,
# };
# use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
# use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
#
fn main() {
#     let size = Size::new(200, 100);
#     let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size.into());
#     let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
#     let mut window = Window::new("Example", &OutputSettings::default());
#     let app_start = Instant::now();
#     let env = DefaultEnvironment::new(app_start.elapsed());
#
    let mut count = 0;
    // This derives higher-level mouse events from the raw simulator events
    let mut mouse_tracker = MouseTracker::new();

    let mut view = counter_view(count);

    let mut state = view.build_state(&mut count);
    let layout = view.layout(&size.into(), &env, &mut count, &mut state);
#
#     let mut source_tree = view.render_tree(
#         &layout,
#         Point::zero(),
#         &env,
#         &mut count,
#         &mut state,
#     );

    let mut target_tree = view.render_tree(
        &layout,
        Point::zero(),
        &env,
        &mut count,
        &mut state,
    );

    'running: loop {
#         target.display_mut().clear(Rgb888::BLACK).unwrap();

        // Render...
#         Render::render_animated(
#             &mut target,
#             &source_tree,
#             &target_tree,
#             &Rgb888::WHITE,
#             &AnimationDomain::top_level(app_start.elapsed()),
#         );
#
        // Flush to display...
#         window.update(target.display());

        // Handle events
        let context = EventContext::new(app_start.elapsed());

        let mut should_recompute_view = false;
        // This is missing a check for simulator exit events!
        for event in window.events().filter_map(|event| mouse_tracker.process_event(event)) {
            let result = view.handle_event( // <---- Event handling here!
                &event,
                &context,
                &mut target_tree,
                &mut count,
                &mut state
            );
            should_recompute_view |= result.recompute_view;
        }


        if should_recompute_view {
            // Construct view again with the updated state
            // Create a new target tree
#             let time = app_start.elapsed();
#             target_tree.join_from(
#                 &source_tree,
#                 &AnimationDomain::top_level(time),
#             );
#             source_tree = target_tree;
#
#             view = counter_view(count);
#             let env = DefaultEnvironment::new(time);
#             let layout = view.layout(&size.into(), &env, &mut count, &mut state);
#
#             target_tree = view.render_tree(
#                 &layout,
#                 Point::zero(),
#                 &env,
#                 &mut count,
#                 &mut state,
#             );
        }
    }
}

fn counter_view(count: i32) -> impl View<Rgb888, i32> {
    Button::new(
        |count: &mut i32| *count += 1,
        move |_| {
            Text::new_fmt::<48>(
                format_args!("I've been tapped {count} times!"),
                &embedded_graphics::mono_font::ascii::FONT_10X20,
            )
            .foreground_color(Rgb888::WHITE)
            .padding(Edges::All, 10)
        }
    )
}
```
