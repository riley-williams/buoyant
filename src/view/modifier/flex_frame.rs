use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::{Alignment, HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
pub struct FlexFrame<T> {
    child: T,
    min_width: Option<u32>,
    ideal_width: Option<u32>,
    max_width: Option<Dimension>,
    min_height: Option<u32>,
    ideal_height: Option<u32>,
    max_height: Option<Dimension>,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T: ViewMarker> FlexFrame<T> {
    pub fn new(child: T) -> Self {
        Self {
            child,
            min_width: None,
            ideal_width: None,
            max_width: None,
            min_height: None,
            ideal_height: None,
            max_height: None,
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }

    /// Applies a minimum width to the frame.
    ///
    /// The child view may still resolve to a smaller width if it is smaller than
    /// the minimum width.
    pub const fn with_min_width(mut self, min_width: u32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// The width to propose to the child view when a parent view requests a compact width.
    ///
    /// This is especially useful for views like shapes which have no inherent size.
    pub const fn with_ideal_width(mut self, ideal_width: u32) -> Self {
        self.ideal_width = Some(ideal_width);
        self
    }

    /// The maximum width of the frame.
    ///
    /// The child view will be proposed a width up to the max, and the frame will resolve
    /// greedily up to this width.
    pub fn with_max_width<U: Into<Dimension>>(mut self, max_width: U) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    /// Sets the frame to expand to fill as much horizontal space as possible.
    pub const fn with_infinite_max_width(mut self) -> Self {
        self.max_width = Some(Dimension::infinite());
        self
    }

    /// Sets the frame to expand to fill as much space as possible.
    pub const fn with_infinite_max_dimensions(mut self) -> Self {
        self.max_width = Some(Dimension::infinite());
        self.max_height = Some(Dimension::infinite());
        self
    }

    /// Applies a minimum height to the frame.
    ///
    /// The child view may still resolve to a smaller height if it is smaller than
    /// the minimum height.
    pub const fn with_min_height(mut self, min_height: u32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// The height to propose to the child view when a parent view requests a compact height.
    ///
    /// This is especially useful for views like shapes which have no inherent size.
    pub const fn with_ideal_height(mut self, ideal_height: u32) -> Self {
        self.ideal_height = Some(ideal_height);
        self
    }

    /// The maximum height of the frame.
    ///
    /// The child view will be proposed a height up to the max, and the frame will resolve
    /// greedily up to this height.
    pub fn with_max_height<U: Into<Dimension>>(mut self, max_height: U) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    /// Sets the frame to expand to fill as much vertical space as possible.
    pub const fn with_infinite_max_height(mut self) -> Self {
        self.max_height = Some(Dimension::infinite());
        self
    }

    /// Sets the maximum size of the frame.
    pub fn with_max_size<U: Into<Dimension>>(mut self, max_width: U, max_height: U) -> Self {
        self.max_width = Some(max_width.into());
        self.max_height = Some(max_height.into());
        self
    }

    /// Sets the ideal size of the frame.
    pub const fn with_ideal_size(mut self, ideal_width: u32, ideal_height: u32) -> Self {
        self.ideal_width = Some(ideal_width);
        self.ideal_height = Some(ideal_height);
        self
    }

    /// Sets the minimum size of the frame.
    pub const fn with_min_size(mut self, min_width: u32, min_height: u32) -> Self {
        self.min_width = Some(min_width);
        self.min_height = Some(min_height);
        self
    }

    /// The horizontal alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// The vertical alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// The alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment.horizontal();
        self.vertical_alignment = alignment.vertical();
        self
    }
}

fn clamp_optional<T: Ord + Copy>(mut value: T, min: Option<T>, max: Option<T>) -> T {
    value = value.min(max.unwrap_or(value));
    value.max(min.unwrap_or(value))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout<T> {
    inner: T,
    frame_size: Dimensions,
    inner_size: Dimensions,
}

impl<V> ViewMarker for FlexFrame<V>
where
    V: ViewMarker,
{
    type Renderables = V::Renderables;
    type Transition = V::Transition;
}

impl<Captures: ?Sized, V> ViewLayout<Captures> for FlexFrame<V>
where
    V: ViewLayout<Captures>,
{
    type Sublayout = Layout<V::Sublayout>;
    type State = V::State;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.child.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.child.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let sublayout_width_offer = match offer.width {
            ProposedDimension::Exact(d) => ProposedDimension::Exact(clamp_optional(
                d,
                self.min_width,
                self.max_width.map(Into::into),
            )),
            ProposedDimension::Compact => {
                self.ideal_width
                    .map_or(ProposedDimension::Compact, |ideal_width| {
                        ProposedDimension::Exact(
                            self.min_width.map_or(ideal_width, |w| w.max(ideal_width)),
                        )
                    })
            }
            ProposedDimension::Infinite => match self.max_width {
                Some(max_width) if max_width.is_infinite() => ProposedDimension::Infinite,
                Some(max_width) => ProposedDimension::Exact(max_width.into()),
                None => ProposedDimension::Infinite,
            },
        };

        let sublayout_height_offer = match offer.height {
            ProposedDimension::Exact(d) => ProposedDimension::Exact(clamp_optional(
                d,
                self.min_height,
                self.max_height.map(Into::into),
            )),
            ProposedDimension::Compact => {
                self.ideal_height
                    .map_or(ProposedDimension::Compact, |ideal_height| {
                        ProposedDimension::Exact(
                            self.min_height
                                .map_or(ideal_height, |h| h.max(ideal_height)),
                        )
                    })
            }
            ProposedDimension::Infinite => match self.max_height {
                Some(max_height) if max_height.is_infinite() => ProposedDimension::Infinite,
                Some(max_height) => ProposedDimension::Exact(max_height.into()),
                None => ProposedDimension::Infinite,
            },
        };

        let sublayout_offer = ProposedDimensions {
            width: sublayout_width_offer,
            height: sublayout_height_offer,
        };

        let sublayout = self.child.layout(&sublayout_offer, env, captures, state);

        // restrict self size to min/max regardless of what the sublayout returns
        let sublayout_width = sublayout.resolved_size.width;
        let sublayout_height = sublayout.resolved_size.height;

        let w = self
            .max_width
            .unwrap_or(sublayout_width)
            .min(greatest_possible(sublayout_width_offer, sublayout_width))
            .max(self.min_width.map_or(sublayout_width, Into::into));

        let h = self
            .max_height
            .unwrap_or(sublayout_height)
            .min(greatest_possible(sublayout_height_offer, sublayout_height))
            .max(self.min_height.map_or(sublayout_height, Into::into));

        let resolved_size = Dimensions {
            width: w,
            height: h,
        };

        let layout = Layout {
            inner: sublayout.sublayouts,
            frame_size: resolved_size,
            inner_size: sublayout.resolved_size,
        };

        ResolvedLayout {
            sublayouts: layout,
            resolved_size,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.frame_size.width.into(),
                    layout.inner_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.frame_size.height.into(),
                    layout.inner_size.height.into(),
                ),
            );

        self.child
            .render_tree(&layout.inner, new_origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.child
            .handle_event(event, context, render_tree, captures, state)
    }
}

fn greatest_possible(proposal: ProposedDimension, ideal: Dimension) -> Dimension {
    match proposal {
        ProposedDimension::Exact(d) => d.into(),
        ProposedDimension::Compact => ideal,
        ProposedDimension::Infinite => Dimension::infinite(),
    }
}
