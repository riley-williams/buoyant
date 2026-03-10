//! Workflow definitions for the example runner.
//!
//! Each workflow is defined in its own module and returns a [`WorkflowEntry`]
//! which can be registered with the [`WorkflowRunner`].

pub mod chart;
pub mod chart_demos;
pub mod counter;
pub mod demo;
pub mod toggle;

pub use chart::{chart_bar_workflow, chart_line_workflow, chart_multi_workflow, chart_scatter_workflow};
pub use chart_demos::{
    chart_demo_budget, chart_demo_metrics, chart_demo_sales, chart_demo_scatter,
    chart_demo_sparklines, chart_demo_temperature,
};
pub use counter::counter_workflow;
pub use demo::demo_workflow;
pub use toggle::toggle_workflow;
