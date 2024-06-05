use core::cmp::max;

use crate::{
    layout::{Environment, Layout, PreRender, VerticalAlignment},
    primitives::{iint, uint, Point, Size},
    render::{Render, RenderProxy, RenderTarget},
};

pub struct HStack<T> {
    items: T,
    alignment: VerticalAlignment,
    spacing: uint,
}

impl<T> HStack<T> {
    pub fn spacing(self, spacing: uint) -> Self {
        Self { spacing, ..self }
    }

    pub fn alignment(self, alignment: VerticalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<U> HStack<U> {
    pub fn one(item0: U) -> Self {
        HStack {
            items: (item0),
            alignment: VerticalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<U: Layout> Layout for HStack<U> {
    type Cache<'a> = U::Cache<'a> where U: 'a;
    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, Self::Cache<'_>> {
        let item_layout = self.items.layout(offer, env);
        PreRender {
            source_view: self,
            layout_cache: item_layout.layout_cache,
            resolved_size: item_layout.resolved_size,
        }
    }
}

impl<U, V> HStack<(U, V)> {
    pub fn two(item0: U, item1: V) -> Self {
        HStack {
            items: (item0, item1),
            alignment: VerticalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<U: Layout, V: Layout> Layout for HStack<(U, V)> {
    type Cache<'a> = (
        PreRender<'a, U, U::Cache<'a>>,
        PreRender<'a, V, V::Cache<'a>>,
    ) where U: 'a, V: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, Self::Cache<'_>> {
        const N: usize = 2;
        let mut c0: Option<PreRender<'_, U, U::Cache<'_>>> = None;
        let mut c1: Option<PreRender<'_, V, V::Cache<'_>>> = None;

        let mut f0 = |size: Size| {
            let layout = self.items.0.layout(size, env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: Size| {
            let layout = self.items.1.layout(size, env);
            let size = layout.resolved_size;
            c1 = Some(layout);
            size
        };

        // precalculate priority to avoid multiple dynamic dispatch calls
        let mut subviews: [(LayoutStage, LayoutFn, i8); N] = [
            (LayoutStage::Unsized, &mut f0, self.items.0.priority()),
            (LayoutStage::Unsized, &mut f1, self.items.1.priority()),
        ];
        let total_size = layout_n(&mut subviews, offer, self.spacing);
        PreRender {
            source_view: self,
            layout_cache: (c0.unwrap(), c1.unwrap()),
            resolved_size: total_size,
        }
    }
}

impl<'a, P, U: Layout, V: Layout> Render<P>
    for PreRender<
        '_,
        HStack<(U, V)>,
        (
            PreRender<'a, U, U::Cache<'a>>,
            PreRender<'a, V, V::Cache<'a>>,
        ),
    >
where
    PreRender<'a, U, U::Cache<'a>>: Render<P>,
    PreRender<'a, V, V::Cache<'a>>: Render<P>,
{
    fn render(&self, target: &mut impl RenderTarget<P>, env: &impl Environment) {
        let mut proxy = RenderProxy::new(
            target,
            Point::new(
                0,
                self.source_view.alignment.align(
                    self.resolved_size.height as iint,
                    self.layout_cache.0.resolved_size.height as iint,
                ),
            ),
        );
        self.layout_cache.0.render(&mut proxy, env);

        proxy.origin.x +=
            (self.layout_cache.0.resolved_size.width + self.source_view.spacing) as iint;
        proxy.origin.y = self.source_view.alignment.align(
            self.resolved_size.height as iint,
            self.layout_cache.1.resolved_size.height as iint,
        );
        self.layout_cache.1.render(&mut proxy, env);
    }
}

impl<U, V, W> HStack<(U, V, W)> {
    pub fn three(item0: U, item1: V, item2: W) -> Self {
        HStack {
            items: (item0, item1, item2),
            alignment: VerticalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<U: Layout, V: Layout, W: Layout> Layout for HStack<(U, V, W)> {
    type Cache<'a> = (
        PreRender<'a, U, U::Cache<'a>>,
        PreRender<'a, V, V::Cache<'a>>,
        PreRender<'a, W, W::Cache<'a>>,
    ) where U: 'a, V: 'a, W: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, Self::Cache<'_>> {
        const N: usize = 3;
        let mut c0: Option<PreRender<'_, U, U::Cache<'_>>> = None;
        let mut c1: Option<PreRender<'_, V, V::Cache<'_>>> = None;
        let mut c2: Option<PreRender<'_, W, W::Cache<'_>>> = None;

        let mut f0 = |size: Size| {
            let layout = self.items.0.layout(size, env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: Size| {
            let layout = self.items.1.layout(size, env);
            let size = layout.resolved_size;
            c1 = Some(layout);
            size
        };
        let mut f2 = |size: Size| {
            let layout = self.items.2.layout(size, env);
            let size = layout.resolved_size;
            c2 = Some(layout);
            size
        };

        // precalculate priority to avoid multiple dynamic dispatch calls
        let mut subviews: [(LayoutStage, LayoutFn, i8); N] = [
            (LayoutStage::Unsized, &mut f0, self.items.0.priority()),
            (LayoutStage::Unsized, &mut f1, self.items.1.priority()),
            (LayoutStage::Unsized, &mut f2, self.items.2.priority()),
        ];
        let total_size = layout_n(&mut subviews, offer, self.spacing);
        PreRender {
            source_view: self,
            layout_cache: (c0.unwrap(), c1.unwrap(), c2.unwrap()),
            resolved_size: total_size,
        }
    }
}

impl<'a, P, U: Layout, V: Layout, W: Layout> Render<P>
    for PreRender<
        '_,
        HStack<(U, V, W)>,
        (
            PreRender<'a, U, U::Cache<'a>>,
            PreRender<'a, V, V::Cache<'a>>,
            PreRender<'a, W, W::Cache<'a>>,
        ),
    >
where
    PreRender<'a, U, U::Cache<'a>>: Render<P>,
    PreRender<'a, V, V::Cache<'a>>: Render<P>,
    PreRender<'a, W, W::Cache<'a>>: Render<P>,
{
    fn render(&self, target: &mut impl RenderTarget<P>, env: &impl Environment) {
        let mut proxy = RenderProxy::new(
            target,
            Point::new(
                0,
                self.source_view.alignment.align(
                    self.resolved_size.height as iint,
                    self.layout_cache.0.resolved_size.height as iint,
                ),
            ),
        );
        self.layout_cache.0.render(&mut proxy, env);

        proxy.origin.x +=
            (self.layout_cache.0.resolved_size.width + self.source_view.spacing) as iint;
        proxy.origin.y = self.source_view.alignment.align(
            self.resolved_size.height as iint,
            self.layout_cache.1.resolved_size.height as iint,
        );
        self.layout_cache.1.render(&mut proxy, env);

        proxy.origin.x +=
            (self.layout_cache.1.resolved_size.width + self.source_view.spacing) as iint;
        proxy.origin.y = self.source_view.alignment.align(
            self.resolved_size.height as iint,
            self.layout_cache.2.resolved_size.height as iint,
        );
        self.layout_cache.2.render(&mut proxy, env);
    }
}

//
// impl<U: Layout, V: Layout, W: Layout, X: Layout> Layout for HStack<(U, V, W, X)> {
//     fn layout(&self, offer: Size, env: &dyn Environment) -> Size {
//         const N: usize = 4;
//         // precalculate priority to avoid multiple dynamic dispatch calls
//         let mut subviews: [(LayoutStage, &Layout, i8); N] = [
//             (LayoutStage::Unsized, &self.items.0, self.items.0.priority()),
//             (LayoutStage::Unsized, &self.items.1, self.items.1.priority()),
//             (LayoutStage::Unsized, &self.items.2, self.items.2.priority()),
//             (LayoutStage::Unsized, &self.items.3, self.items.3.priority()),
//         ];
//         layout_n(&mut subviews, offer, env)
//     }
// }

type LayoutFn<'a> = &'a mut dyn FnMut(Size) -> Size;

fn layout_n<const N: usize>(
    subviews: &mut [(LayoutStage, LayoutFn, i8); N],
    offer: Size,
    spacing: uint,
) -> Size {
    let mut remaining_width = offer.width - spacing * (N - 1) as uint;

    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indecies: [usize; N] = [0; N];
        let mut max = i8::MIN;
        let mut slice_start: usize = 0;
        let mut slice_len: usize = 0;
        for (i, (size, _, priority)) in subviews.iter().enumerate() {
            // skip sized subviews
            if *size != LayoutStage::Unsized {
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
        if slice_len == 0 {
            break;
        }

        // Size all the unsized views that are unwilling to shrink
        let mut group_offer = Size::new(remaining_width / slice_len as uint, offer.height);
        let remainder = remaining_width as usize % slice_len;

        // Create a slice of the subviews to be sized
        let subviews_indecies = &subviews_indecies[slice_start..slice_start + slice_len];

        // Loop until no view candidates are invalidated, or no nonfinal candidates are left
        loop {
            let mut did_layout_nonfinal_candidate = false;
            let mut nonfinal_candidate_invalidated = false;
            for (i, subview_index) in subviews_indecies.iter().enumerate() {
                if let LayoutStage::Final(_) = subviews[*subview_index].0 {
                    continue;
                }
                // Adjust the offer width to account for the remainder. The initial views will be
                // offered an extra pixel. This is mostly important for rendering character pixels
                // where the pixels are large.
                let adjusted_offer = if i < remainder {
                    Size::new(group_offer.width + 1, group_offer.height)
                } else {
                    group_offer
                };

                let subview_size = subviews.get_mut(*subview_index).unwrap().1(adjusted_offer);
                if subview_size.width > adjusted_offer.width {
                    // The subview is unwilling to shrink, reslice the remaining width
                    subviews[*subview_index].0 = LayoutStage::Final(subview_size);
                    remaining_width = remaining_width.saturating_sub(subview_size.width);
                    slice_len -= 1;
                    // on the last subview, the length will go to zero
                    group_offer.width = remaining_width
                        .checked_div(slice_len as uint)
                        .unwrap_or(group_offer.width);
                    if did_layout_nonfinal_candidate {
                        nonfinal_candidate_invalidated = true;
                        break;
                    }
                } else {
                    subviews[*subview_index].0 = LayoutStage::Candidate(subview_size);
                    did_layout_nonfinal_candidate = true;
                }
            }
            if !nonfinal_candidate_invalidated {
                break;
            }
        }
        // subtract the candidates from the remaining width
        for index in subviews_indecies.iter() {
            if let LayoutStage::Candidate(s) = subviews[*index].0 {
                remaining_width = remaining_width.saturating_sub(s.width);
            }
        }
    }

    // At this point all the subviews should have either a final or a candidate size
    // Calculate the final HStack size
    subviews.iter().fold(
        Size::new(offer.width - remaining_width, 0),
        |acc, (size, _, _)| match size {
            LayoutStage::Final(s) | LayoutStage::Candidate(s) => {
                Size::new(acc.width, max(acc.height, s.height))
            }
            _ => unreachable!(),
        },
    )
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum LayoutStage {
    Unsized,
    Candidate(Size),
    Final(Size),
}
