//! `ForEach` view. See [`ForEach`] for details.

use core::cmp::max;

use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, LayoutDirection, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// A homogeneous collection of views, arranged vertically. Up to N views
/// will be rendered.
///
/// Alignment and spacing can be configured, and have the same behavior
/// as with `VStack`.
///
/// Example:
///
/// ```
/// use buoyant::view::{ForEach, Text};
/// use buoyant::layout::HorizontalAlignment;
/// use embedded_graphics::mono_font::ascii::FONT_6X13;
///
/// let mut names = heapless::Vec::<String, 10>::new();
/// names.push("Alice".to_string()).unwrap();
/// names.push("Bob".to_string()).unwrap();
/// names.push("Charlie".to_string()).unwrap();
///
/// ForEach::<10>::new(&names, |name| {
///     Text::new(name, &FONT_6X13)
/// })
///     .with_spacing(12)
///     .with_alignment(HorizontalAlignment::Leading);
/// ```
#[expect(missing_debug_implementations)]
pub struct ForEach<const N: usize> {}

/// Prefer using `ForEach::new` to avoid needing to specify
/// type parameters.
#[derive(Debug, Clone)]
pub struct ForEachView<'a, const N: usize, I, V, F, D = Vertical>
where
    F: Fn(&'a I) -> V,
{
    items: &'a [I],
    build_view: F,
    direction_with_alignment: D,
    spacing: u32,
}

/// Specifies a vertical direction for `ForEach`. Used in [`ForEachView::with_direction`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Vertical(pub HorizontalAlignment);
/// Specifies a horizontal direction for `ForEach`. Used in [`ForEachView::with_direction`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Horizontal(pub VerticalAlignment);

pub trait ForEachDirection: Copy + Default {
    fn layout_dir() -> LayoutDirection;
    fn align(&self, accumulated: i32, layout_size: Dimensions, item_size: Dimensions) -> Point;
    fn size_of(&self, item_size: Dimensions) -> i32;
}

impl ForEachDirection for Vertical {
    fn layout_dir() -> LayoutDirection {
        LayoutDirection::Vertical
    }

    fn align(&self, accumulated: i32, layout_size: Dimensions, item_size: Dimensions) -> Point {
        let alignment = &self.0;
        let width = alignment.align(layout_size.width.into(), item_size.width.into());
        Point::new(width, accumulated)
    }

    fn size_of(&self, item_size: Dimensions) -> i32 {
        item_size.height.into()
    }
}

impl ForEachDirection for Horizontal {
    fn layout_dir() -> LayoutDirection {
        LayoutDirection::Horizontal
    }

    fn align(&self, accumulated: i32, layout_size: Dimensions, item_size: Dimensions) -> Point {
        let alignment = &self.0;
        let height = alignment.align(layout_size.height.into(), item_size.height.into());
        Point::new(accumulated, height)
    }

    fn size_of(&self, item_size: Dimensions) -> i32 {
        item_size.width.into()
    }
}

#[derive(Debug, Clone)]
struct ForEachEnvironment<'a, T, D> {
    inner_environment: &'a T,
    _direction: D,
}

impl<T: LayoutEnvironment, D: ForEachDirection> LayoutEnvironment for ForEachEnvironment<'_, T, D> {
    fn layout_direction(&self) -> LayoutDirection {
        D::layout_dir()
    }

    fn app_time(&self) -> core::time::Duration {
        self.inner_environment.app_time()
    }
}

impl<'a, T: LayoutEnvironment, D> ForEachEnvironment<'a, T, D> {
    fn new(environment: &'a T, direction: D) -> Self {
        Self {
            inner_environment: environment,
            _direction: direction,
        }
    }
}

impl<const N: usize> ForEach<N> {
    #[allow(missing_docs)]
    #[expect(clippy::new_ret_no_self)]
    #[deprecated(since = "0.6.0", note = "Use `ForEach::<N>::new_vertical`")]
    pub fn new<'a, I, V, F>(items: &'a [I], build_view: F) -> ForEachView<'a, N, I, V, F, Vertical>
    where
        F: Fn(&'a I) -> V,
    {
        Self::new_vertical(items, build_view)
    }

    #[allow(missing_docs)]
    pub fn new_vertical<'a, I, V, F>(
        items: &'a [I],
        build_view: F,
    ) -> ForEachView<'a, N, I, V, F, Vertical>
    where
        F: Fn(&'a I) -> V,
    {
        ForEachView {
            items,
            build_view,
            direction_with_alignment: Vertical::default(),
            spacing: 0,
        }
    }

    #[allow(missing_docs)]
    pub fn new_horizontal<'a, I, V, F>(
        items: &'a [I],
        build_view: F,
    ) -> ForEachView<'a, N, I, V, F, Horizontal>
    where
        F: Fn(&'a I) -> V,
    {
        ForEachView {
            items,
            build_view,
            direction_with_alignment: Horizontal::default(),
            spacing: 0,
        }
    }
}

impl<'a, const N: usize, I, V, F> ForEachView<'a, N, I, V, F, Vertical>
where
    F: Fn(&'a I) -> V,
{
    /// Sets an alignment strategy for when child views vary in size
    #[must_use]
    pub fn with_alignment(self, alignment: HorizontalAlignment) -> Self {
        ForEachView {
            items: self.items,
            build_view: self.build_view,
            direction_with_alignment: Vertical(alignment),
            spacing: self.spacing,
        }
    }
}

impl<'a, const N: usize, I, V, F> ForEachView<'a, N, I, V, F, Horizontal>
where
    F: Fn(&'a I) -> V,
{
    /// Sets an alignment strategy for when child views vary in size
    #[must_use]
    pub fn with_alignment(self, alignment: VerticalAlignment) -> Self {
        ForEachView {
            items: self.items,
            build_view: self.build_view,
            direction_with_alignment: Horizontal(alignment),
            spacing: self.spacing,
        }
    }
}

impl<'a, const N: usize, I, V, F, D> ForEachView<'a, N, I, V, F, D>
where
    D: ForEachDirection,
    F: Fn(&'a I) -> V,
{
    /// Sets an direction and alignment strategy for when child views vary in size
    #[must_use]
    pub fn with_direction<D2>(self, direction: D2) -> ForEachView<'a, N, I, V, F, D2>
    where
        D2: ForEachDirection, // const shinenigans
    {
        ForEachView {
            items: self.items,
            build_view: self.build_view,
            direction_with_alignment: direction,
            spacing: self.spacing,
        }
    }

    /// Inserts spacing between child views
    #[must_use]
    pub const fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, const N: usize, I, V, F, D> ViewMarker for ForEachView<'a, N, I, V, F, D>
where
    F: Fn(&'a I) -> V,
    V: ViewMarker,
{
    type Renderables = heapless::Vec<V::Renderables, N>;
    type Transition = Opacity;
}

impl<'a, const N: usize, I, V, F, Captures: ?Sized, D: ForEachDirection> ViewLayout<Captures>
    for ForEachView<'a, N, I, V, F, D>
where
    V: ViewLayout<Captures>,
    F: Fn(&'a I) -> V,
{
    type Sublayout = heapless::Vec<ResolvedLayout<V::Sublayout>, N>;
    type State = heapless::Vec<V::State, N>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let mut state = heapless::Vec::new();
        for item in self.items {
            let view = (self.build_view)(item);
            _ = state.push(view.build_state(captures));
        }
        state
    }
    // This layout implementation trades extra work for lower memory usage as embedded is the
    // primary target environment. Views are repeatedly created for every layout call, but it
    // should be assumed that this is cheap
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let env = &ForEachEnvironment::new(env, self.direction_with_alignment);
        let mut sublayouts: heapless::Vec<ResolvedLayout<V::Sublayout>, N> = heapless::Vec::new();
        let mut subview_stages: heapless::Vec<(i8, bool), N> = heapless::Vec::new();

        // TODO: guess there are no empty views, often no extra work needed?
        for item in self.items {
            let view = (self.build_view)(item);
            _ = sublayouts.push(ResolvedLayout::default());
            _ = subview_stages.push((view.priority(), view.is_empty()));
        }

        let mut layout_fn = |index: usize, offer: ProposedDimensions| {
            let layout = (self.build_view)(&self.items[index]).layout(
                &offer,
                env,
                captures,
                &mut state[index],
            );
            let size = layout.resolved_size;
            sublayouts[index] = layout;
            size
        };

        let direction = D::layout_dir();

        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indices: [usize; N] = [0; N];
        let mut flexibilities: [Dimension; N] = [0u32.into(); N];
        let size = layout_n(
            &subview_stages,
            &mut subviews_indices,
            &mut flexibilities,
            direction,
            *offer,
            self.spacing,
            &mut layout_fn,
        );
        ResolvedLayout {
            sublayouts,
            resolved_size: size,
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let env = &ForEachEnvironment::new(env, self.direction_with_alignment);

        let mut accumulated_size = 0;
        let mut renderables = heapless::Vec::new();

        for ((item_layout, item), item_state) in layout.sublayouts.iter().zip(self.items).zip(state)
        {
            let aligned_origin = self.direction_with_alignment.align(
                accumulated_size,
                layout.resolved_size,
                item_layout.resolved_size,
            ) + origin;

            let view = (self.build_view)(item);

            // TODO: If we include an ID here, rows can be animated and transitioned
            let item = renderables.push(view.render_tree(
                item_layout,
                aligned_origin,
                env,
                captures,
                item_state,
            ));
            assert!(item.is_ok());

            if !view.is_empty() {
                accumulated_size += self
                    .direction_with_alignment
                    .size_of(item_layout.resolved_size)
                    + self.spacing as i32;
            }
        }

        renderables
    }
}

#[allow(clippy::too_many_lines)]
fn layout_n(
    subviews: &[(i8, bool)],
    subviews_indices: &mut [usize],
    flexibilities: &mut [Dimension],
    direction: LayoutDirection,
    offer: ProposedDimensions,
    spacing: u32,
    layout_fn: &mut dyn FnMut(usize, ProposedDimensions) -> Dimensions,
) -> Dimensions {
    let proposed_dimension = match direction {
        LayoutDirection::Horizontal => offer.width,
        LayoutDirection::Vertical => offer.height,
    };
    let ProposedDimension::Exact(size) = proposed_dimension else {
        // Compact or infinite offer
        let mut total_size: Dimension = 0u32.into();
        let mut max_cross_size: Dimension = 0u32.into();
        let mut non_empty_views: u32 = 0;
        for (i, (_, is_empty)) in subviews.iter().enumerate() {
            // layout must be called at least once on every view to avoid panic unwrapping the
            // resolved layout.
            let dimensions = layout_fn(i, offer);
            if *is_empty {
                continue;
            }

            let (size, cross_size) = match direction {
                LayoutDirection::Vertical => (dimensions.height, dimensions.width),
                LayoutDirection::Horizontal => (dimensions.width, dimensions.height),
            };
            total_size += size;
            max_cross_size = max(max_cross_size, cross_size);
            non_empty_views += 1;
        }
        return match direction {
            LayoutDirection::Horizontal => Dimensions {
                width: total_size + spacing * (non_empty_views.saturating_sub(1)),
                height: max_cross_size,
            },
            LayoutDirection::Vertical => Dimensions {
                width: max_cross_size,
                height: total_size + spacing * (non_empty_views.saturating_sub(1)),
            },
        };
    };

    // compute the "flexibility" of each view on the vertical axis and sort by decreasing
    // flexibility
    // Flexibility is defined as the difference between the responses to 0 and infinite height offers
    flexibilities.fill(Dimension::from(0u32));
    let mut num_empty_views = 0;
    let (min_proposal, max_proposal) = match direction {
        LayoutDirection::Horizontal => (
            ProposedDimensions {
                width: ProposedDimension::Exact(0),
                height: offer.height,
            },
            ProposedDimensions {
                width: ProposedDimension::Infinite,
                height: offer.height,
            },
        ),
        LayoutDirection::Vertical => (
            ProposedDimensions {
                width: offer.width,
                height: ProposedDimension::Exact(0),
            },
            ProposedDimensions {
                width: offer.width,
                height: ProposedDimension::Infinite,
            },
        ),
    };

    for index in 0..subviews.len() {
        let minimum_dimension = layout_fn(index, min_proposal);
        // skip any further work for empty views
        if subviews[index].1 {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = layout_fn(index, max_proposal);
        flexibilities[index] = match direction {
            LayoutDirection::Horizontal => maximum_dimension.width - minimum_dimension.width,
            LayoutDirection::Vertical => maximum_dimension.height - minimum_dimension.height,
        };
    }

    let len = subviews.len() as u32;
    let mut remaining_size = size.saturating_sub(spacing * len.saturating_sub(num_empty_views + 1));
    let mut last_priority_group: Option<i8> = None;
    let mut max_cross_size: Dimension = 0u32.into();
    loop {
        subviews_indices.fill(0);
        let mut max = i8::MIN;
        let mut slice_start: usize = 0;
        let mut slice_len: usize = 0;
        for (i, (priority, is_empty)) in subviews.iter().enumerate() {
            if last_priority_group.is_some_and(|p| p <= *priority) || *is_empty {
                continue;
            }
            match max.cmp(priority) {
                core::cmp::Ordering::Less => {
                    max = *priority;
                    slice_start = i;
                    slice_len = 1;
                    subviews_indices[slice_start] = i;
                }
                core::cmp::Ordering::Equal => {
                    if slice_len == 0 {
                        slice_start = i;
                    }

                    subviews_indices[slice_start + slice_len] = i;
                    slice_len += 1;
                }
                core::cmp::Ordering::Greater => {}
            }
        }
        last_priority_group = Some(max);

        if slice_len == 0 {
            break;
        }

        let group_indices = &mut subviews_indices[slice_start..slice_start + slice_len];
        group_indices.sort_unstable_by_key(|&i| flexibilities[i]);

        let mut remaining_group_size = group_indices.len() as u32;

        match direction {
            LayoutDirection::Horizontal => {
                for index in group_indices {
                    let width_fraction = remaining_size / remaining_group_size
                        + remaining_size % remaining_group_size;
                    let size = layout_fn(
                        *index,
                        ProposedDimensions {
                            width: ProposedDimension::Exact(width_fraction),
                            height: offer.height,
                        },
                    );
                    remaining_size = remaining_size.saturating_sub(size.width.into());
                    remaining_group_size -= 1;
                    max_cross_size = max_cross_size.max(size.height);
                }
            }
            LayoutDirection::Vertical => {
                for index in group_indices {
                    let height_fraction = remaining_size / remaining_group_size
                        + remaining_size % remaining_group_size;
                    let size = layout_fn(
                        *index,
                        ProposedDimensions {
                            width: offer.width,
                            height: ProposedDimension::Exact(height_fraction),
                        },
                    );
                    remaining_size = remaining_size.saturating_sub(size.height.into());
                    remaining_group_size -= 1;
                    max_cross_size = max_cross_size.max(size.width);
                }
            }
        }
    }

    match direction {
        LayoutDirection::Horizontal => Dimensions {
            width: (size.saturating_sub(remaining_size)).into(),
            height: max_cross_size,
        },
        LayoutDirection::Vertical => Dimensions {
            width: max_cross_size,
            height: (size.saturating_sub(remaining_size)).into(),
        },
    }
}
