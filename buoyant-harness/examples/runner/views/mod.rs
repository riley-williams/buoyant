//! Views for the agentic workflow examples.
//!
//! Each view module defines a state type and view function that can be
//! used with the workflow harness.

pub mod chart;
pub mod counter;
pub mod demo;

pub use counter::{CounterState, counter_view};
pub use demo::{DemoState, demo_view};
