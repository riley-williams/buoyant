use core::cmp::max;

use crate::{
    environment::LayoutEnvironment,
    event::{EventContext, EventResult},
    layout::{HorizontalAlignment, LayoutDirection, ResolvedLayout},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker, modifier::FixedSize},
};

use core::cell::RefCell;

/// A stack of heterogeneous views that arranges its children vertically.
///
/// [`VStack`] attempts to fairly distribute the available height among its children,
/// laying out groups of children based on priority.
#[derive(Debug, Clone)]
pub struct VStack<T> {
    items: T,
    alignment: HorizontalAlignment,
    spacing: u32,
}

struct VerticalEnvironment<'a, T> {
    inner_environment: &'a T,
}

impl<T: LayoutEnvironment> LayoutEnvironment for VerticalEnvironment<'_, T> {
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::Vertical
    }

    fn app_time(&self) -> core::time::Duration {
        self.inner_environment.app_time()
    }
}

impl<'a, T: LayoutEnvironment> From<&'a T> for VerticalEnvironment<'a, T> {
    fn from(environment: &'a T) -> Self {
        Self {
            inner_environment: environment,
        }
    }
}

impl<T> PartialEq for VStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.spacing == other.spacing && self.alignment == other.alignment
    }
}

impl<T: ViewMarker> VStack<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(items: T) -> Self {
        Self {
            items,
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }

    /// Sets the spacing between items in the stack.
    #[must_use]
    pub fn with_spacing(self, spacing: u32) -> Self {
        Self { spacing, ..self }
    }

    /// Sets the horizontal alignment to use when placing child views of different widths.
    #[must_use]
    pub fn with_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self { alignment, ..self }
    }

    /// Lays out and renders the stack at its ideal height rather than attempting to distribute
    /// space fairly.
    ///
    /// This can significantly reduce the cost of layout when the view is known to fit in the
    /// available space. However, child views which have no intrinsic ideal size, such as
    /// shapes, may become zero-sized if not contained within e.g. `.fixed_frame`.
    #[must_use]
    pub fn lazy(self) -> FixedSize<Self>
    where
        Self: ViewMarker,
    {
        FixedSize::new(false, true, self)
    }
}

type LayoutFn<'a> = &'a mut dyn FnMut(ProposedDimensions) -> Dimensions;

fn layout_n(
    subviews: &mut [(LayoutFn, i8, bool)],
    offer: ProposedDimensions,
    spacing: u32,
    flexibilities: &mut [Dimension],
    subviews_indices: &mut [usize],
) -> Dimensions {
    let subview_count = subviews.len();
    // These asserts should be provably true, and optimized away in release builds
    assert!(subview_count <= flexibilities.len());
    assert!(subview_count <= subviews_indices.len());

    // Ensure initial values are as expected
    debug_assert!(!flexibilities.iter().any(|e| *e != Dimension::new(0)));
    debug_assert!(!subviews_indices.iter().any(|e| *e != 0));

    let ProposedDimension::Exact(height) = offer.height else {
        let mut total_height: Dimension = 0u32.into();
        let mut max_width: Dimension = 0u32.into();
        let mut non_empty_views: u32 = 0;
        for (layout_fn, _, is_empty) in subviews {
            // layout must be called at least once on every view to avoid panic unwrapping the
            // resolved layout.
            // TODO: Allowing layouts to return a cheap "empty" layout could avoid this?
            let dimensions = layout_fn(offer);
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
    let mut num_empty_views = 0;
    let min_proposal = ProposedDimensions {
        width: offer.width,
        height: ProposedDimension::Exact(0),
    };
    let max_proposal = ProposedDimensions {
        width: offer.width,
        height: ProposedDimension::Infinite,
    };

    for index in 0..subview_count {
        let minimum_dimension = subviews[index].0(min_proposal);
        // skip any further work for empty views
        if subviews[index].2 {
            num_empty_views += 1;
            continue;
        }

        let maximum_dimension = subviews[index].0(max_proposal);
        flexibilities[index] = maximum_dimension.height - minimum_dimension.height;
    }

    let mut remaining_height =
        height.saturating_sub(spacing * (subview_count.saturating_sub(num_empty_views + 1)) as u32);
    let mut last_priority_group: Option<i8> = None;
    let mut max_width: Dimension = 0u32.into();
    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut max = i8::MIN;
        let mut slice_start: usize = 0;
        let mut slice_len: usize = 0;
        for (i, (_, priority, is_empty)) in subviews.iter().enumerate() {
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
            let size = subviews[*index].0(ProposedDimensions {
                width: offer.width,
                height: ProposedDimension::Exact(height_fraction),
            });
            remaining_height = remaining_height.saturating_sub(size.height.into());
            remaining_group_size -= 1;
            max_width = max_width.max(size.width);
        }
    }

    // Prevent stack from reporting oversize, even if the children misbehave
    if let ProposedDimension::Exact(offer_width) = offer.width {
        max_width = max_width.min(offer_width.into());
    }

    Dimensions {
        width: max_width,
        height: (height.saturating_sub(remaining_height)).into(),
    }
}

use paste::paste;

// Helper macro to count the number of elements
macro_rules! count {
    () => (const { 0 });
    ($head:tt $(, $rest:tt)*) => (const { 1 + count!($($rest),*) });
}

macro_rules! impl_view_for_vstack {
    ($(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type),+> ViewMarker for VStack<($($type),+)>
        where
            $($type: ViewMarker),+
        {
            type Renderables = ($($type::Renderables),+);
            type Transition = crate::transition::Opacity;
        }

        impl<$($type),+, Captures: ?Sized> ViewLayout<Captures> for VStack<($($type),+)>
        where
            $($type: ViewLayout<Captures>),+
        {
            type State = ($($type::State),+);
            // FIXME: Could just be width + sublayouts?
            type Sublayout = ResolvedLayout<($(ResolvedLayout<$type::Sublayout>),+)>;

            fn transition(&self) -> Self::Transition {
                crate::transition::Opacity
            }

            fn build_state(&self, captures: &mut Captures) -> Self::State {
                ($(self.items.$n.build_state(captures)),+)
            }

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> ResolvedLayout<Self::Sublayout> {
                const N: usize = count!($($n),+);
                let env = &VerticalEnvironment::from(env);

                let captures_cell = RefCell::new(captures);

                $(
                    let mut [<c$n>]: Option<ResolvedLayout<$type::Sublayout>> = None;
                )+

                $(
                    let mut [<f$n>] = |size: ProposedDimensions| {
                        // Calls to this layout cannot overlap, so this borrow will not conflict
                        let mut captures = captures_cell.borrow_mut();
                        let layout = self.items.$n.layout(&size, env, &mut *captures, &mut state.$n);
                        let size = layout.resolved_size;
                        [<c$n>] = Some(layout);
                        size
                    };
                )+

                let mut subviews: [(LayoutFn, i8, bool); N] = [
                    $(
                        (&mut [<f$n>], self.items.$n.priority(), self.items.$n.is_empty()),
                    )+
                ];

                let mut flexibilities: [Dimension; N] = [Dimension::new(0); N];
                let mut subviews_indices: [usize; N] = [0; N];
                let total_size = layout_n(&mut subviews, *offer, self.spacing, &mut flexibilities, &mut subviews_indices);
                ResolvedLayout {
                    sublayouts: ($([<c$n>].unwrap()),+),
                    resolved_size: total_size,
                }.nested()
            }

            #[allow(unused_assignments)]
            fn render_tree(
                &self,
                layout: &Self::Sublayout,
                origin: Point,
                env: &impl LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> Self::Renderables {
                let env = &VerticalEnvironment::from(env);
                let mut height_offset = 0;

                $(
                    let offset = origin + Point::new(
                        self.alignment.align(
                            layout.resolved_size.width.into(),
                            layout.sublayouts.$n.resolved_size.width.into(),
                        ),
                        height_offset,
                    );

                    let [<subtree_$n>] = self.items.$n.render_tree(
                        &layout.sublayouts.$n.sublayouts,
                        offset,
                        env,
                        captures,
                        &mut state.$n,
                    );

                    if !self.items.$n.is_empty() {
                        let child_height: u32 = layout.sublayouts.$n.resolved_size.height.into();
                        height_offset += (child_height + self.spacing) as i32;
                    }
                )+

                ($([<subtree_$n>]),+)
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
                $(
                    result.merge(self.items.$n.handle_event(event, context, &mut render_tree.$n, captures, &mut state.$n));
                    if result.handled {
                        return result;
                    }
                )+
                result
            }
        }
    }
    };
}

impl_view_for_vstack!((0, T0), (1, T1));
impl_view_for_vstack!((0, T0), (1, T1), (2, T2));
impl_view_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3));
impl_view_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_view_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_view_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_view_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_view_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8)
);
impl_view_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8),
    (9, T9)
);

// Implement single-item conformance for convenience, although it does nothing
impl<T> ViewMarker for VStack<(T,)>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
    type Transition = crate::transition::Opacity;
}

impl<Captures, T> ViewLayout<Captures> for VStack<(T,)>
where
    T: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn transition(&self) -> Self::Transition {
        crate::transition::Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.items.0.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.items.0.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.items
            .0
            .render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::event::Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.items
            .0
            .handle_event(event, context, render_tree, captures, state)
    }
}
