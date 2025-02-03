use core::cmp::max;

use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

struct ForEachEnvironment<'a, T> {
    inner_environment: &'a T,
}

impl<T: LayoutEnvironment> LayoutEnvironment for ForEachEnvironment<'_, T> {
    fn alignment(&self) -> crate::layout::Alignment {
        self.inner_environment.alignment()
    }

    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::Vertical
    }

    fn app_time(&self) -> core::time::Duration {
        self.inner_environment.app_time()
    }
}

impl<'a, T: LayoutEnvironment> From<&'a T> for ForEachEnvironment<'a, T> {
    fn from(environment: &'a T) -> Self {
        Self {
            inner_environment: environment,
        }
    }
}

pub struct ForEach<const N: usize, I: IntoIterator, V, F>
where
    F: Fn(&I::Item) -> V,
{
    iter: I,
    build_view: F,
    alignment: HorizontalAlignment,
    spacing: u16,
}

impl<const N: usize, I: IntoIterator + Copy, V, F> ForEach<N, I, V, F>
where
    V: Layout,
    F: Fn(&I::Item) -> V,
{
    pub fn new(iter: I, build_view: F) -> Self {
        Self {
            iter,
            build_view,
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }

    #[must_use]
    pub fn with_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<const N: usize, I: IntoIterator + Copy, V, F> Layout for ForEach<N, I, V, F>
where
    V: Layout,
    F: Fn(&I::Item) -> V,
{
    type Sublayout = heapless::Vec<ResolvedLayout<V::Sublayout>, N>;

    // This layout implementation trades extra work for lower memory usage as embedded is the
    // primary target environment. Views are repeatedly created for every layout call, but it
    // should be assumed that this is cheap
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let env = &ForEachEnvironment::from(env);
        let mut sublayouts: heapless::Vec<ResolvedLayout<V::Sublayout>, N> = heapless::Vec::new();

        // TODO: consolidate array init to avoid accidentally allowing them to become different
        // lengths
        let mut items: heapless::Vec<I::Item, N> = heapless::Vec::new();
        _ = self.iter.into_iter().try_for_each(|item| items.push(item));
        // if let Err(_) = result {
        //     // TODO: log an error, iterator was too large
        // }

        let mut subview_stages: heapless::Vec<(i8, bool), N> = heapless::Vec::new();
        // fill sublayouts with an initial garbage layout
        for item in &items {
            let view = (self.build_view)(item);
            _ = sublayouts.push(view.layout(offer, env));
            _ = subview_stages.push((view.priority(), view.is_empty()));
        }

        let layout_fn = |index: usize, offer: ProposedDimensions| {
            let layout = (self.build_view)(&items[index]).layout(&offer, env);
            let size = layout.resolved_size;
            sublayouts[index] = layout;
            size
        };

        let size = layout_n(&mut subview_stages, *offer, self.spacing, layout_fn);
        ResolvedLayout {
            sublayouts,
            resolved_size: size,
        }
    }
}

impl<const N: usize, I: IntoIterator + Copy, V: Renderable<C>, C, F> Renderable<C>
    for ForEach<N, I, V, F>
where
    V: Layout,
    F: Fn(&I::Item) -> V,
{
    type Renderables = heapless::Vec<V::Renderables, N>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let env = &ForEachEnvironment::from(env);

        let mut accumulated_height = 0;
        let mut renderables = heapless::Vec::new();

        for (item_layout, item) in layout.sublayouts.iter().zip(self.iter.into_iter()) {
            let aligned_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        item_layout.resolved_size.width.into(),
                    ),
                    accumulated_height,
                );
            let view = (self.build_view)(&item);
            // TODO: If we include an ID here, rows can be animated and transitioned
            let item = renderables.push(view.render_tree(item_layout, aligned_origin, env));
            assert!(item.is_ok());

            if !view.is_empty() {
                let item_height: i16 = item_layout.resolved_size.height.into();
                accumulated_height += item_height;
            }
        }

        renderables
    }
}

fn layout_n<const N: usize>(
    subviews: &mut heapless::Vec<(i8, bool), N>,
    offer: ProposedDimensions,
    spacing: u16,
    mut layout_fn: impl FnMut(usize, ProposedDimensions) -> Dimensions,
) -> Dimensions {
    let ProposedDimension::Exact(height) = offer.height else {
        let mut total_height: Dimension = 0.into();
        let mut max_width: Dimension = 0.into();
        let mut non_empty_views: u16 = 0;
        for (i, (_, is_empty)) in subviews.iter().enumerate() {
            // layout must be called at least once on every view to avoid panic unwrapping the
            // resolved layout.
            // TODO: Allowing layouts to return a cheap "empty" layout could avoid this?
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
    let mut flexibilities: [Dimension; N] = [0.into(); N];
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
        height.saturating_sub(spacing * (N.saturating_sub(num_empty_views + 1)) as u16);
    let mut last_priority_group: Option<i8> = None;
    let mut max_width: Dimension = 0.into();
    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indecies: [usize; N] = [0; N];
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
                    subviews_indecies[slice_start] = i;
                }
                core::cmp::Ordering::Equal => {
                    if slice_len == 0 {
                        slice_start = i;
                    }

                    subviews_indecies[slice_start + slice_len] = i;
                    slice_len += 1;
                }
                core::cmp::Ordering::Greater => {}
            }
        }
        last_priority_group = Some(max);

        if slice_len == 0 {
            break;
        }

        let group_indecies = &mut subviews_indecies[slice_start..slice_start + slice_len];
        group_indecies.sort_unstable_by_key(|&i| flexibilities[i]);

        let mut remaining_group_size = group_indecies.len() as u16;

        for index in group_indecies {
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
