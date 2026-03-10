use super::scale::{ChartScale, DataBounds};

/// A type that can produce chart renderables from data.
///
/// Implemented by series types (`LineSeries`, `BarSeries`, `PointSeries`) and
/// tuples of series for multi-series charts.
pub trait ChartContent {
    /// The render tree output of this chart content.
    type Renderables: Clone;

    /// Computes the data-space bounds across all data points.
    fn data_bounds(&self) -> Option<DataBounds>;

    /// Builds the renderable output from the computed scale.
    fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables;
}

// Tuple implementations for composing multiple series
macro_rules! impl_chart_content_tuple {
    ($(($idx:tt, $type:ident)),+) => {
        impl<$($type: ChartContent),+> ChartContent for ($($type,)+) {
            type Renderables = ($($type::Renderables,)+);

            fn data_bounds(&self) -> Option<DataBounds> {
                let mut bounds: Option<DataBounds> = None;
                $(
                    if let Some(b) = self.$idx.data_bounds() {
                        bounds = Some(match bounds {
                            None => b,
                            Some(existing) => existing.union(&b),
                        });
                    }
                )+
                bounds
            }

            fn build_renderables(&self, scale: &ChartScale) -> Self::Renderables {
                ($(self.$idx.build_renderables(scale),)+)
            }
        }
    };
}

impl_chart_content_tuple!((0, A));
impl_chart_content_tuple!((0, A), (1, B));
impl_chart_content_tuple!((0, A), (1, B), (2, C));
impl_chart_content_tuple!((0, A), (1, B), (2, C), (3, D));
