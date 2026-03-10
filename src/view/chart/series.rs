use super::content::ChartContent;
use super::mark::{BarMark, ChartMark, LineMark, PointMark};
use super::scale::{ChartScale, DataBounds};
use crate::primitives::geometry::Rectangle;
use crate::render::ShadeSubtree;
use crate::render::chart::bar::{BarRenderable, ChartBar};
use crate::render::chart::line::LineRenderable;
use crate::render::chart::point::PointRenderable;

/// A series wrapper that applies a per-series color override.
///
/// Created by calling `.with_color(color)` on any series type.
/// The color overrides the inherited foreground for this series only.
pub struct ColoredSeries<S, C> {
    pub(crate) inner: S,
    pub(crate) color: C,
}

impl<S: core::fmt::Debug, C: core::fmt::Debug> core::fmt::Debug for ColoredSeries<S, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ColoredSeries")
            .field("inner", &self.inner)
            .field("color", &self.color)
            .finish()
    }
}

impl<S: ChartContent, C: Clone> ChartContent for ColoredSeries<S, C> {
    type Renderables = ShadeSubtree<C, S::Renderables>;

    fn data_bounds(&self) -> Option<DataBounds> {
        self.inner.data_bounds()
    }

    fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables {
        ShadeSubtree::new(self.color.clone(), self.inner.build_renderables(scale))
    }
}

/// A line series that connects data points with line segments.
///
/// `N` is the maximum number of data points the series can render.
///
/// # Examples
///
/// ```ignore
/// LineSeries::<100, _>::new(&sensor_data, |p| LineMark::new(p.time, p.value))
/// ```
pub struct LineSeries<'a, const N: usize, I, F>
where
    F: Fn(&I) -> LineMark,
{
    data: &'a [I],
    map_fn: F,
    line_width: u32,
}

impl<'a, const N: usize, I, F> core::fmt::Debug for LineSeries<'a, N, I, F>
where
    F: Fn(&I) -> LineMark,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LineSeries")
            .field("len", &self.data.len())
            .field("line_width", &self.line_width)
            .finish()
    }
}

impl<'a, const N: usize, I, F> LineSeries<'a, N, I, F>
where
    F: Fn(&I) -> LineMark,
{
    /// Creates a new line series from a data slice and mapping function.
    #[must_use]
    pub fn new(data: &'a [I], map_fn: F) -> Self {
        Self {
            data,
            map_fn,
            line_width: 2,
        }
    }

    /// Sets the line width in pixels.
    #[must_use]
    pub fn with_line_width(mut self, width: u32) -> Self {
        self.line_width = width;
        self
    }

    /// Applies a per-series color override.
    #[must_use]
    pub fn with_color<C>(self, color: C) -> ColoredSeries<Self, C> {
        ColoredSeries { inner: self, color }
    }
}

impl<'a, const N: usize, I, F> ChartContent for LineSeries<'a, N, I, F>
where
    F: Fn(&I) -> LineMark,
{
    type Renderables = LineRenderable<N>;

    fn data_bounds(&self) -> Option<DataBounds> {
        DataBounds::from_marks(self.data.iter().map(&self.map_fn))
    }

    fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables {
        let mut points = heapless::Vec::new();
        for item in self.data.iter().take(N) {
            let mark = (self.map_fn)(item);
            let _ = points.push(scale.map_point(&mark));
        }
        LineRenderable {
            points,
            line_width: self.line_width,
            frame: Rectangle::new(scale.origin, scale.size),
        }
    }
}

/// A bar series that renders data points as vertical bars.
///
/// `N` is the maximum number of bars the series can render.
pub struct BarSeries<'a, const N: usize, I, F>
where
    F: Fn(&I) -> BarMark,
{
    data: &'a [I],
    map_fn: F,
    spacing: u32,
}

impl<'a, const N: usize, I, F> core::fmt::Debug for BarSeries<'a, N, I, F>
where
    F: Fn(&I) -> BarMark,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BarSeries")
            .field("len", &self.data.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, const N: usize, I, F> BarSeries<'a, N, I, F>
where
    F: Fn(&I) -> BarMark,
{
    /// Creates a new bar series from a data slice and mapping function.
    #[must_use]
    pub fn new(data: &'a [I], map_fn: F) -> Self {
        Self {
            data,
            map_fn,
            spacing: 2,
        }
    }

    /// Sets the spacing between bars in pixels.
    #[must_use]
    pub fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Applies a per-series color override.
    #[must_use]
    pub fn with_color<C>(self, color: C) -> ColoredSeries<Self, C> {
        ColoredSeries { inner: self, color }
    }
}

impl<'a, const N: usize, I, F> ChartContent for BarSeries<'a, N, I, F>
where
    F: Fn(&I) -> BarMark,
{
    type Renderables = BarRenderable<N>;

    fn data_bounds(&self) -> Option<DataBounds> {
        let marks_bounds = DataBounds::from_marks(self.data.iter().map(&self.map_fn));
        // Bars always extend to y=0
        marks_bounds.map(|b| DataBounds::new(b.x_min, b.x_max, b.y_min.min(0), b.y_max.max(0)))
    }

    fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables {
        let count = self.data.len().min(N);
        let bar_w = scale.bar_width(count, self.spacing);
        let baseline_y = scale.map_y(0);

        let mut bars = heapless::Vec::new();
        for (i, item) in self.data.iter().take(N).enumerate() {
            let mark = (self.map_fn)(item);
            let mapped_y = scale.map_y(mark.y());

            let total_bar_area =
                bar_w * count as u32 + self.spacing * count.saturating_sub(1) as u32;
            let x_start =
                scale.origin.x + (scale.size.width.saturating_sub(total_bar_area)) as i32 / 2;
            let bar_x = x_start + (bar_w + self.spacing) as i32 * i as i32;

            let (top, height) = if mapped_y < baseline_y {
                (mapped_y, baseline_y - mapped_y)
            } else {
                (baseline_y, mapped_y - baseline_y)
            };

            let _ = bars.push(ChartBar {
                x: bar_x as i16,
                y: top,
                width: bar_w as i16,
                height,
            });
        }

        BarRenderable {
            bars,
            frame: Rectangle::new(scale.origin, scale.size),
        }
    }
}

/// A point/scatter series that renders data points as filled squares.
///
/// `N` is the maximum number of points the series can render.
pub struct PointSeries<'a, const N: usize, I, F>
where
    F: Fn(&I) -> PointMark,
{
    data: &'a [I],
    map_fn: F,
    point_size: u32,
}

impl<'a, const N: usize, I, F> core::fmt::Debug for PointSeries<'a, N, I, F>
where
    F: Fn(&I) -> PointMark,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PointSeries")
            .field("len", &self.data.len())
            .field("point_size", &self.point_size)
            .finish()
    }
}

impl<'a, const N: usize, I, F> PointSeries<'a, N, I, F>
where
    F: Fn(&I) -> PointMark,
{
    /// Creates a new point series from a data slice and mapping function.
    #[must_use]
    pub fn new(data: &'a [I], map_fn: F) -> Self {
        Self {
            data,
            map_fn,
            point_size: 4,
        }
    }

    /// Sets the diameter of each point in pixels.
    #[must_use]
    pub fn with_point_size(mut self, size: u32) -> Self {
        self.point_size = size;
        self
    }

    /// Applies a per-series color override.
    #[must_use]
    pub fn with_color<C>(self, color: C) -> ColoredSeries<Self, C> {
        ColoredSeries { inner: self, color }
    }
}

impl<'a, const N: usize, I, F> ChartContent for PointSeries<'a, N, I, F>
where
    F: Fn(&I) -> PointMark,
{
    type Renderables = PointRenderable<N>;

    fn data_bounds(&self) -> Option<DataBounds> {
        DataBounds::from_marks(self.data.iter().map(&self.map_fn))
    }

    fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables {
        let mut points = heapless::Vec::new();
        for item in self.data.iter().take(N) {
            let mark = (self.map_fn)(item);
            let _ = points.push(scale.map_point(&mark));
        }
        PointRenderable {
            points,
            point_size: self.point_size,
            frame: Rectangle::new(scale.origin, scale.size),
        }
    }
}
