use core::cmp::{max, min};

use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, LayoutDirection, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
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
}

impl<Color: Copy, T: RenderEnvironment<Color = Color>> RenderEnvironment
    for VerticalEnvironment<'_, T>
{
    type Color = Color;
    fn foreground_color(&self) -> Color {
        self.inner_environment.foreground_color()
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

    pub fn spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    pub fn alignment(self, alignment: HorizontalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<U: Layout, V: Layout> Layout for VStack<(U, V)> {
    type Sublayout = (ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>);

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 2;
        let env = &VerticalEnvironment::from(env);
        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;

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

impl<U: Layout, V: Layout, W: Layout> Layout for VStack<(U, V, W)> {
    type Sublayout = (
        ResolvedLayout<U::Sublayout>,
        ResolvedLayout<V::Sublayout>,
        ResolvedLayout<W::Sublayout>,
    );

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 3;
        let env = &VerticalEnvironment::from(env);

        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;
        let mut c2: Option<ResolvedLayout<W::Sublayout>> = None;

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

impl<Pixel: Copy, U: CharacterRender<Pixel>, V: CharacterRender<Pixel>> CharacterRender<Pixel>
    for VStack<(U, V)>
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &VerticalEnvironment::from(env);

        let mut height = 0;

        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.0.resolved_size.width as i16,
                ),
                height,
            );

        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        let new_origin = Point::new(
            origin.x
                + self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.1.resolved_size.width as i16,
                ),
            height,
        );

        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);
    }
}

impl<Pixel: Copy, U, V, W> CharacterRender<Pixel> for VStack<(U, V, W)>
where
    U: CharacterRender<Pixel>,
    V: CharacterRender<Pixel>,
    W: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &VerticalEnvironment::from(env);

        let mut height = 0;

        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.0.resolved_size.width as i16,
                ),
                height,
            );
        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.1.resolved_size.width as i16,
                ),
                height,
            );

        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);

        height += (layout.sublayouts.1.resolved_size.height + self.spacing) as i16;
        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.2.resolved_size.width as i16,
                ),
                height,
            );
        self.items
            .2
            .render(target, &layout.sublayouts.2, new_origin, env);
    }
}

// -- Embedded Render

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U: crate::render::PixelRender<Pixel>, V: crate::render::PixelRender<Pixel>>
    crate::render::PixelRender<Pixel> for VStack<(U, V)>
where
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &VerticalEnvironment::from(env);

        let mut height = 0;

        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.0.resolved_size.width as i16,
                ),
                height,
            );

        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        let new_origin = Point::new(
            origin.x
                + self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.1.resolved_size.width as i16,
                ),
            height,
        );

        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);
    }
}

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U, V, W> crate::render::PixelRender<Pixel> for VStack<(U, V, W)>
where
    U: crate::render::PixelRender<Pixel>,
    V: crate::render::PixelRender<Pixel>,
    W: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &VerticalEnvironment::from(env);

        let mut height = 0;

        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.0.resolved_size.width as i16,
                ),
                height,
            );
        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        height += (layout.sublayouts.0.resolved_size.height + self.spacing) as i16;
        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.1.resolved_size.width as i16,
                ),
                height,
            );

        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);

        height += (layout.sublayouts.1.resolved_size.height + self.spacing) as i16;
        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.2.resolved_size.width as i16,
                ),
                height,
            );
        self.items
            .2
            .render(target, &layout.sublayouts.2, new_origin, env);
    }
}
