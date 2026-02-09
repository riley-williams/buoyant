//! Workflow definitions for the example runner.
//!
//! Each workflow is defined in its own module and returns a [`WorkflowEntry`]
//! which can be registered with the [`WorkflowRunner`].

pub mod counter;
pub mod demo;
pub mod toggle;

pub use counter::counter_workflow;
pub use demo::demo_workflow;
pub use toggle::toggle_workflow;
