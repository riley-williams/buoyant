use core::cmp::{max, min};

use crate::{
    environment::Environment,
    layout::{HorizontalAlignment, Layout, LayoutDirection, ResolvedLayout},
    pixel::RenderUnit,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};

pub struct VStack<T> {
    items: T,
    alignment: HorizontalAlignment,
    spacing: u16,
}

struct VerticalEnvironment<'a> {
    pub environment: &'a dyn Environment,
}

impl Environment for VerticalEnvironment<'_> {
    fn alignment(&self) -> crate::layout::Alignment {
        self.environment.alignment()
    }
    fn layout_direction(&self) -> LayoutDirection {
        LayoutDirection::Vertical
    }
}

impl<'a> From<&'a dyn Environment> for VerticalEnvironment<'a> {
    fn from(environment: &'a dyn Environment) -> Self {
        Self { environment }
    }
}

impl<T> VStack<T> {
    pub fn spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    pub fn alignment(self, alignment: HorizontalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<U, V> VStack<(U, V)> {
    pub fn two(item0: U, item1: V) -> Self {
        VStack {
            items: (item0, item1),
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<U: Layout, V: Layout> Layout for VStack<(U, V)> {
    type Sublayout<'a> = (
        ResolvedLayout<U::Sublayout<'a>>,
        ResolvedLayout<V::Sublayout<'a>>,
    ) where U: 'a, V: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> ResolvedLayout<Self::Sublayout<'_>> {
        const N: usize = 2;
        let env = &VerticalEnvironment::from(env);
        let mut c0: Option<ResolvedLayout<U::Sublayout<'_>>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout<'_>>> = None;

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
        ResolvedLayout {
            sublayouts: (c0.unwrap(), c1.unwrap()),
            resolved_size: total_size,
        }
    }
}

impl<'a, Pixel, U: Layout, V: Layout>
    Render<
        Pixel,
        (
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
        ),
    > for VStack<(U, V)>
where
    U: Render<Pixel, U::Sublayout<'a>>,
    V: Render<Pixel, V::Sublayout<'a>>,
    Pixel: RenderUnit,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<(
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
        )>,
        env: &dyn Environment,
    ) {
        let env = &VerticalEnvironment::from(env);

        let original_window = target.window();
        let mut height = 0;

        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.0.resolved_size.width as i16,
                    ),
                    height,
                ),
        );

        self.items.0.render(target, &layout.sublayouts.0, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        target.set_window_origin(Point::new(
            original_window.origin.x
                + self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.1.resolved_size.width as i16,
                ),
            height,
        ));

        self.items.1.render(target, &layout.sublayouts.1, env);
        target.set_window(original_window);
    }
}

impl<U, V, W> VStack<(U, V, W)> {
    pub fn three(item0: U, item1: V, item2: W) -> Self {
        VStack {
            items: (item0, item1, item2),
            alignment: HorizontalAlignment::default(),
            spacing: 0,
        }
    }
}

impl<U: Layout, V: Layout, W: Layout> Layout for VStack<(U, V, W)> {
    type Sublayout<'a> = (
        ResolvedLayout<U::Sublayout<'a>>,
        ResolvedLayout<V::Sublayout<'a>>,
        ResolvedLayout<W::Sublayout<'a>>,
    ) where U: 'a, V: 'a, W: 'a;

    fn layout(&self, offer: Size, env: &dyn Environment) -> ResolvedLayout<Self::Sublayout<'_>> {
        const N: usize = 3;
        let env = &VerticalEnvironment::from(env);

        let mut c0: Option<ResolvedLayout<U::Sublayout<'_>>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout<'_>>> = None;
        let mut c2: Option<ResolvedLayout<W::Sublayout<'_>>> = None;

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
        ResolvedLayout {
            sublayouts: (c0.unwrap(), c1.unwrap(), c2.unwrap()),
            resolved_size: total_size,
        }
    }
}

impl<'a, Pixel, U: Layout, V: Layout, W: Layout>
    Render<
        Pixel,
        (
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
            ResolvedLayout<W::Sublayout<'a>>,
        ),
    > for VStack<(U, V, W)>
where
    U: Render<Pixel, U::Sublayout<'a>>,
    V: Render<Pixel, V::Sublayout<'a>>,
    W: Render<Pixel, W::Sublayout<'a>>,
    Pixel: RenderUnit,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<(
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
            ResolvedLayout<W::Sublayout<'a>>,
        )>,
        env: &dyn Environment,
    ) {
        let env = &VerticalEnvironment::from(env);

        let original_window = target.window();
        let mut height = 0;

        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.0.resolved_size.width as i16,
                    ),
                    height,
                ),
        );
        self.items.0.render(target, &layout.sublayouts.0, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.1.resolved_size.width as i16,
                    ),
                    height,
                ),
        );

        self.items.1.render(target, &layout.sublayouts.1, env);

        height += (layout.sublayouts.1.resolved_size.height + self.spacing) as i16;
        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.2.resolved_size.width as i16,
                    ),
                    height,
                ),
        );
        self.items.2.render(target, &layout.sublayouts.2, env);

        target.set_window(original_window);
    }
}

type LayoutFn<'a> = &'a mut dyn FnMut(Size) -> Size;

fn layout_n<const N: usize>(
    subviews: &mut [(LayoutStage, LayoutFn, i8); N],
    offer: Size,
    spacing: u16,
) -> Size {
    let mut remaining_height = offer.height.saturating_sub(spacing * (N - 1) as u16);

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
        let mut group_offer = Size::new(offer.width, remaining_height / slice_len as u16);
        let mut remainder = remaining_height as usize % slice_len;

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
                // Adjust the offer height to account for the remainder. The initial views will be
                // offered an extra pixel. This is mostly important for rendering character pixels
                // where the pixels are large.
                let adjusted_offer = if i < remainder {
                    Size::new(group_offer.width, group_offer.height + 1)
                } else {
                    group_offer
                };

                let subview_size = subviews.get_mut(*subview_index).unwrap().1(adjusted_offer);
                if subview_size.height > adjusted_offer.height {
                    // The subview is unwilling to shrink, reslice the remaining width
                    subviews[*subview_index].0 = LayoutStage::Final(subview_size);
                    remaining_height = remaining_height.saturating_sub(subview_size.height);
                    slice_len -= 1;
                    // on the last subview, the length will go to zero
                    group_offer.height = remaining_height
                        .checked_div(slice_len as u16)
                        .unwrap_or(group_offer.height);
                    if slice_len != 0 {
                        remainder = i + remaining_height as usize % slice_len;
                    }
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
                remaining_height = remaining_height.saturating_sub(s.height);
            }
        }

        if remaining_height > 0 {
            // If there is any remaining height, offer it to each of the candidate views.
            // The first view is always offered the extra height first...hope this is right
            for subview_index in subviews_indecies.iter() {
                if let LayoutStage::Candidate(s) = subviews[*subview_index].0 {
                    let leftover = s + Size::new(0, remaining_height);
                    let subview_size = subviews.get_mut(*subview_index).unwrap().1(leftover);
                    remaining_height -= subview_size.height - s.height;
                    subviews[*subview_index].0 = LayoutStage::Final(subview_size);
                    // unnecessary?
                }
            }
        }
    }

    // At this point all the subviews should have either a final or a candidate size
    // Calculate the final VStack size
    let total_child_size = subviews.iter().fold(
        Size::new(0, offer.height - remaining_height),
        |acc, (size, _, _)| match size {
            LayoutStage::Final(s) | LayoutStage::Candidate(s) => {
                Size::new(max(acc.width, s.width), acc.height)
            }
            _ => unreachable!(),
        },
    );

    Size::new(
        min(offer.width, total_child_size.width),
        min(offer.height, total_child_size.height),
    )
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum LayoutStage {
    Unsized,
    Candidate(Size),
    Final(Size),
}
