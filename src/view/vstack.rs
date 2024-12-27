use core::cmp::max;

use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, LayoutDirection, ProposedDimensions, ResolvedLayout},
    primitives::{Dimension, Dimensions, Point, ProposedDimension},
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

    pub fn with_spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    pub fn with_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<U: Layout, V: Layout> Layout for VStack<(U, V)> {
    type Sublayout = (ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>);

    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 2;
        let env = &VerticalEnvironment::from(env);
        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;

        let mut f0 = |size: ProposedDimensions| {
            let layout = self.items.0.layout(size, env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: ProposedDimensions| {
            let layout = self.items.1.layout(size, env);
            let size = layout.resolved_size;
            c1 = Some(layout);
            size
        };

        // precalculate priority to avoid multiple dynamic dispatch calls
        let mut subviews: [(LayoutFn, i8, bool); N] = [
            (&mut f0, self.items.0.priority(), self.items.0.is_empty()),
            (&mut f1, self.items.1.priority(), self.items.1.is_empty()),
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

    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 3;
        let env = &VerticalEnvironment::from(env);

        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;
        let mut c2: Option<ResolvedLayout<W::Sublayout>> = None;

        let mut f0 = |size: ProposedDimensions| {
            let layout = self.items.0.layout(size, env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: ProposedDimensions| {
            let layout = self.items.1.layout(size, env);
            let size = layout.resolved_size;
            c1 = Some(layout);
            size
        };
        let mut f2 = |size: ProposedDimensions| {
            let layout = self.items.2.layout(size, env);
            let size = layout.resolved_size;
            c2 = Some(layout);
            size
        };

        // precalculate priority to avoid multiple dynamic dispatch calls
        let mut subviews: [(LayoutFn, i8, bool); N] = [
            (&mut f0, self.items.0.priority(), self.items.0.is_empty()),
            (&mut f1, self.items.1.priority(), self.items.1.is_empty()),
            (&mut f2, self.items.2.priority(), self.items.2.is_empty()),
        ];
        let total_size = layout_n(&mut subviews, offer, self.spacing);
        ResolvedLayout {
            sublayouts: (c0.unwrap(), c1.unwrap(), c2.unwrap()),
            resolved_size: total_size,
        }
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
    for index in 0..N {
        let min_proposal = ProposedDimensions {
            width: offer.width,
            height: ProposedDimension::Exact(0),
        };
        let minimum_dimension = subviews[index].0(min_proposal);
        // skip any further work for empty views
        if subviews[index].2 {
            num_empty_views += 1;
            continue;
        }

        let max_proposal = ProposedDimensions {
            width: offer.width,
            height: ProposedDimension::Infinite,
        };
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

        let mut height: i16 = 0;

        if !self.items.0.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.0.resolved_size.width.into(),
                    ),
                    height,
                );

            self.items
                .0
                .render(target, &layout.sublayouts.0, new_origin, env);

            height += (u16::from(layout.sublayouts.0.resolved_size.height) + self.spacing) as i16;
        }

        if !self.items.1.is_empty() {
            let new_origin = Point::new(
                origin.x
                    + self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.1.resolved_size.width.into(),
                    ),
                height,
            );

            self.items
                .1
                .render(target, &layout.sublayouts.1, new_origin, env);
        }
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

        if !self.items.0.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.0.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .0
                .render(target, &layout.sublayouts.0, new_origin, env);

            let child_height: u16 = layout.sublayouts.0.resolved_size.height.into();
            height += (child_height + self.spacing) as i16;
        }

        if !self.items.1.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.1.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .1
                .render(target, &layout.sublayouts.1, new_origin, env);

            let child_height: u16 = layout.sublayouts.1.resolved_size.height.into();
            height += (child_height + self.spacing) as i16;
        }

        if !self.items.2.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.2.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .2
                .render(target, &layout.sublayouts.2, new_origin, env);
        }
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

        let mut height: i16 = 0;

        let new_origin = origin
            + Point::new(
                self.alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.0.resolved_size.width.into(),
                ),
                height,
            );

        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        let child_height: u16 = layout.sublayouts.0.resolved_size.height.into();
        height += (child_height + self.spacing) as i16;
        let new_origin = Point::new(
            origin.x
                + self.alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.1.resolved_size.width.into(),
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

        if !self.items.0.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.0.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .0
                .render(target, &layout.sublayouts.0, new_origin, env);

            let child_height: u16 = layout.sublayouts.0.resolved_size.height.into();
            height += (child_height + self.spacing) as i16;
        }

        if !self.items.1.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.1.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .1
                .render(target, &layout.sublayouts.1, new_origin, env);

            let child_height: u16 = layout.sublayouts.1.resolved_size.height.into();
            height += (child_height + self.spacing) as i16;
        }

        if !self.items.2.is_empty() {
            let new_origin = origin
                + Point::new(
                    self.alignment.align(
                        layout.resolved_size.width.into(),
                        layout.sublayouts.2.resolved_size.width.into(),
                    ),
                    height,
                );
            self.items
                .2
                .render(target, &layout.sublayouts.2, new_origin, env);
        }
    }
}
