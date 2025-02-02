use core::cmp::max;

use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

pub struct VStack<T> {
    items: T,
    alignment: HorizontalAlignment,
    spacing: u16,
}

struct VerticalEnvironment<'a, T> {
    inner_environment: &'a T,
}

impl<T: LayoutEnvironment> LayoutEnvironment for VerticalEnvironment<'_, T> {
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

impl<T> VStack<T> {
    pub fn new(items: T) -> Self {
        Self {
            items,
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }

    #[must_use]
    pub fn with_spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    #[must_use]
    pub fn with_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

type LayoutFn<'a> = &'a mut dyn FnMut(ProposedDimensions) -> Dimensions;

fn layout_n<const N: usize>(
    subviews: &mut [(LayoutFn, i8, bool); N],
    offer: ProposedDimensions,
    spacing: u16,
) -> Dimensions {
    let ProposedDimension::Exact(height) = offer.height else {
        let mut total_height: Dimension = 0.into();
        let mut max_width: Dimension = 0.into();
        let mut non_empty_views: u16 = 0;
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

    for index in 0..N {
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
        height.saturating_sub(spacing * (N.saturating_sub(num_empty_views + 1)) as u16);
    let mut last_priority_group: Option<i8> = None;
    let mut max_width: Dimension = 0.into();
    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indecies: [usize; N] = [0; N];
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
                    subviews_indecies[slice_start] = i;
                }
                core::cmp::Ordering::Equal => {
                    if slice_len == 0 {
                        slice_start = i;
                    }

                    subviews_indecies[slice_start + slice_len] = i;
                    slice_len += 1;
                }
                _ => {}
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

macro_rules! impl_layout_for_vstack {
    ($(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type: Layout),+> Layout for VStack<($($type),+)> {
            type Sublayout = ($(ResolvedLayout<$type::Sublayout>),+);

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
            ) -> ResolvedLayout<Self::Sublayout> {
                const N: usize = count!($($n),+);
                let env = &VerticalEnvironment::from(env);

                $(
                    let mut [<c$n>]: Option<ResolvedLayout<$type::Sublayout>> = None;
                )+

                $(
                    let mut [<f$n>] = |size: ProposedDimensions| {
                        let layout = self.items.$n.layout(&size, env);
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

                let total_size = layout_n(&mut subviews, *offer, self.spacing);
                ResolvedLayout {
                    sublayouts: ($([<c$n>].unwrap()),+),
                    resolved_size: total_size,
                }
            }
        }

        impl<$($type: Renderable<C>),+, C> Renderable<C> for VStack<($($type),+)> {
            type Renderables = ($($type::Renderables),+);

            #[allow(unused_assignments)]
            fn render_tree(
                &self,
                layout: &ResolvedLayout<Self::Sublayout>,
                origin: Point,
                env: &impl LayoutEnvironment,
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
                        &layout.sublayouts.$n,
                        offset,
                        env
                    );

                    if !self.items.$n.is_empty() {
                        let child_height: u16 = layout.sublayouts.$n.resolved_size.height.into();
                        height_offset += (child_height + self.spacing) as i16;
                    }
                )+

                ($([<subtree_$n>]),+)
            }
        }
    }
    };
}

// Smarter count?
// macro_rules! count_tts {
//     () => { 0 };
//     ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
//     ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
// }

// Helper macro to count the number of elements
macro_rules! count {
    () => (0);
    ($head:tt $(, $rest:tt)*) => (1 + count!($($rest),*));
}

impl_layout_for_vstack!((0, T0), (1, T1));
impl_layout_for_vstack!((0, T0), (1, T1), (2, T2));
impl_layout_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3));
impl_layout_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_layout_for_vstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_layout_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_layout_for_vstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_layout_for_vstack!(
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
impl_layout_for_vstack!(
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
