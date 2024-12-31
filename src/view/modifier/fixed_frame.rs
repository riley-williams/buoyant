use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

pub struct FixedFrame<T> {
    width: Option<u16>,
    height: Option<u16>,
    horizontal_alignment: Option<HorizontalAlignment>,
    vertical_alignment: Option<VerticalAlignment>,
    child: T,
}

impl<T> FixedFrame<T> {
    pub fn new(
        child: T,
        width: Option<u16>,
        height: Option<u16>,
        horizontal_alignment: Option<HorizontalAlignment>,
        vertical_alignment: Option<VerticalAlignment>,
    ) -> Self {
        Self {
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            child,
        }
    }
}

impl<T> PartialEq for FixedFrame<T> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<V: Layout> Layout for FixedFrame<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let modified_offer = ProposedDimensions {
            width: self
                .width
                .map(ProposedDimension::Exact)
                .unwrap_or(offer.width),
            height: self
                .height
                .map(ProposedDimension::Exact)
                .unwrap_or(offer.height),
        };
        let child_layout = self.child.layout(&modified_offer, env);
        let resolved_size = Dimensions {
            width: self
                .width
                .map(Dimension::from)
                .unwrap_or(child_layout.resolved_size.width),
            height: self
                .height
                .map(Dimension::from)
                .unwrap_or(child_layout.resolved_size.height),
        };
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size,
            origin: Point::zero(),
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) {
        layout.origin = origin;
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.unwrap_or_default().align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.resolved_size.width.into(),
                ),
                self.vertical_alignment.unwrap_or_default().align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.resolved_size.height.into(),
                ),
            );

        self.child
            .place_subviews(&mut layout.sublayouts, new_origin, env);
    }
}

impl<Pixel: Copy, View: Layout> CharacterRender<Pixel> for FixedFrame<View>
where
    View: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<ResolvedLayout<View::Sublayout>>,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, &layout.sublayouts, env);
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, View: Layout> crate::render::PixelRender<Pixel> for FixedFrame<View>
where
    View: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<ResolvedLayout<View::Sublayout>>,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, &layout.sublayouts, env);
    }
}
