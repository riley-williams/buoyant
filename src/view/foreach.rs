use core::cmp::{max, min};

use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
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
}

impl<Color: Copy, T: RenderEnvironment<Color = Color>> RenderEnvironment
    for ForEachEnvironment<'_, T>
{
    type Color = Color;
    fn foreground_color(&self) -> Color {
        self.inner_environment.foreground_color()
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
}

impl<const N: usize, I: IntoIterator + Copy, V, F> ForEach<N, I, V, F>
where
    V: Layout,
    F: Fn(&I::Item) -> V,
{
    pub fn new(iter: I, build_view: F) -> Self {
        Self { iter, build_view }
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
    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        let env = &ForEachEnvironment::from(env);
        let mut sublayouts: heapless::Vec<ResolvedLayout<V::Sublayout>, N> = heapless::Vec::new();

        // TODO: consolidate array init to avoid accidentally allowing them to become different
        // lengths
        let mut items: heapless::Vec<I::Item, N> = heapless::Vec::new();
        _ = self.iter.into_iter().try_for_each(|item| items.push(item));
        // if let Err(_) = result {
        //     // TODO: log an error, iterator was too large
        // }

        let mut subview_stages: heapless::Vec<(LayoutStage, i8), N> = heapless::Vec::new();
        // fill sublayouts with an initial garbage layout
        for item in &items {
            let view = (self.build_view)(item);
            _ = sublayouts.push(view.layout(offer, env));
            _ = subview_stages.push((LayoutStage::Unsized, view.priority()));
        }

        let layout_fn = |index: usize, offer: Size| {
            let layout = (self.build_view)(&items[index]).layout(offer, env);
            let size = layout.resolved_size;
            sublayouts[index] = layout;
            size
        };

        let size = layout_n(&mut subview_stages, offer, 0, layout_fn);
        ResolvedLayout {
            sublayouts,
            resolved_size: size,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum LayoutStage {
    Unsized,
    Candidate(Size),
    Final(Size),
}

fn layout_n<const N: usize>(
    subviews: &mut heapless::Vec<(LayoutStage, i8), N>,
    offer: Size,
    spacing: u16,
    mut layout_fn: impl FnMut(usize, Size) -> Size,
) -> Size {
    let mut remaining_height = offer
        .height
        .saturating_sub(spacing * (subviews.len() - 1) as u16);

    loop {
        // collect the unsized subviews with the max layout priority into a group
        let mut subviews_indecies: heapless::Vec<usize, N> =
            heapless::Vec::from_slice(&[0; N]).expect("This can never fail, N vec from N slice");
        let mut max = i8::MIN;
        let mut slice_start: usize = 0;
        let mut slice_len: usize = 0;
        for (i, (size, priority)) in subviews.iter().enumerate() {
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

                let subview_size = layout_fn(*subview_index, adjusted_offer);
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
                    let subview_size = layout_fn(*subview_index, leftover);
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
        |acc, (size, _)| match size {
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

impl<const N: usize, Pixel: Copy, I: IntoIterator + Copy, V, F> CharacterRender<Pixel>
    for ForEach<N, I, V, F>
where
    V: CharacterRender<Pixel>,
    F: Fn(&I::Item) -> V,
{
    fn render(
        &self,
        target: &mut impl crate::render_target::CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &ForEachEnvironment::from(env);

        let mut height = 0;

        for (item_layout, item) in layout.sublayouts.iter().zip(self.iter.into_iter()) {
            // TODO: defaulting to center alignment
            let aligned_origin = origin
                + Point::new(
                    (layout.resolved_size.width as i16 - item_layout.resolved_size.width as i16)
                        / 2,
                    height,
                );
            let view = (self.build_view)(&item);
            view.render(target, item_layout, aligned_origin, env);

            height += item_layout.resolved_size.height as i16;
        }
    }
}

// -- Embedded Render

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<const N: usize, Pixel: Copy, I: IntoIterator + Copy, V, F> crate::render::PixelRender<Pixel>
    for ForEach<N, I, V, F>
where
    V: crate::render::PixelRender<Pixel>,
    F: Fn(&I::Item) -> V,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = &ForEachEnvironment::from(env);

        let mut height = 0;

        for (item_layout, item) in layout.sublayouts.iter().zip(self.iter.into_iter()) {
            // TODO: defaulting to center alignment
            let aligned_origin = origin
                + Point::new(
                    (layout.resolved_size.width as i16 - item_layout.resolved_size.width as i16)
                        / 2,
                    height,
                );
            let view = (self.build_view)(&item);
            view.render(target, item_layout, aligned_origin, env);

            height += item_layout.resolved_size.height as i16;
        }
    }
}
