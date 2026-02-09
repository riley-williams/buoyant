//! Demo workflow exercising the full multi-tab demo view.

use buoyant::focus::Role;
use buoyant_harness::{Harness, WorkflowEntry, workflow};

use crate::views::{DemoState, demo_view};

/// A comprehensive workflow that exercises the full demo view.
///
/// This workflow navigates through tabs, toggles features, and increments
/// the counter to demonstrate the full capabilities of the harness.
pub fn demo_workflow() -> WorkflowEntry {
    workflow("demo", |config| {
        let mut h = config.init_with_roles(
            demo_view,
            DemoState::default(),
            Role::Button | Role::Container,
        )?;

        // Initial state
        h.render("initial")?;

        // Acquire focus on first element
        h.focus_forward();
        h.render("focused")?;

        // Navigate to Toggle tab and select it
        h.next();
        h.render("toggle_tab_focused")?;

        h.select();
        h.render("toggle_tab_selected")?;

        // Navigate to the toggle button and activate it
        h.next();
        h.next();
        h.next();
        h.render("toggle_button_focused")?;

        h.select();
        h.render("toggle_enabled")?;

        // Go back to Counter tab
        h.previous();
        h.previous();
        h.previous();
        h.select();
        h.render("counter_tab")?;

        // Navigate to increment button and click it multiple times
        h.next();
        h.next();
        h.render("increment_focused")?;

        h.select();
        h.render("counter_1")?;

        h.select();
        h.render("counter_2")?;

        h.select();
        h.render("counter_3")?;

        // Navigate to Info tab
        h.previous();
        h.previous();
        h.previous();
        h.next();
        h.next();
        h.select();
        h.render("info_tab")?;

        Ok(())
    })
}
