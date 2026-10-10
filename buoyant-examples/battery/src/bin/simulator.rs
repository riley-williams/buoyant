//! This simulates a portable battery display, which is designed to be somewhat
//! flexible in the physical dimensions of the display.
//!
//! A side button is simulated with the space bar.
//!
//! The 1, 2, and 3 keys can be used to cycle between charge/discharge states on each port.
//!
//! This example is not intended to be a recommendation on Buoyant app architecture. I hacked it
//! together in an afternoon.

use std::time::Instant;

use battery::{
    app::{ButtonState, State, root_view},
    charge_simulator::ChargeSim,
    color,
};
use buoyant::app::App;
use buoyant::render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettings, SimulatorDisplay, SimulatorEvent, Window, sdl2::Keycode,
};

fn main() {
    let mut window = Window::new("Charge Simulator", &OutputSettings::default());
    // Some display sizes to try:
    // Size::new(210, 110)
    // Size::new(110, 310)
    let mut display: SimulatorDisplay<color::ColorFormat> =
        SimulatorDisplay::new(Size::new(210, 110));
    let mut target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, color::BACKGROUND);

    // Send at least one update to the window so it doesn't panic when fetching events
    window.update(target.display());

    let app_start = Instant::now();

    let simulator = ChargeSim::new(1.0, app_start.elapsed());
    let initial_state = State::new(simulator);
    let mut app = App::new(initial_state, target.size(), root_view);

    let mut button_down = ButtonState::Unpressed;

    let mut c1_step = 0;
    let mut c2_step = 0;
    let mut a_step = 0;

    let c1_steps = [
        -1.0, -10.0, -100.0, -140.0, 0.0, 1.0, 10.0, 100.0, 140.0, 0.0,
    ];
    let c2_steps = [0.0, 1.0, 10.0, 100.0, 140.0];
    let a_steps = [0.0, 1.0, 5.0, 20.0];

    'running: loop {
        // Sync the app's virtual clock with wall-clock time so animations advance.
        app.set_time(app_start.elapsed());

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Space if button_down.is_unpressed() => {
                        button_down = ButtonState::Pressed(app_start.elapsed());
                    }
                    Keycode::Num1 => {
                        app.state_mut().simulator.battery.ports.usbc1_power =
                            cycle(&mut c1_step, &c1_steps);
                    }
                    Keycode::Num2 => {
                        app.state_mut().simulator.battery.ports.usbc2_power =
                            cycle(&mut c2_step, &c2_steps);
                    }
                    Keycode::Num3 => {
                        app.state_mut().simulator.battery.ports.usba_power =
                            cycle(&mut a_step, &a_steps);
                    }
                    _ => {}
                },
                SimulatorEvent::KeyUp { keycode, .. } if keycode == Keycode::Space => {
                    if button_down
                        .pressed()
                        .is_some_and(|i| (app_start.elapsed().abs_diff(i)).as_millis() <= 1000)
                    {
                        app.state_mut().screen.increment();
                    }
                    button_down = ButtonState::Unpressed;
                }

                _ => {}
            }
        }

        if button_down
            .pressed()
            .is_some_and(|i| (app_start.elapsed().abs_diff(i)).as_millis() > 1000)
        {
            app.state_mut().auto_off = !app.state().auto_off;
            button_down = ButtonState::Reset;
        }

        app.state_mut().simulator.update(app_start.elapsed());

        // Render if there was a state change or an animation is still in flight.
        if app.should_redraw() || target.clear_animation_status() {
            target.clear(color::BACKGROUND);
            app.render_animated(&mut target, &color::WHITE);
            window.update(target.display());
        }
    }
}

fn cycle(current: &mut usize, steps: &[f32]) -> f32 {
    if steps.is_empty() {
        return 0.0;
    }
    *current = (*current + 1) % steps.len();
    steps.get(*current).copied().unwrap_or_default()
}
