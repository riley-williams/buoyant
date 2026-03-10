/// Chart content production from data.
pub mod content;
/// Data point mark types.
pub mod mark;
/// Data-to-pixel coordinate mapping.
pub mod scale;
/// Series types for line, bar, and point charts.
pub mod series;

pub use content::ChartContent;
pub use mark::{BarMark, ChartMark, LineMark, PointMark};
pub use scale::{ChartScale, DataBounds};
pub use series::{BarSeries, ColoredSeries, LineSeries, PointSeries};

use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    transition::Opacity,
    view::{Event, ViewLayout, ViewMarker},
};

/// A chart view that renders data series as lines, bars, or points.
///
/// Chart consumes all offered space and renders the content within that area.
/// Data is mapped to pixel coordinates using a [`ChartScale`] computed from
/// the data bounds and the chart's resolved size.
///
/// # Examples
///
/// ```ignore
/// // Single line chart
/// Chart::new(
///     LineSeries::<100, _>::new(&sensor_data, |p| LineMark::new(p.time, p.value))
/// )
///
/// // Multi-series chart (bar + line overlay)
/// Chart::new((
///     BarSeries::<20, _>::new(&monthly, |d| BarMark::new(d.month, d.sales))
///         .with_color(Rgb565::BLUE),
///     LineSeries::<20, _>::new(&monthly, |d| LineMark::new(d.month, d.target))
///         .with_color(Rgb565::RED),
/// ))
/// ```
#[derive(Debug)]
pub struct Chart<S> {
    content: S,
}

impl<S> Chart<S> {
    /// Creates a new chart with the given content.
    #[must_use]
    pub const fn new(content: S) -> Self {
        Self { content }
    }
}

/// Layout data stored between `layout()` and `render_tree()` calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartSublayout {
    bounds: DataBounds,
    chart_size: Dimensions,
}

impl<S: ChartContent> ViewMarker for Chart<S> {
    type Renderables = S::Renderables;
    type Transition = Opacity;
}

impl<S: ChartContent, Captures: ?Sized> ViewLayout<Captures> for Chart<S> {
    type State = ();
    type Sublayout = ChartSublayout;
    type FocusTree = ();

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let bounds = self
            .content
            .data_bounds()
            .unwrap_or(DataBounds::new(0, 0, 0, 0));

        // Chart is greedy: consume all offered space
        let size = Dimensions {
            width: offer.width.resolve_most_flexible(0, 100),
            height: offer.height.resolve_most_flexible(0, 100),
        };

        ResolvedLayout {
            sublayouts: ChartSublayout {
                bounds,
                chart_size: size,
            },
            resolved_size: size,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        let scale = ChartScale::from_layout(layout.bounds, origin, layout.chart_size);
        self.content.build_renderables(&scale)
    }

    fn handle_event(
        &self,
        _event: &Event,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> EventResult {
        EventResult::Deferred
    }
}
