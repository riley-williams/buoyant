//! Configuration for the test harness.
//!
//! [`HarnessConfig`] is passed to each workflow function and provides
//! builder-style methods to customize display size, colors, and timing
//! before initializing a [`TestHarness`].

use std::path::PathBuf;

use buoyant::{
    focus::{DefaultFocus, RoleSet},
    primitives::Size,
    render::{AnimatedJoin, Render},
    view::ViewLayout,
};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use serde::Serialize;

use crate::TestHarness;

/// Configuration passed to each workflow from the runner.
///
/// Provides builder-style methods to customize harness behavior before
/// initializing a [`TestHarness`] via [`init`](HarnessConfig::init) or
/// [`init_with_roles`](HarnessConfig::init_with_roles).
#[derive(Debug, Clone)]
pub struct HarnessConfig {
    /// Workflow name.
    pub name: String,
    /// Display size.
    pub size: Size,
    /// Foreground color.
    pub foreground: Rgb888,
    /// Background color.
    pub background: Rgb888,
    /// Time between steps in milliseconds.
    pub step_time: u64,
    /// Whether to show focus overlay on screenshots.
    pub show_overlay: bool,
    /// Whether running in visual debugging mode.
    pub visual_mode: bool,
    /// Directory for image output.
    pub images_dir: PathBuf,
    /// Directory for state JSON output.
    pub state_dir: PathBuf,
}

impl Default for HarnessConfig {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            size: Size::new(480, 320),
            foreground: Rgb888::WHITE,
            background: Rgb888::BLACK,
            step_time: 1000,
            show_overlay: true,
            visual_mode: false,
            images_dir: PathBuf::from("./workflow_output/default/images"),
            state_dir: PathBuf::from("./workflow_output/default/state"),
        }
    }
}

impl HarnessConfig {
    /// Override the display size.
    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Override the foreground color.
    #[must_use]
    pub fn foreground(mut self, color: Rgb888) -> Self {
        self.foreground = color;
        self
    }

    /// Override the background color.
    #[must_use]
    pub fn background(mut self, color: Rgb888) -> Self {
        self.background = color;
        self
    }

    /// Override the step time in milliseconds.
    #[must_use]
    pub fn step_time(mut self, ms: u64) -> Self {
        self.step_time = ms;
        self
    }

    /// Initialize a [`TestHarness`] with the given view function and initial state.
    ///
    /// # Errors
    ///
    /// Returns an error if the output directories cannot be created.
    pub fn init<V, S, F>(self, view_fn: F, state: S) -> Result<TestHarness<V, S, F>, String>
    where
        V: ViewLayout<S>,
        V::FocusTree: DefaultFocus,
        V::Renderables: Render<Rgb888> + AnimatedJoin,
        S: Serialize + 'static,
        F: Fn(&S) -> V,
    {
        TestHarness::new(self, view_fn, state)
    }

    /// Initialize a [`TestHarness`] with custom focus roles.
    ///
    /// # Errors
    ///
    /// Returns an error if the output directories cannot be created.
    pub fn init_with_roles<V, S, F>(
        self,
        view_fn: F,
        state: S,
        roles: impl Into<RoleSet>,
    ) -> Result<TestHarness<V, S, F>, String>
    where
        V: ViewLayout<S>,
        V::FocusTree: DefaultFocus,
        V::Renderables: Render<Rgb888> + AnimatedJoin,
        S: Serialize + 'static,
        F: Fn(&S) -> V,
    {
        let harness = TestHarness::new(self, view_fn, state)?;
        Ok(harness.with_roles(roles))
    }
}
