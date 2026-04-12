// ANCHOR: all
use std::process::exit;
use std::time::{Duration, Instant};

use buoyant::{
    app::{App, Harness},
    event::simulator::MouseTracker,
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

// ANCHOR: state
#[derive(Clone, Default)]
struct State {
    count: i32,
}
// ANCHOR_END: state

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

    // Create app with view lifecycle management
    let mut app = App::new(State::default(), size.into(), counter_view);

    // ANCHOR: event_loop
    // Main event loop
    loop {
        // ANCHOR: handle_events
        // Update the app time
        app.set_time(app_start.elapsed());

        // Process simulator events
        window
            .events()
            .filter_map(|event| {
                // The simulator won't exit if we don't handle this
                if event == embedded_graphics_simulator::SimulatorEvent::Quit {
                    exit(0);
                }
                mouse_tracker.process_event(event)
            })
            .for_each(|event| {
                app.send(event);
            });
        // ANCHOR_END: handle_events

        // ANCHOR: render_idle
        // Only render if active animation was reported or redraw is needed
        if app.should_redraw() || target.clear_animation_status() {
            // Render animated transition between source and target trees
            app.render_animated(&mut target, &Rgb888::WHITE);

            // Send to the display
            window.update(target.display());
            // Clear for the next frame
            target.clear(Rgb888::BLACK);
        } else {
            // Optionally cap polling rate by sleeping
            std::thread::sleep(Duration::from_millis(5));
        }
        // ANCHOR_END: render_idle
    }
    // ANCHOR_END: event_loop
}
// ANCHOR_END: main

// ANCHOR: view
fn counter_view(state: &State) -> impl View<Rgb888, State> + use<> {
    let count = state.count;
    Button::new(
        |state: &mut State| state.count += 1,
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
