//! Chart demonstration workflows showcasing realistic chart use cases.

use buoyant_harness::{WorkflowEntry, workflow};

use crate::views::chart_demos::{
    ChartDemoState, budget_view, metrics_view, sales_view, scatter_view, sparklines_view,
    temperature_view,
};

/// 24-hour temperature line chart with title and thick line.
pub fn chart_demo_temperature() -> WorkflowEntry {
    workflow("chart_demo_temperature", |config| {
        let mut h = config.init(temperature_view, ChartDemoState)?;
        h.render("temperature_dashboard")?;
        Ok(())
    })
}

/// Quarterly revenue bar chart with wide spacing.
pub fn chart_demo_sales() -> WorkflowEntry {
    workflow("chart_demo_sales", |config| {
        let mut h = config.init(sales_view, ChartDemoState)?;
        h.render("quarterly_sales")?;
        Ok(())
    })
}

/// Response time vs load scatter plot.
pub fn chart_demo_scatter() -> WorkflowEntry {
    workflow("chart_demo_scatter", |config| {
        let mut h = config.init(scatter_view, ChartDemoState)?;
        h.render("latency_scatter")?;
        Ok(())
    })
}

/// Monthly budget (bars) vs target (line) overlay.
pub fn chart_demo_budget() -> WorkflowEntry {
    workflow("chart_demo_budget", |config| {
        let mut h = config.init(budget_view, ChartDemoState)?;
        h.render("budget_vs_spending")?;
        Ok(())
    })
}

/// System metrics: CPU line + Memory line + Alert points.
pub fn chart_demo_metrics() -> WorkflowEntry {
    workflow("chart_demo_metrics", |config| {
        let mut h = config.init(metrics_view, ChartDemoState)?;
        h.render("system_metrics")?;
        Ok(())
    })
}

/// Compact sparkline charts arranged in rows.
pub fn chart_demo_sparklines() -> WorkflowEntry {
    workflow("chart_demo_sparklines", |config| {
        let mut h = config.init(sparklines_view, ChartDemoState)?;
        h.render("sparklines")?;
        Ok(())
    })
}
