use core::cmp::max;

use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::{AnimationConfiguration, CharacterRender},
    render_target::CharacterRenderTarget,
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
}

impl<Color: Copy, T: RenderEnvironment<Color = Color>> RenderEnvironment
    for HorizontalEnvironment<'_, T>
{
    type Color = Color;
    fn foreground_color(&self) -> Color {
        self.inner_environment.foreground_color()
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
    pub fn with_spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

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

impl<U: Layout, V: Layout> Layout for HStack<(U, V)> {
    type Sublayout = (ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>);

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 2;
        let env = HorizontalEnvironment::from(env);
        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;

        let mut f0 = |size: &ProposedDimensions| {
            let layout = self.items.0.layout(size, &env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: &ProposedDimensions| {
            let layout = self.items.1.layout(size, &env);
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
            origin: Point::zero(),
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) {
        let env = HorizontalEnvironment::from(env);
        let mut width: i16 = 0;

        if !self.items.0.is_empty() {
            let new_origin = origin
                + Point::new(
                    width,
                    self.alignment.align(
                        layout.resolved_size.height.into(),
                        layout.sublayouts.0.resolved_size.height.into(),
                    ),
                );

            self.items
                .0
                .place_subviews(&mut layout.sublayouts.0, new_origin, &env);
            width += (u16::from(layout.sublayouts.0.resolved_size.width) + self.spacing) as i16;
        }

        if !self.items.1.is_empty() {
            let new_origin = origin
                + Point::new(
                    width,
                    self.alignment.align(
                        layout.resolved_size.height.into(),
                        layout.sublayouts.1.resolved_size.height.into(),
                    ),
                );

            self.items
                .1
                .place_subviews(&mut layout.sublayouts.1, new_origin, &env);
        }
    }
}

impl<U: Layout, V: Layout, W: Layout> Layout for HStack<(U, V, W)> {
    type Sublayout = (
        ResolvedLayout<U::Sublayout>,
        ResolvedLayout<V::Sublayout>,
        ResolvedLayout<W::Sublayout>,
    );

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        const N: usize = 3;
        let mut c0: Option<ResolvedLayout<U::Sublayout>> = None;
        let mut c1: Option<ResolvedLayout<V::Sublayout>> = None;
        let mut c2: Option<ResolvedLayout<W::Sublayout>> = None;

        let env = HorizontalEnvironment::from(env);

        let mut f0 = |size: &ProposedDimensions| {
            let layout = self.items.0.layout(size, &env);
            let size = layout.resolved_size;
            c0 = Some(layout);
            size
        };
        let mut f1 = |size: &ProposedDimensions| {
            let layout = self.items.1.layout(size, &env);
            let size = layout.resolved_size;
            c1 = Some(layout);
            size
        };
        let mut f2 = |size: &ProposedDimensions| {
            let layout = self.items.2.layout(size, &env);
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
            origin: Point::zero(),
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) {
        let env = HorizontalEnvironment::from(env);
        let mut width = 0;

        if !self.items.0.is_empty() {
            let new_origin = origin
                + Point::new(
                    width,
                    self.alignment.align(
                        layout.resolved_size.height.into(),
                        layout.sublayouts.0.resolved_size.height.into(),
                    ),
                );

            self.items
                .0
                .place_subviews(&mut layout.sublayouts.0, new_origin, &env);
            width += (u16::from(layout.sublayouts.0.resolved_size.width) + self.spacing) as i16;
        }

        if !self.items.1.is_empty() {
            let new_origin = origin
                + Point::new(
                    width,
                    self.alignment.align(
                        layout.resolved_size.height.into(),
                        layout.sublayouts.1.resolved_size.height.into(),
                    ),
                );

            self.items
                .1
                .place_subviews(&mut layout.sublayouts.1, new_origin, &env);
            width += (u16::from(layout.sublayouts.1.resolved_size.width) + self.spacing) as i16;
        }

        if !self.items.2.is_empty() {
            let new_origin = origin
                + Point::new(
                    width,
                    self.alignment.align(
                        layout.resolved_size.height.into(),
                        layout.sublayouts.2.resolved_size.height.into(),
                    ),
                );

            self.items
                .2
                .place_subviews(&mut layout.sublayouts.2, new_origin, &env);
        }
    }
}

type LayoutFn<'a> = &'a mut dyn FnMut(&ProposedDimensions) -> Dimensions;

fn layout_n<const N: usize>(
    subviews: &mut [(LayoutFn, i8, bool); N],
    offer: &ProposedDimensions,
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
        let minimum_dimension = subviews[index].0(&min_proposal);
        // skip any further work for empty views
        if subviews[index].2 {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = subviews[index].0(&max_proposal);
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
                _ => {}
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
            let size = subviews[*index].0(&ProposedDimensions {
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

// -- Character Render

impl<Pixel: Copy, U, V> CharacterRender<Pixel> for HStack<(U, V)>
where
    U: CharacterRender<Pixel>,
    V: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = HorizontalEnvironment::from(env);

        self.items.0.render(target, &layout.sublayouts.0, &env);
        self.items.1.render(target, &layout.sublayouts.1, &env);
    }
}

impl<Pixel: Copy, U, V, W> CharacterRender<Pixel> for HStack<(U, V, W)>
where
    U: CharacterRender<Pixel>,
    V: CharacterRender<Pixel>,
    W: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = HorizontalEnvironment::from(env);

        self.items.0.render(target, &layout.sublayouts.0, &env);
        self.items.1.render(target, &layout.sublayouts.1, &env);
        self.items.2.render(target, &layout.sublayouts.2, &env);
    }
}

// -- Embedded Render

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U, V> crate::render::PixelRender<Pixel> for HStack<(U, V)>
where
    U: crate::render::PixelRender<Pixel>,
    V: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = HorizontalEnvironment::from(env);

        self.items.0.render(target, &layout.sublayouts.0, &env);
        self.items.1.render(target, &layout.sublayouts.1, &env);
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl RenderEnvironment<Color = Pixel>,
        target_env: &impl RenderEnvironment<Color = Pixel>,
        config: &AnimationConfiguration,
    ) {
        let source_env = &HorizontalEnvironment::from(source_env);
        let target_env = &HorizontalEnvironment::from(target_env);

        crate::render::PixelRender::render_animated(
            target,
            &source_view.items.0,
            &source_layout.sublayouts.0,
            &target_view.items.0,
            &target_layout.sublayouts.0,
            source_env,
            target_env,
            config,
        );

        crate::render::PixelRender::render_animated(
            target,
            &source_view.items.1,
            &source_layout.sublayouts.1,
            &target_view.items.1,
            &target_layout.sublayouts.1,
            source_env,
            target_env,
            config,
        );
    }
}

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U, V, W> crate::render::PixelRender<Pixel> for HStack<(U, V, W)>
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
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let env = HorizontalEnvironment::from(env);

        self.items.0.render(target, &layout.sublayouts.0, &env);
        self.items.1.render(target, &layout.sublayouts.1, &env);
        self.items.2.render(target, &layout.sublayouts.2, &env);
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl RenderEnvironment<Color = Pixel>,
        target_env: &impl RenderEnvironment<Color = Pixel>,
        config: &AnimationConfiguration,
    ) {
        let source_env = &HorizontalEnvironment::from(source_env);
        let target_env = &HorizontalEnvironment::from(target_env);
        crate::render::PixelRender::render_animated(
            target,
            &source_view.items.0,
            &source_layout.sublayouts.0,
            &target_view.items.0,
            &target_layout.sublayouts.0,
            source_env,
            target_env,
            config,
        );

        crate::render::PixelRender::render_animated(
            target,
            &source_view.items.1,
            &source_layout.sublayouts.1,
            &target_view.items.1,
            &target_layout.sublayouts.1,
            source_env,
            target_env,
            config,
        );

        crate::render::PixelRender::render_animated(
            target,
            &source_view.items.2,
            &source_layout.sublayouts.2,
            &target_view.items.2,
            &target_layout.sublayouts.2,
            source_env,
            target_env,
            config,
        );
    }
}
