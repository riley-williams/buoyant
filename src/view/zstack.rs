use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimensions},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

pub struct ZStack<T> {
    items: T,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T> PartialEq for ZStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<T> ZStack<T> {
    pub fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: alignment,
            ..self
        }
    }

    pub fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: alignment,
            ..self
        }
    }
}

impl<U, V> ZStack<(U, V)> {
    pub fn two(item0: U, item1: V) -> Self {
        ZStack {
            items: (item0, item1),
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }
}

impl<U: Layout, V: Layout> Layout for ZStack<(U, V)> {
    type Sublayout = (ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>);

    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let layout0 = self.items.0.layout(offer, env);
        let layout1 = self.items.1.layout(offer, env);
        let size = layout0.resolved_size.union(layout1.resolved_size);

        ResolvedLayout {
            sublayouts: (layout0, layout1),
            resolved_size: size.intersecting_proposal(offer),
        }
    }
}

impl<Pixel: Copy, U: Layout, V: Layout> CharacterRender<Pixel> for ZStack<(U, V)>
where
    U: CharacterRender<Pixel>,
    V: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<(ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>)>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.0.resolved_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.0.resolved_size.height.into(),
                ),
            );

        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.1.resolved_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.1.resolved_size.height.into(),
                ),
            );
        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U: Layout, V: Layout> crate::render::PixelRender<Pixel> for ZStack<(U, V)>
where
    U: crate::render::PixelRender<Pixel>,
    V: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<(ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>)>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.0.resolved_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.0.resolved_size.height.into(),
                ),
            );

        self.items
            .0
            .render(target, &layout.sublayouts.0, new_origin, env);

        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.1.resolved_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.1.resolved_size.height.into(),
                ),
            );
        self.items
            .1
            .render(target, &layout.sublayouts.1, new_origin, env);
    }
}
