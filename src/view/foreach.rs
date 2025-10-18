use core::cmp::max;

use crate::{
    environment::LayoutEnvironment,
    event::{EventContext, EventResult},
    layout::{HorizontalAlignment, LayoutDirection, ResolvedLayout, SafeAreaInsets},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone)]
struct ForEachEnvironment<'a, T> {
    inner_environment: &'a T,
}

impl<T: LayoutEnvironment> LayoutEnvironment for ForEachEnvironment<'_, T> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::Vertical
    }

    fn app_time(&self) -> core::time::Duration {
        self.inner_environment.app_time()
    }

    fn safe_area_insets(&self) -> &SafeAreaInsets {
        self.inner_environment.safe_area_insets()
    }
}

impl<'a, T: LayoutEnvironment> From<&'a T> for ForEachEnvironment<'a, T> {
    fn from(environment: &'a T) -> Self {
        Self {
            inner_environment: environment,
        }
    }
}

/// Prefer using `ForEach::new` to avoid needing to specify
/// type parameters.
#[derive(Debug, Clone)]
pub struct ForEachView<'a, const N: usize, I, V, F>
where
    F: Fn(&'a I) -> V,
{
    items: &'a [I],
    build_view: F,
    alignment: HorizontalAlignment,
    spacing: u32,
}

/// A homogeneous collection of views, arranged vertically. Up to N views
/// will be rendered.
///
/// Alignment and spacing can be configured, and have the same behavior
/// as with `VStack`.
///
/// # Examples
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

impl<const N: usize> ForEach<N> {
    #[allow(missing_docs)]
    #[expect(clippy::new_ret_no_self)]
    pub fn new<'a, I, V, F>(items: &'a [I], build_view: F) -> ForEachView<'a, N, I, V, F>
    where
        F: Fn(&'a I) -> V,
    {
        ForEachView {
            items,
            build_view,
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<'a, const N: usize, I, V, F> ForEachView<'a, N, I, V, F>
where
    F: Fn(&'a I) -> V,
{
    /// Sets an alignment strategy for when child views vary in width
    #[must_use]
    pub const fn with_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Inserts spacing between child views
    #[must_use]
    pub const fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, const N: usize, I, V, F> ViewMarker for ForEachView<'a, N, I, V, F>
where
    F: Fn(&'a I) -> V,
    V: ViewMarker,
{
    type Renderables = heapless::Vec<V::Renderables, N>;
    type Transition = Opacity;
}

impl<'a, const N: usize, I, V, F, Captures: ?Sized> ViewLayout<Captures>
    for ForEachView<'a, N, I, V, F>
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
        let env = &ForEachEnvironment::from(env);
        let mut sublayouts: heapless::Vec<ResolvedLayout<V::Sublayout>, N> = heapless::Vec::new();
        let mut subview_stages: heapless::Vec<(i8, bool), N> = heapless::Vec::new();

        // fill sublayouts with an initial garbage layout
        // TODO: guess there are no empty views, often no extra work needed?
        for (i, item) in self.items.iter().enumerate() {
            let view = (self.build_view)(item);
            let Some(item_state) = state.get_mut(i) else {
                break;
            };
            _ = sublayouts.push(view.layout(offer, env, captures, item_state));
            _ = subview_stages.push((view.priority(), view.is_empty()));
        }

        let layout_fn = |index: usize, offer: ProposedDimensions| {
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

        let size = layout_n(&subview_stages, *offer, self.spacing, layout_fn);
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
        let env = &ForEachEnvironment::from(env);

        let mut accumulated_height = 0;
        let mut renderables = heapless::Vec::new();

        for ((item_layout, item), item_state) in layout.sublayouts.iter().zip(self.items).zip(state)
        {
            let aligned_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        item_layout.resolved_size.width.into(),
                    ),
                    accumulated_height,
                );
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
                let item_height: i32 = item_layout.resolved_size.height.into();
                accumulated_height += item_height + self.spacing as i32;
            }
        }

        renderables
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        let mut result = EventResult::default();
        for ((item, item_state), render_tree) in self
            .items
            .iter()
            .zip(state.iter_mut())
            .zip(render_tree.iter_mut())
        {
            let view = (self.build_view)(item);
            result.merge(view.handle_event(event, context, render_tree, captures, item_state));
            if result.handled {
                return result;
            }
        }
        result
    }
}

fn layout_n<const N: usize>(
    subviews: &heapless::Vec<(i8, bool), N>,
    offer: ProposedDimensions,
    spacing: u32,
    mut layout_fn: impl FnMut(usize, ProposedDimensions) -> Dimensions,
) -> Dimensions {
    let ProposedDimension::Exact(height) = offer.height else {
        // Compact or infinite offer
        let mut total_height: Dimension = 0u32.into();
        let mut max_width: Dimension = 0u32.into();
        let mut non_empty_views: u32 = 0;
        for (i, (_, is_empty)) in subviews.iter().enumerate() {
            // layout must be called at least once on every view to avoid panic unwrapping the
            // resolved layout.
            let dimensions = layout_fn(i, offer);
            if *is_empty {
                continue;
            }

            total_height += dimensions.height;
            max_width = max(max_width, dimensions.width);
            non_empty_views += 1;
        }
        return Dimensions {
            width: max_width,
            height: total_height + spacing * (non_empty_views.saturating_sub(1)),
        };
    };

    // compute the "flexibility" of each view on the vertical axis and sort by decreasing
    // flexibility
    // Flexibility is defined as the difference between the responses to 0 and infinite height offers
    let mut flexibilities: [Dimension; N] = [0u32.into(); N];
    let mut num_empty_views = 0;
    let min_proposal = ProposedDimensions {
        width: offer.width,
        height: ProposedDimension::Exact(0),
    };

    let max_proposal = ProposedDimensions {
        width: offer.width,
        height: ProposedDimension::Infinite,
    };

    for index in 0..subviews.len() {
        let minimum_dimension = layout_fn(index, min_proposal);
        // skip any further work for empty views
        if subviews[index].1 {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = layout_fn(index, max_proposal);
        flexibilities[index] = maximum_dimension.height - minimum_dimension.height;
    }

    let mut remaining_height =
        height.saturating_sub(spacing * (N.saturating_sub(num_empty_views + 1)) as u32);
    let mut last_priority_group: Option<i8> = None;
    let mut max_width: Dimension = 0u32.into();
    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indices: [usize; N] = [0; N];
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

        for index in group_indices {
            let height_fraction =
                remaining_height / remaining_group_size + remaining_height % remaining_group_size;
            let size = layout_fn(
                *index,
                ProposedDimensions {
                    width: offer.width,
                    height: ProposedDimension::Exact(height_fraction),
                },
            );
            remaining_height = remaining_height.saturating_sub(size.height.into());
            remaining_group_size -= 1;
            max_width = max_width.max(size.width);
        }
    }

    Dimensions {
        width: max_width,
        height: (height.saturating_sub(remaining_height)).into(),
    }
}
