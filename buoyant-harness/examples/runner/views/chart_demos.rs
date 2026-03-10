//! Chart demonstration views showcasing realistic use cases and customization options.

use buoyant::view::prelude::*;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use serde::Serialize;

mod color {
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::WebColors;

    pub const TEMP_LINE: Rgb888 = Rgb888::CSS_LIME_GREEN;
    pub const SALES_BAR: Rgb888 = Rgb888::CSS_CORNFLOWER_BLUE;
    pub const SCATTER_PT: Rgb888 = Rgb888::CSS_ORANGE_RED;
    pub const SPENDING: Rgb888 = Rgb888::CSS_INDIAN_RED;
    pub const BUDGET: Rgb888 = Rgb888::CSS_LIME_GREEN;
    pub const CPU: Rgb888 = Rgb888::CSS_DEEP_SKY_BLUE;
    pub const MEMORY: Rgb888 = Rgb888::CSS_MEDIUM_PURPLE;
    pub const ALERTS: Rgb888 = Rgb888::CSS_GOLD;
    pub const SPARK_LINE: Rgb888 = Rgb888::CSS_CYAN;
    pub const SPARK_BAR: Rgb888 = Rgb888::CSS_CORAL;
    pub const SPARK_POINT: Rgb888 = Rgb888::CSS_MEDIUM_SPRING_GREEN;
    pub const LABEL: Rgb888 = Rgb888::CSS_LIGHT_SKY_BLUE;
    pub const CHART_BG: Rgb888 = Rgb888::CSS_DARK_SLATE_GRAY;
}

/// Shared state for chart demos (no interaction needed).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChartDemoState;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct TempReading {
    hour: i32,
    celsius: i32,
}

#[derive(Debug, Clone, Copy)]
struct Revenue {
    quarter: i32,
    amount: i32,
}

#[derive(Debug, Clone, Copy)]
struct LoadSample {
    rps: i32,
    latency_ms: i32,
}

#[derive(Debug, Clone, Copy)]
struct MonthlyBudget {
    month: i32,
    spent: i32,
    target: i32,
}

#[derive(Debug, Clone, Copy)]
struct MetricSample {
    tick: i32,
    cpu: i32,
    mem: i32,
}

#[derive(Debug, Clone, Copy)]
struct AlertPoint {
    tick: i32,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
struct Spark {
    x: i32,
    y: i32,
}

// ---------------------------------------------------------------------------
// Static data sets
// ---------------------------------------------------------------------------

/// 24-hour temperature readings (hour 0–23)
const TEMP_DATA: &[TempReading] = &[
    TempReading { hour: 0, celsius: 14 },
    TempReading { hour: 1, celsius: 13 },
    TempReading { hour: 2, celsius: 12 },
    TempReading { hour: 3, celsius: 11 },
    TempReading { hour: 4, celsius: 11 },
    TempReading { hour: 5, celsius: 12 },
    TempReading { hour: 6, celsius: 14 },
    TempReading { hour: 7, celsius: 16 },
    TempReading { hour: 8, celsius: 19 },
    TempReading { hour: 9, celsius: 21 },
    TempReading { hour: 10, celsius: 23 },
    TempReading { hour: 11, celsius: 25 },
    TempReading { hour: 12, celsius: 27 },
    TempReading { hour: 13, celsius: 28 },
    TempReading { hour: 14, celsius: 28 },
    TempReading { hour: 15, celsius: 27 },
    TempReading { hour: 16, celsius: 25 },
    TempReading { hour: 17, celsius: 23 },
    TempReading { hour: 18, celsius: 21 },
    TempReading { hour: 19, celsius: 19 },
    TempReading { hour: 20, celsius: 17 },
    TempReading { hour: 21, celsius: 16 },
    TempReading { hour: 22, celsius: 15 },
    TempReading { hour: 23, celsius: 14 },
];

const SALES_DATA: &[Revenue] = &[
    Revenue { quarter: 1, amount: 420 },
    Revenue { quarter: 2, amount: 580 },
    Revenue { quarter: 3, amount: 510 },
    Revenue { quarter: 4, amount: 730 },
];

const LOAD_DATA: &[LoadSample] = &[
    LoadSample { rps: 100, latency_ms: 12 },
    LoadSample { rps: 200, latency_ms: 14 },
    LoadSample { rps: 350, latency_ms: 18 },
    LoadSample { rps: 500, latency_ms: 25 },
    LoadSample { rps: 600, latency_ms: 32 },
    LoadSample { rps: 750, latency_ms: 28 },
    LoadSample { rps: 800, latency_ms: 45 },
    LoadSample { rps: 900, latency_ms: 52 },
    LoadSample { rps: 1000, latency_ms: 68 },
    LoadSample { rps: 1100, latency_ms: 95 },
    LoadSample { rps: 1200, latency_ms: 120 },
    LoadSample { rps: 1300, latency_ms: 180 },
];

const BUDGET_DATA: &[MonthlyBudget] = &[
    MonthlyBudget { month: 1, spent: 3200, target: 3000 },
    MonthlyBudget { month: 2, spent: 2800, target: 3000 },
    MonthlyBudget { month: 3, spent: 3500, target: 3000 },
    MonthlyBudget { month: 4, spent: 2900, target: 3000 },
    MonthlyBudget { month: 5, spent: 3100, target: 3000 },
    MonthlyBudget { month: 6, spent: 2700, target: 3000 },
];

const METRICS_DATA: &[MetricSample] = &[
    MetricSample { tick: 0, cpu: 25, mem: 40 },
    MetricSample { tick: 1, cpu: 30, mem: 42 },
    MetricSample { tick: 2, cpu: 45, mem: 44 },
    MetricSample { tick: 3, cpu: 80, mem: 55 },
    MetricSample { tick: 4, cpu: 92, mem: 68 },
    MetricSample { tick: 5, cpu: 70, mem: 65 },
    MetricSample { tick: 6, cpu: 55, mem: 60 },
    MetricSample { tick: 7, cpu: 40, mem: 58 },
    MetricSample { tick: 8, cpu: 35, mem: 55 },
    MetricSample { tick: 9, cpu: 60, mem: 62 },
    MetricSample { tick: 10, cpu: 85, mem: 70 },
    MetricSample { tick: 11, cpu: 50, mem: 64 },
];

const ALERT_DATA: &[AlertPoint] = &[
    AlertPoint { tick: 3, value: 80 },
    AlertPoint { tick: 4, value: 92 },
    AlertPoint { tick: 10, value: 85 },
];

const SPARK_LINE_DATA: &[Spark] = &[
    Spark { x: 0, y: 4 }, Spark { x: 1, y: 7 }, Spark { x: 2, y: 5 },
    Spark { x: 3, y: 9 }, Spark { x: 4, y: 6 }, Spark { x: 5, y: 8 },
    Spark { x: 6, y: 11 }, Spark { x: 7, y: 10 },
];

const SPARK_BAR_DATA: &[Spark] = &[
    Spark { x: 0, y: 30 }, Spark { x: 1, y: 50 }, Spark { x: 2, y: 25 },
    Spark { x: 3, y: 60 }, Spark { x: 4, y: 45 },
];

const SPARK_POINT_DATA: &[Spark] = &[
    Spark { x: 0, y: 3 }, Spark { x: 1, y: 8 }, Spark { x: 2, y: 2 },
    Spark { x: 3, y: 6 }, Spark { x: 4, y: 9 }, Spark { x: 5, y: 4 },
];

// ---------------------------------------------------------------------------
// Demo 1: Temperature Dashboard
// ---------------------------------------------------------------------------

/// A 24-hour temperature chart with title label and thick line.
///
/// Demonstrates: Text + Chart composition, `with_line_width(3)`.
pub fn temperature_view(
    _state: &ChartDemoState,
) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("24h Temperature", &FONT_10X20).foreground_color(color::LABEL),
        Chart::new(
            LineSeries::<32, _, _>::new(TEMP_DATA, |r| LineMark::new(r.hour, r.celsius))
                .with_line_width(3)
                .with_color(color::TEMP_LINE),
        )
        .padding(Edges::All, 4)
        .background_color(color::CHART_BG, RoundedRectangle::new(6)),
    ))
    .with_spacing(8)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Demo 2: Quarterly Sales
// ---------------------------------------------------------------------------

/// Quarterly revenue bar chart with wide spacing and title.
///
/// Demonstrates: `with_spacing(8)` for wider bar gaps.
pub fn sales_view(_state: &ChartDemoState) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("Quarterly Revenue", &FONT_10X20).foreground_color(color::LABEL),
        Chart::new(
            BarSeries::<8, _, _>::new(SALES_DATA, |r| BarMark::new(r.quarter, r.amount))
                .with_spacing(8)
                .with_color(color::SALES_BAR),
        )
        .padding(Edges::All, 4)
        .background_color(color::CHART_BG, RoundedRectangle::new(6)),
    ))
    .with_spacing(8)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Demo 3: Response Time Scatter
// ---------------------------------------------------------------------------

/// Scatter plot of request latency vs server load.
///
/// Demonstrates: `with_point_size(8)` for larger scatter points.
pub fn scatter_view(_state: &ChartDemoState) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("Latency vs Load", &FONT_10X20).foreground_color(color::LABEL),
        Chart::new(
            PointSeries::<16, _, _>::new(LOAD_DATA, |s| PointMark::new(s.rps, s.latency_ms))
                .with_point_size(8)
                .with_color(color::SCATTER_PT),
        )
        .padding(Edges::All, 4)
        .background_color(color::CHART_BG, RoundedRectangle::new(6)),
    ))
    .with_spacing(8)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Demo 4: Budget vs Spending
// ---------------------------------------------------------------------------

/// Monthly spending bars overlaid with a budget target line.
///
/// Demonstrates: 2-series overlay with per-series colors.
pub fn budget_view(_state: &ChartDemoState) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("Monthly Budget", &FONT_10X20).foreground_color(color::LABEL),
        HStack::new((
            legend_dot(color::SPENDING, "Spent"),
            legend_dot(color::BUDGET, "Target"),
        ))
        .with_spacing(16),
        Chart::new((
            BarSeries::<8, _, _>::new(BUDGET_DATA, |b| BarMark::new(b.month, b.spent))
                .with_spacing(6)
                .with_color(color::SPENDING),
            LineSeries::<8, _, _>::new(BUDGET_DATA, |b| LineMark::new(b.month, b.target))
                .with_line_width(2)
                .with_color(color::BUDGET),
        ))
        .padding(Edges::All, 4)
        .background_color(color::CHART_BG, RoundedRectangle::new(6)),
    ))
    .with_spacing(8)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Demo 5: System Metrics
// ---------------------------------------------------------------------------

/// CPU, memory, and alert overlay with three series.
///
/// Demonstrates: 3-series composition with distinct colors.
pub fn metrics_view(_state: &ChartDemoState) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("System Metrics", &FONT_10X20).foreground_color(color::LABEL),
        HStack::new((
            legend_dot(color::CPU, "CPU"),
            legend_dot(color::MEMORY, "Mem"),
            legend_dot(color::ALERTS, "Alert"),
        ))
        .with_spacing(12),
        Chart::new((
            LineSeries::<16, _, _>::new(METRICS_DATA, |m| LineMark::new(m.tick, m.cpu))
                .with_line_width(2)
                .with_color(color::CPU),
            LineSeries::<16, _, _>::new(METRICS_DATA, |m| LineMark::new(m.tick, m.mem))
                .with_line_width(2)
                .with_color(color::MEMORY),
            PointSeries::<8, _, _>::new(ALERT_DATA, |a| PointMark::new(a.tick, a.value))
                .with_point_size(10)
                .with_color(color::ALERTS),
        ))
        .padding(Edges::All, 4)
        .background_color(color::CHART_BG, RoundedRectangle::new(6)),
    ))
    .with_spacing(8)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Demo 6: Compact Sparklines
// ---------------------------------------------------------------------------

/// Three compact sparkline charts arranged vertically with labels.
///
/// Demonstrates: `frame_sized()` for inline mini-charts, multiple charts in layout.
pub fn sparklines_view(_state: &ChartDemoState) -> impl View<Rgb888, ChartDemoState> + use<> {
    VStack::new((
        Text::new("Sparklines", &FONT_10X20).foreground_color(color::LABEL),
        sparkline_row(
            "Trend",
            Chart::new(
                LineSeries::<16, _, _>::new(SPARK_LINE_DATA, |s| LineMark::new(s.x, s.y))
                    .with_line_width(1)
                    .with_color(color::SPARK_LINE),
            ),
        ),
        sparkline_row(
            "Volume",
            Chart::new(
                BarSeries::<8, _, _>::new(SPARK_BAR_DATA, |s| BarMark::new(s.x, s.y))
                    .with_spacing(2)
                    .with_color(color::SPARK_BAR),
            ),
        ),
        sparkline_row(
            "Events",
            Chart::new(
                PointSeries::<8, _, _>::new(SPARK_POINT_DATA, |s| PointMark::new(s.x, s.y))
                    .with_point_size(4)
                    .with_color(color::SPARK_POINT),
            ),
        ),
    ))
    .with_spacing(12)
    .padding(Edges::All, 16)
    .foreground_color(Rgb888::WHITE)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// A small colored square followed by a text label (chart legend entry).
fn legend_dot(
    dot_color: Rgb888,
    label: &'static str,
) -> impl View<Rgb888, ChartDemoState> + use<> {
    HStack::new((
        Rectangle
            .foreground_color(dot_color)
            .frame_sized(10, 10),
        Text::new(label, &FONT_10X20).foreground_color(Rgb888::WHITE),
    ))
    .with_spacing(4)
}

/// A row with a fixed-width label and a compact sparkline chart.
fn sparkline_row<V: View<Rgb888, ChartDemoState>>(
    label: &'static str,
    chart: V,
) -> impl View<Rgb888, ChartDemoState> + use<V> {
    HStack::new((
        Text::new(label, &FONT_10X20)
            .foreground_color(Rgb888::WHITE)
            .frame()
            .with_width(80),
        chart
            .padding(Edges::All, 2)
            .background_color(color::CHART_BG, RoundedRectangle::new(4))
            .frame()
            .with_height(40),
    ))
    .with_spacing(8)
}
