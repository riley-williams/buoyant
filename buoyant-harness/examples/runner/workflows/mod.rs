//! Workflow definitions for the example runner.
//!
//! Each workflow is defined in its own module and returns a [`WorkflowEntry`]
//! which can be registered with the [`WorkflowRunner`].

pub mod chart;
pub mod counter;
pub mod demo;
pub mod toggle;

pub use chart::{chart_bar_workflow, chart_line_workflow, chart_multi_workflow, chart_scatter_workflow};
pub use counter::counter_workflow;
pub use demo::demo_workflow;
pub use toggle::toggle_workflow;
