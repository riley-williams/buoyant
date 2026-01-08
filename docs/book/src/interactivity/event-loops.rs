// ANCHOR: all
use std::time::{Duration, Instant};

use buoyant::{
    environment::DefaultEnvironment,
    event::{EventContext, EventResult, simulator::MouseTracker},
    primitives::{Point, Size},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

// ANCHOR: main
fn main() {
    let size = Size::new(200, 100);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size.into());
    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
    let mut window = Window::new("Example", &OutputSettings::default());
    // Send at least one update to the window so it doesn't panic when fetching events
    window.update(target.display());

    let app_start = Instant::now();
    // This derives higher-level mouse events from the raw simulator events
    let mut mouse_tracker = MouseTracker::new();

    let mut count = 0;
    let mut view = counter_view(count);
    let mut state = view.build_state(&mut count);

    // Create initial source and target trees for animation
    let time = app_start.elapsed();
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&size.into(), &env, &mut count, &mut state);

    let mut source_tree = &mut view.render_tree(
        &layout.sublayouts,
        Point::zero(),
        &env,
        &mut count,
        &mut state,
    );
    let mut target_tree = &mut view.render_tree(
        &layout.sublayouts,
        Point::zero(),
        &env,
        &mut count,
        &mut state,
    );

    // ANCHOR: event_loop
    // Main event loop
    loop {
        let time = app_start.elapsed();
        let domain = AnimationDomain::top_level(time);
        let context = EventContext::new(time);

        let mut should_exit = false;

        // ANCHOR: handle_events
        // Handle events, merging into a single result
        let result = window
            .events()
            .filter_map(|event| mouse_tracker.process_event(event))
            .fold(EventResult::default(), |result, event| {
                // Manually handle exit events
                if event == buoyant::event::Event::Exit {
                    should_exit = true;
                }
                result.merging(view.handle_event(
                    &event,
                    &context,
                    target_tree,
                    &mut count,
                    &mut state,
                ))
            });
        // ANCHOR_END: handle_events

        if should_exit {
            break;
        }

        // ANCHOR: recompute_view
        // Only recompute the view, layout, and render trees if necessary.
        // Additional handling may be needed to recompute the view in response to external events.
        if result.recompute_view {
            // Join source and target trees at current time, "freezing" animation progress
            target_tree.join_from(source_tree, &domain);
            // Swap trees so the current target becomes the next source.
            // Note this swaps the references instead of the whole section of memory
            core::mem::swap(&mut source_tree, &mut target_tree);
            // Create new view and target tree
            view = counter_view(count);
            let env = DefaultEnvironment::new(time);
            let layout = view.layout(&size.into(), &env, &mut count, &mut state);
            *target_tree = view.render_tree(
                &layout.sublayouts,
                Point::zero(),
                &env,
                &mut count,
                &mut state,
            );
        }
        // ANCHOR_END: recompute_view

        // ANCHOR: render_idle
        // Only render if active animation was reported, the view changed, or redraw was requested
        if target.clear_animation_status() || result.recompute_view || result.redraw {
            // Render animated transition between source and target trees
            Render::render_animated(
                &mut target,
                source_tree,
                target_tree,
                &Rgb888::WHITE,
                &domain,
            );
            // Send to the display
            window.update(target.display());
            // Clear for the next frame
            target.clear(Rgb888::BLACK);

            // Optionally cap frame rate by sleeping here
        } else {
            // limit polling for updates to ~30 fps when idle
            std::thread::sleep(Duration::from_millis(33));
        }
        // ANCHOR_END: render_idle
    }
    // ANCHOR_END: event_loop
}
// ANCHOR_END: main

// ANCHOR: view
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
        },
    )
}
// ANCHOR_END: view
// ANCHOR_END: all
