//! Chart workflows for visual validation of chart rendering.

use buoyant_harness::{WorkflowEntry, workflow};

use crate::views::chart::{
    ChartState, bar_chart_view, line_chart_view, multi_chart_view, scatter_chart_view,
};

/// Workflow rendering a line chart.
pub fn chart_line_workflow() -> WorkflowEntry {
    workflow("chart_line", |config| {
        let mut h = config.init(line_chart_view, ChartState)?;
        h.render("line_chart")?;
        Ok(())
    })
}

/// Workflow rendering a bar chart.
pub fn chart_bar_workflow() -> WorkflowEntry {
    workflow("chart_bar", |config| {
        let mut h = config.init(bar_chart_view, ChartState)?;
        h.render("bar_chart")?;
        Ok(())
    })
}

/// Workflow rendering a scatter plot.
pub fn chart_scatter_workflow() -> WorkflowEntry {
    workflow("chart_scatter", |config| {
        let mut h = config.init(scatter_chart_view, ChartState)?;
        h.render("scatter_chart")?;
        Ok(())
    })
}

/// Workflow rendering a multi-series chart (bar + line overlay).
pub fn chart_multi_workflow() -> WorkflowEntry {
    workflow("chart_multi", |config| {
        let mut h = config.init(multi_chart_view, ChartState)?;
        h.render("multi_chart")?;
        Ok(())
    })
}
