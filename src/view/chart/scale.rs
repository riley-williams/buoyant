use crate::primitives::{Dimensions, Point, Size};

use super::mark::ChartMark;

/// The data-space bounds of a chart's content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataBounds {
    /// Minimum x-coordinate in data space.
    pub x_min: i32,
    /// Maximum x-coordinate in data space.
    pub x_max: i32,
    /// Minimum y-coordinate in data space.
    pub y_min: i32,
    /// Maximum y-coordinate in data space.
    pub y_max: i32,
}

impl DataBounds {
    /// Creates new data bounds.
    #[must_use]
    pub const fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Returns the union of two data bounds (the smallest bounds containing both).
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            x_min: self.x_min.min(other.x_min),
            x_max: self.x_max.max(other.x_max),
            y_min: self.y_min.min(other.y_min),
            y_max: self.y_max.max(other.y_max),
        }
    }

    /// Creates bounds from an iterator of chart marks.
    ///
    /// Returns `None` if the iterator is empty.
    pub fn from_marks(marks: impl Iterator<Item = impl ChartMark>) -> Option<Self> {
        let mut bounds: Option<Self> = None;
        for mark in marks {
            let x = mark.x();
            let y = mark.y();
            bounds = Some(match bounds {
                None => Self::new(x, x, y, y),
                Some(b) => Self {
                    x_min: b.x_min.min(x),
                    x_max: b.x_max.max(x),
                    y_min: b.y_min.min(y),
                    y_max: b.y_max.max(y),
                },
            });
        }
        bounds
    }
}

/// Maps data coordinates to pixel coordinates within a chart area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChartScale {
    /// The data-space bounds.
    pub bounds: DataBounds,
    /// The pixel origin of the chart area.
    pub origin: Point,
    /// The pixel size of the chart area.
    pub size: Size,
}

impl ChartScale {
    /// Creates a new chart scale.
    #[must_use]
    pub const fn new(bounds: DataBounds, origin: Point, size: Size) -> Self {
        Self {
            bounds,
            origin,
            size,
        }
    }

    /// Maps a data x-coordinate to a pixel x-coordinate (i16).
    #[must_use]
    pub fn map_x(&self, x: i32) -> i16 {
        let range = self.bounds.x_max - self.bounds.x_min;
        if range == 0 {
            return (self.origin.x + self.size.width as i32 / 2) as i16;
        }
        let pixel = self.origin.x as i64
            + (x as i64 - self.bounds.x_min as i64) * self.size.width as i64 / range as i64;
        pixel as i16
    }

    /// Maps a data y-coordinate to a pixel y-coordinate (i16).
    ///
    /// Y-axis is inverted: data increases upward, pixels increase downward.
    #[must_use]
    pub fn map_y(&self, y: i32) -> i16 {
        let range = self.bounds.y_max - self.bounds.y_min;
        if range == 0 {
            return (self.origin.y + self.size.height as i32 / 2) as i16;
        }
        // Invert: higher data values → lower pixel y
        let pixel = self.origin.y as i64 + self.size.height as i64
            - (y as i64 - self.bounds.y_min as i64) * self.size.height as i64 / range as i64;
        pixel as i16
    }

    /// Maps a data point to pixel coordinates.
    #[must_use]
    pub fn map_point(&self, mark: &impl ChartMark) -> (i16, i16) {
        (self.map_x(mark.x()), self.map_y(mark.y()))
    }

    /// Returns the pixel width available for each bar in a bar chart.
    #[must_use]
    pub fn bar_width(&self, count: usize, spacing: u32) -> u32 {
        if count == 0 {
            return 0;
        }
        let total_spacing = spacing * count.saturating_sub(1) as u32;
        self.size.width.saturating_sub(total_spacing) / count as u32
    }
}

impl ChartScale {
    /// Creates a chart scale from data bounds and a chart's resolved layout.
    #[must_use]
    pub fn from_layout(bounds: DataBounds, origin: Point, dimensions: Dimensions) -> Self {
        Self::new(bounds, origin, Size::from(dimensions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_bounds_union() {
        let a = DataBounds::new(0, 10, 0, 20);
        let b = DataBounds::new(-5, 5, 10, 30);
        let u = a.union(&b);
        assert_eq!(u, DataBounds::new(-5, 10, 0, 30));
    }

    #[test]
    fn scale_map_x() {
        let scale = ChartScale::new(
            DataBounds::new(0, 100, 0, 100),
            Point::new(10, 10),
            Size::new(200, 100),
        );
        assert_eq!(scale.map_x(0), 10);
        assert_eq!(scale.map_x(50), 110);
        assert_eq!(scale.map_x(100), 210);
    }

    #[test]
    fn scale_map_y_inverted() {
        let scale = ChartScale::new(
            DataBounds::new(0, 100, 0, 100),
            Point::new(0, 0),
            Size::new(100, 100),
        );
        // y=0 (data min) → pixel y = origin + height = 100
        assert_eq!(scale.map_y(0), 100);
        // y=100 (data max) → pixel y = origin = 0
        assert_eq!(scale.map_y(100), 0);
        // y=50 → pixel y = 50
        assert_eq!(scale.map_y(50), 50);
    }

    #[test]
    fn scale_zero_range_centers() {
        let scale = ChartScale::new(
            DataBounds::new(5, 5, 10, 10),
            Point::new(0, 0),
            Size::new(100, 80),
        );
        assert_eq!(scale.map_x(5), 50);
        assert_eq!(scale.map_y(10), 40);
    }

    #[test]
    fn bar_width_calculation() {
        let scale = ChartScale::new(
            DataBounds::new(0, 4, 0, 10),
            Point::zero(),
            Size::new(100, 50),
        );
        // 5 bars, 2px spacing: total_spacing = 8, available = 92, width = 18
        assert_eq!(scale.bar_width(5, 2), 18);
        // 0 bars
        assert_eq!(scale.bar_width(0, 2), 0);
    }
}
