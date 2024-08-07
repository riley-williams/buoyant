use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Point, Size},
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

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        let modified_offer = Size::new(
            self.width.unwrap_or(offer.width),
            self.height.unwrap_or(offer.height),
        );
        let child_layout = self.child.layout(modified_offer, env);
        let resolved_size = Size::new(
            self.width.unwrap_or(child_layout.resolved_size.width),
            self.height.unwrap_or(child_layout.resolved_size.height),
        );
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size,
        }
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
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.unwrap_or_default().align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.resolved_size.width as i16,
                ),
                self.vertical_alignment.unwrap_or_default().align(
                    layout.resolved_size.height as i16,
                    layout.sublayouts.resolved_size.height as i16,
                ),
            );

        self.child
            .render(target, &layout.sublayouts, new_origin, env);
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
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.unwrap_or_default().align(
                    layout.resolved_size.width as i16,
                    layout.sublayouts.resolved_size.width as i16,
                ),
                self.vertical_alignment.unwrap_or_default().align(
                    layout.resolved_size.height as i16,
                    layout.sublayouts.resolved_size.height as i16,
                ),
            );

        self.child
            .render(target, &layout.sublayouts, new_origin, env);
    }
}
