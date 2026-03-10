use crate::primitives::Plottable;

/// A data point that can be plotted on a chart.
pub trait ChartMark {
    /// The x coordinate in data space.
    fn x(&self) -> i32;
    /// The y coordinate in data space.
    fn y(&self) -> i32;
}

/// A mark that connects data points with lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineMark {
    x: i32,
    y: i32,
}

impl LineMark {
    /// Creates a new line mark with the given data coordinates.
    #[must_use]
    pub fn new(x: impl Plottable, y: impl Plottable) -> Self {
        Self {
            x: x.as_i32(),
            y: y.as_i32(),
        }
    }
}

impl ChartMark for LineMark {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

/// A mark that renders data points as vertical bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarMark {
    x: i32,
    y: i32,
}

impl BarMark {
    /// Creates a new bar mark with the given data coordinates.
    #[must_use]
    pub fn new(x: impl Plottable, y: impl Plottable) -> Self {
        Self {
            x: x.as_i32(),
            y: y.as_i32(),
        }
    }
}

impl ChartMark for BarMark {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

/// A mark that renders data points as individual points (scatter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointMark {
    x: i32,
    y: i32,
}

impl PointMark {
    /// Creates a new point mark with the given data coordinates.
    #[must_use]
    pub fn new(x: impl Plottable, y: impl Plottable) -> Self {
        Self {
            x: x.as_i32(),
            y: y.as_i32(),
        }
    }
}

impl ChartMark for PointMark {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}
