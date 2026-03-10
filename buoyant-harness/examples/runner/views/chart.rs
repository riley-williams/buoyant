//! Chart views for harness workflow visual validation.

use buoyant::view::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use serde::Serialize;

mod color {
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::WebColors;

    pub const LINE: Rgb888 = Rgb888::CSS_LIME_GREEN;
    pub const BAR: Rgb888 = Rgb888::CSS_CORNFLOWER_BLUE;
    pub const POINT: Rgb888 = Rgb888::CSS_ORANGE_RED;
    pub const LINE2: Rgb888 = Rgb888::CSS_GOLD;
}

/// State for chart views (no interaction needed).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChartState;

/// Sample data point for charts.
#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub x: i32,
    pub y: i32,
}

const LINE_DATA: &[DataPoint] = &[
    DataPoint { x: 0, y: 10 },
    DataPoint { x: 1, y: 25 },
    DataPoint { x: 2, y: 18 },
    DataPoint { x: 3, y: 35 },
    DataPoint { x: 4, y: 28 },
    DataPoint { x: 5, y: 42 },
    DataPoint { x: 6, y: 38 },
    DataPoint { x: 7, y: 50 },
];

const BAR_DATA: &[DataPoint] = &[
    DataPoint { x: 0, y: 30 },
    DataPoint { x: 1, y: 45 },
    DataPoint { x: 2, y: 20 },
    DataPoint { x: 3, y: 55 },
    DataPoint { x: 4, y: 35 },
];

const SCATTER_DATA: &[DataPoint] = &[
    DataPoint { x: 5, y: 12 },
    DataPoint { x: 12, y: 28 },
    DataPoint { x: 18, y: 15 },
    DataPoint { x: 25, y: 42 },
    DataPoint { x: 30, y: 35 },
    DataPoint { x: 38, y: 48 },
    DataPoint { x: 42, y: 22 },
    DataPoint { x: 50, y: 55 },
    DataPoint { x: 55, y: 30 },
    DataPoint { x: 60, y: 45 },
];

/// A line chart view.
pub fn line_chart_view(_state: &ChartState) -> impl View<Rgb888, ChartState> + use<> {
    Chart::new(
        LineSeries::<20, _, _>::new(LINE_DATA, |p| LineMark::new(p.x, p.y))
            .with_line_width(2)
            .with_color(color::LINE),
    )
    .padding(Edges::All, 8)
    .foreground_color(Rgb888::WHITE)
}

/// A bar chart view.
pub fn bar_chart_view(_state: &ChartState) -> impl View<Rgb888, ChartState> + use<> {
    Chart::new(
        BarSeries::<20, _, _>::new(BAR_DATA, |p| BarMark::new(p.x, p.y))
            .with_spacing(4)
            .with_color(color::BAR),
    )
    .padding(Edges::All, 8)
    .foreground_color(Rgb888::WHITE)
}

/// A scatter plot view.
pub fn scatter_chart_view(_state: &ChartState) -> impl View<Rgb888, ChartState> + use<> {
    Chart::new(
        PointSeries::<20, _, _>::new(SCATTER_DATA, |p| PointMark::new(p.x, p.y))
            .with_point_size(6)
            .with_color(color::POINT),
    )
    .padding(Edges::All, 8)
    .foreground_color(Rgb888::WHITE)
}

/// A multi-series chart with line + bar overlay.
pub fn multi_chart_view(_state: &ChartState) -> impl View<Rgb888, ChartState> + use<> {
    Chart::new((
        BarSeries::<20, _, _>::new(BAR_DATA, |p| BarMark::new(p.x, p.y))
            .with_spacing(4)
            .with_color(color::BAR),
        LineSeries::<20, _, _>::new(LINE_DATA, |p| LineMark::new(p.x, p.y))
            .with_line_width(2)
            .with_color(color::LINE2),
    ))
    .padding(Edges::All, 8)
    .foreground_color(Rgb888::WHITE)
}
