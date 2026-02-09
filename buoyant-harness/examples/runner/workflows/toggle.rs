//! Toggle workflow demonstrating tab navigation and feature toggling.

use buoyant::focus::Role;
use buoyant_harness::{Harness, WorkflowEntry, workflow};

use crate::views::{DemoState, demo_view};

/// A workflow focused on the toggle feature.
///
/// Demonstrates navigating directly to a specific tab and toggling
/// a feature on and off.
pub fn toggle_workflow() -> WorkflowEntry {
    workflow("toggle", |config| {
        let mut h = config.init_with_roles(
            demo_view,
            DemoState::default(),
            Role::Button | Role::Container,
        )?;

        h.render("initial")?;

        // Focus and navigate to toggle tab
        h.focus_forward();
        h.next(); // Move to Toggle tab
        h.select();
        h.render("toggle_tab")?;

        // Navigate to the toggle button
        h.next();
        h.next();
        h.next();
        h.render("toggle_focused")?;

        // Toggle on
        h.select();
        h.render("toggle_on")?;

        // Toggle off
        h.select();
        h.render("toggle_off")?;

        // Toggle on again
        h.select();
        h.render("toggle_on_again")?;

        Ok(())
    })
}
