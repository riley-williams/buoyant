//! Counter workflow demonstrating basic harness usage.

use buoyant::focus::Role;
use buoyant_harness::{Harness, WorkflowEntry, workflow};

use crate::views::{CounterState, counter_view};

/// A simple workflow focused on counter interactions.
///
/// This is a minimal example showing basic harness usage with
/// increment and decrement operations.
pub fn counter_workflow() -> WorkflowEntry {
    workflow("counter", |config| {
        let mut h = config.init_with_roles(counter_view, CounterState::default(), Role::Button)?;

        h.render("initial")?;

        // Focus on first button (decrement)
        h.focus_forward();
        h.render("decrement_focused")?;

        // Move to increment button
        h.next();
        h.render("increment_focused")?;

        // Increment several times
        for i in 1..=5 {
            h.select();
            h.render(&format!("count_{i}"))?;
        }

        // Go back to decrement
        h.previous();
        h.render("decrement_focused_again")?;

        // Decrement twice
        h.select();
        h.render("count_4")?;

        h.select();
        h.render("count_3")?;

        Ok(())
    })
}
