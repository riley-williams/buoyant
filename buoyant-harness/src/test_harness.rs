//! Test harness with screenshot capture and visual debugging.
//!
//! [`TestHarness`] wraps [`App`] and adds screenshot capture,
//! state serialization, focus overlay rendering, and an optional
//! visual debugging window.

use std::fs;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use buoyant::{
    app::{App, Harness},
    event::{Event, EventResult},
    focus::{DefaultFocus, RoleSet},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget},
    view::ViewLayout,
};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{WebColors as _, *};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use serde::Serialize;

use crate::HarnessConfig;

/// A test harness that wraps [`App`] and adds screenshot capture,
/// state serialization, and optional visual debugging.
///
/// `TestHarness` implements [`Deref`] and [`DerefMut`] to [`App`],
/// so all [`App`] methods (including those from the [`Harness`] trait)
/// are directly accessible.
///
/// # Example
///
/// ```ignore
/// let mut h = config.init(my_view, MyState::default())?;
/// h.render("initial")?;
/// h.next();
/// h.select();
/// h.render("after_select")?;
/// ```
pub struct TestHarness<V, S, F>
where
    V: ViewLayout<S>,
{
    /// The inner [`App`] managing view lifecycle and event handling.
    pub app: App<V, S, F>,
    config: HarnessConfig,
    /// Current step number for ordering screenshots.
    step_number: usize,
    /// Window for visual debugging mode (created lazily).
    window: Option<Window>,
}

impl<V, S, F> std::fmt::Debug for TestHarness<V, S, F>
where
    V: ViewLayout<S>,
    App<V, S, F>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestHarness")
            .field("app", &self.app)
            .field("config", &self.config)
            .field("step_number", &self.step_number)
            .finish_non_exhaustive()
    }
}

impl<V, S, F> Deref for TestHarness<V, S, F>
where
    V: ViewLayout<S>,
    F: Fn(&S) -> V,
{
    type Target = App<V, S, F>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<V, S, F> DerefMut for TestHarness<V, S, F>
where
    V: ViewLayout<S>,
    F: Fn(&S) -> V,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<V, S, F> Harness for TestHarness<V, S, F>
where
    V: ViewLayout<S>,
    V::FocusTree: DefaultFocus,
    V::Renderables: AnimatedJoin,
    F: Fn(&S) -> V,
{
    fn send(&mut self, event: impl Into<Event>) -> EventResult {
        self.app.send(event)
    }
}

impl<V, S, F> TestHarness<V, S, F>
where
    V: ViewLayout<S>,
    V::FocusTree: DefaultFocus,
    V::Renderables: Render<Rgb888> + AnimatedJoin,
    S: Serialize + 'static,
    F: Fn(&S) -> V,
{
    /// Creates a new `TestHarness` with the given configuration, view function, and state.
    ///
    /// This creates output directories and initializes the inner [`App`].
    ///
    /// # Errors
    ///
    /// Returns an error if the output directories cannot be created.
    pub fn new(config: HarnessConfig, view_fn: F, state: S) -> Result<Self, String>
    where
        S: 'static,
    {
        // Create output directories
        fs::create_dir_all(&config.images_dir)
            .map_err(|e| format!("Failed to create images directory: {e}"))?;
        fs::create_dir_all(&config.state_dir)
            .map_err(|e| format!("Failed to create state directory: {e}"))?;

        let app = App::new(state, config.size, view_fn);

        Ok(Self {
            app,
            config,
            step_number: 0,
            window: None,
        })
    }

    /// Sets the roles for focus navigation.
    #[must_use]
    pub fn with_roles(mut self, roles: impl Into<RoleSet>) -> Self {
        self.app = self.app.with_roles(roles);
        self
    }

    /// Initializes the focus tree to the last element.
    #[must_use]
    pub fn with_focus_at_end(mut self) -> Self {
        self.app = self.app.with_focus_at_end();
        self
    }

    /// Returns a reference to the harness configuration.
    #[must_use]
    pub fn config(&self) -> &HarnessConfig {
        &self.config
    }

    /// Returns the current step number.
    #[must_use]
    pub fn step_number(&self) -> usize {
        self.step_number
    }

    /// Renders the current view state and saves it as a named step.
    ///
    /// This method:
    /// - Renders the view to an image
    /// - Optionally draws a focus overlay
    /// - Saves `{step_name}.png` to the images directory
    /// - Saves `{step_name}.json` to the state directory
    /// - In visual mode, displays the frame in a window with animation
    ///
    /// # Errors
    ///
    /// Returns an error if screenshots or state files cannot be saved,
    /// or if the visual debugging window is closed by the user.
    pub fn render(&mut self, step_name: &str) -> Result<(), String> {
        self.step_number += 1;

        // Create a display for rendering
        let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(self.config.size.into());

        // Clear and render
        display
            .clear(self.config.background)
            .map_err(|e| format!("{e:?}"))?;

        {
            let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);

            // Render only the target tree for screenshots
            self.app
                .render_only_target(&mut target, &self.config.foreground);

            // Draw focus overlay if enabled
            if self.config.show_overlay {
                self.draw_focus_overlay(&mut target);
            }
        }

        // Save PNG with step number prefix for correct sorting
        let output_settings = OutputSettingsBuilder::new().build();
        let output_image = display.to_rgb_output_image(&output_settings);
        let png_path = self
            .config
            .images_dir
            .join(format!("{:03}_{step_name}.png", self.step_number));
        output_image
            .save_png(&png_path)
            .map_err(|e| format!("Failed to save PNG: {e}"))?;

        // Save JSON state with step number prefix
        let json_path = self
            .config
            .state_dir
            .join(format!("{:03}_{step_name}.json", self.step_number));
        let json = serde_json::to_string_pretty(&self.app.state())
            .map_err(|e| format!("Failed to serialize state: {e}"))?;
        fs::write(&json_path, json).map_err(|e| format!("Failed to write JSON: {e}"))?;

        println!(
            "[{}] Step {}: {step_name}",
            self.config.name, self.step_number
        );

        // Visual mode: display in window with animation
        if self.config.visual_mode {
            self.display_in_window()?;
        }

        // Advance virtual time by the step duration
        self.app
            .advance_time(Duration::from_millis(self.config.step_time));

        Ok(())
    }

    /// Draws the focus overlay on the render target.
    fn draw_focus_overlay<T: RenderTarget<ColorFormat = Rgb888>>(&self, target: &mut T) {
        self.app.draw_focus_overlay(target, Rgb888::CSS_YELLOW, 2);
    }

    /// Displays the current frame in the window (visual mode) with animation.
    ///
    /// Loops rendering animated frames until animation completes or the step time is exceeded.
    fn display_in_window(&mut self) -> Result<(), String> {
        let step_start = Instant::now();
        let step_duration = Duration::from_millis(self.config.step_time);

        let mut frame_display: SimulatorDisplay<Rgb888> =
            SimulatorDisplay::new(self.config.size.into());

        let mut target = EmbeddedGraphicsRenderTarget::new(&mut frame_display);

        // Render frames until animation completes or timeout
        while target.clear_animation_status() {
            let frame_elapsed = step_start.elapsed();
            if frame_elapsed >= step_duration {
                break;
            }

            target
                .display_mut()
                .clear(self.config.background)
                .map_err(|e| format!("{e:?}"))?;

            // Render animated transition between source and target trees
            let elapsed = self.app.elapsed();
            let domain = AnimationDomain::top_level(elapsed + frame_elapsed);
            let trees = self.app.render_trees();
            Render::render_animated(
                &mut target,
                trees.source(),
                trees.target(),
                &self.config.foreground,
                &domain,
            );

            // Draw focus overlay if enabled
            if self.config.show_overlay {
                self.draw_focus_overlay(&mut target);
            }

            // Create window lazily
            let window = self.window.get_or_insert_with(|| {
                let output_settings = OutputSettingsBuilder::new().build();
                let title = format!("Workflow: {}", self.config.name);
                Window::new(&title, &output_settings)
            });

            window.update(target.display());

            // Handle window events (only exit)
            for event in window.events() {
                if event == SimulatorEvent::Quit {
                    return Err("Window closed by user".to_string());
                }
            }

            // ~60 fps between animation frames
            std::thread::sleep(Duration::from_millis(16));
        }

        // Sleep for remaining step time to show the final state
        let remaining = step_duration.saturating_sub(step_start.elapsed());
        if !remaining.is_zero() {
            std::thread::sleep(remaining);
        }

        Ok(())
    }
}
