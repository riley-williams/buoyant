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
    app::{App, ButtonState},
    charge_simulator::ChargeSim,
    color,
};
use buoyant::{
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
};
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
    let app_start = Instant::now();

    let simulator = ChargeSim::new(1.0);
    let mut app = App::new(simulator);

    let mut source_tree = app.tree(target.size().into(), app_start.elapsed());
    let mut target_tree = app.tree(target.size().into(), app_start.elapsed());

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
        // Create a new target tree if the state changes
        if app.reset_dirty() {
            // merge into target
            target_tree.join_from(
                &source_tree,
                &AnimationDomain::top_level(app_start.elapsed()),
            );
            // swap
            source_tree = target_tree;
            // compute new target
            target_tree = app.tree(target.size().into(), app_start.elapsed());
        }

        target.clear(color::BACKGROUND);

        // Render frame
        Render::render_animated(
            &mut target,
            &source_tree,
            &target_tree,
            &color::WHITE,
            &AnimationDomain::top_level(app_start.elapsed()),
        );

        // Flush to window
        window.update(target.display());

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Space if button_down.is_unpressed() => {
                        button_down = ButtonState::Pressed(Instant::now());
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
                        .is_some_and(|i| i.elapsed().as_millis() <= 1000)
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
            .is_some_and(|i| i.elapsed().as_millis() > 1000)
        {
            app.state_mut().auto_off = !app.state().auto_off;
            button_down = ButtonState::Reset;
        }

        app.state_mut().simulator.update();
    }
}

fn cycle(current: &mut usize, steps: &[f32]) -> f32 {
    if steps.is_empty() {
        return 0.0;
    }
    *current = (*current + 1) % steps.len();
    steps.get(*current).copied().unwrap_or_default()
}
