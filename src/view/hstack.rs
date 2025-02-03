use core::cmp::max;
use paste::paste;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, LayoutDirection, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

pub struct HStack<T> {
    items: T,
    alignment: VerticalAlignment,
    spacing: u16,
}

struct HorizontalEnvironment<'a, T> {
    inner_environment: &'a T,
}

impl<T: LayoutEnvironment> LayoutEnvironment for HorizontalEnvironment<'_, T> {
    fn alignment(&self) -> crate::layout::Alignment {
        self.inner_environment.alignment()
    }
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::Horizontal
    }
    fn app_time(&self) -> core::time::Duration {
        self.inner_environment.app_time()
    }
}

impl<'a, T: LayoutEnvironment> From<&'a T> for HorizontalEnvironment<'a, T> {
    fn from(environment: &'a T) -> Self {
        Self {
            inner_environment: environment,
        }
    }
}

impl<T> HStack<T> {
    #[must_use]
    pub fn with_spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    #[must_use]
    pub fn with_alignment(self, alignment: VerticalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<T> PartialEq for HStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.spacing == other.spacing && self.alignment == other.alignment
    }
}

impl<T> HStack<T> {
    pub fn new(items: T) -> Self {
        HStack {
            items,
            alignment: VerticalAlignment::default(),
            spacing: 0,
        }
    }
}

type LayoutFn<'a> = &'a mut dyn FnMut(ProposedDimensions) -> Dimensions;

fn layout_n<const N: usize>(
    subviews: &mut [(LayoutFn, i8, bool); N],
    offer: ProposedDimensions,
    spacing: u16,
) -> Dimensions {
    let ProposedDimension::Exact(width) = offer.width else {
        let mut total_width: Dimension = 0.into();
        let mut max_height: Dimension = 0.into();
        let mut non_empty_views: u16 = 0;
        for (layout_fn, _, is_empty) in subviews {
            // layout must be called at least once on every view to avoid panic unwrapping the
            // resolved layout.
            // TODO: Allowing layouts to return a cheap "empty" layout could avoid this?
            let dimensions = layout_fn(offer);
            if *is_empty {
                continue;
            }

            total_width += dimensions.width;
            max_height = max(max_height, dimensions.height);
            non_empty_views += 1;
        }
        return Dimensions {
            width: total_width + spacing * (non_empty_views.saturating_sub(1)),
            height: max_height,
        };
    };

    // TODO: Include the minimum width, this is more important than the flexibility
    // if it exceeds the slice size the view is offered.
    // compute the "flexibility" of each view on the horizontal axis and sort by increasing
    // flexibility
    // Flexibility is defined as the difference between the responses to 0 and infinite width offers
    let mut flexibilities: [Dimension; N] = [0.into(); N];
    let mut num_empty_views = 0;

    let min_proposal = ProposedDimensions {
        width: ProposedDimension::Exact(0),
        height: offer.height,
    };

    let max_proposal = ProposedDimensions {
        width: ProposedDimension::Infinite,
        height: offer.height,
    };

    for index in 0..N {
        let minimum_dimension = subviews[index].0(min_proposal);
        // skip any further work for empty views
        if subviews[index].2 {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = subviews[index].0(max_proposal);
        flexibilities[index] = maximum_dimension.width - minimum_dimension.width;
    }

    let mut remaining_width =
        width.saturating_sub(spacing * (N.saturating_sub(num_empty_views + 1)) as u16);
    let mut last_priority_group: Option<i8> = None;
    let mut max_height: Dimension = 0.into();

    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indices: [usize; N] = [0; N];
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
        // unstable variant is no-alloc, we'll see what instability issues this creates during
        // frame animation...
        group_indices.sort_unstable_by_key(|&i| flexibilities[i]);

        let mut remaining_group_size = group_indices.len() as u16;

        for index in group_indices {
            let width_fraction =
                remaining_width / remaining_group_size + remaining_width % remaining_group_size;
            let size = subviews[*index].0(ProposedDimensions {
                width: ProposedDimension::Exact(width_fraction),
                height: offer.height,
            });
            remaining_width = remaining_width.saturating_sub(size.width.into());
            remaining_group_size -= 1;
            max_height = max_height.max(size.height);
        }
    }

    // Prevent stack from reporting oversize, even if the children misbehave
    if let ProposedDimension::Exact(offer_height) = offer.height {
        max_height = max_height.min(offer_height.into());
    }

    Dimensions {
        width: (width.saturating_sub(remaining_width)).into(),
        height: max_height,
    }
}

macro_rules! count {
    () => (0);
    ($head:tt $(, $rest:tt)*) => (1 + count!($($rest),*));
}

macro_rules! impl_layout_for_hstack {
    ($(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type: Layout),+> Layout for HStack<($($type),+)> {
            type Sublayout = ($(ResolvedLayout<$type::Sublayout>),+);

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
            ) -> ResolvedLayout<Self::Sublayout> {
                const N: usize = count!($($n),+);
                let env = &HorizontalEnvironment::from(env);

                $(
                    let mut [<c $n>]: Option<ResolvedLayout<$type::Sublayout>> = None;
                    let mut [<f $n>] = |size: ProposedDimensions| {
                        let layout = self.items.$n.layout(&size, env);
                        let size = layout.resolved_size;
                        [<c $n>] = Some(layout);
                        size
                    };
                )+

                let mut subviews: [(LayoutFn, i8, bool); N] = [
                    $(
                        (&mut paste::paste!{[<f $n>]}, self.items.$n.priority(), self.items.$n.is_empty()),
                    )+
                ];

                let total_size = layout_n(&mut subviews, *offer, self.spacing);
                ResolvedLayout {
                    sublayouts: ($(
                        [<c $n>] .unwrap()
                    ),+),
                    resolved_size: total_size,
                }
            }
        }

        impl<$($type: Renderable<C>),+, C> Renderable<C> for HStack<($($type),+)> {
            type Renderables = ($($type::Renderables),+);

            #[allow(unused_assignments)]
            fn render_tree(
                &self,
                layout: &ResolvedLayout<Self::Sublayout>,
                origin: Point,
                env: &impl LayoutEnvironment,
            ) -> Self::Renderables {
                let env = HorizontalEnvironment::from(env);
                let mut width_offset = 0;
                $(
                    let offset = origin + Point::new(
                        width_offset,
                        self.alignment.align(
                            layout.resolved_size.height.into(),
                            layout.sublayouts.$n.resolved_size.height.into(),
                        ),
                    );

                    let [<subtree_$n>] = self.items.$n.render_tree(
                        &layout.sublayouts.$n,
                        offset,
                        &env
                    );

                    if !self.items.$n.is_empty() {
                        let child_width: u16 = layout.sublayouts.$n.resolved_size.width.into();
                        width_offset += (child_width + self.spacing) as i16;
                    }
                )+

                ($([<subtree_$n>]),+)
            }
        }
    }
    }
}

impl_layout_for_hstack!((0, T0), (1, T1));
impl_layout_for_hstack!((0, T0), (1, T1), (2, T2));
impl_layout_for_hstack!((0, T0), (1, T1), (2, T2), (3, T3));
impl_layout_for_hstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_layout_for_hstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_layout_for_hstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_layout_for_hstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_layout_for_hstack!(
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
impl_layout_for_hstack!(
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
