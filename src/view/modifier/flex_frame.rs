use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};

pub struct FlexFrame<T> {
    child: T,
    min_width: Option<u16>,
    max_width: Option<u16>,
    min_height: Option<u16>,
    max_height: Option<u16>,
    horizontal_alignment: Option<HorizontalAlignment>,
    vertical_alignment: Option<VerticalAlignment>,
}

impl<T> FlexFrame<T> {
    pub fn new(
        child: T,
        min_width: Option<u16>,
        max_width: Option<u16>,
        min_height: Option<u16>,
        max_height: Option<u16>,
        horizontal_alignment: Option<HorizontalAlignment>,
        vertical_alignment: Option<VerticalAlignment>,
    ) -> Self {
        Self {
            child,
            min_width,
            max_width,
            min_height,
            max_height,
            horizontal_alignment,
            vertical_alignment,
        }
    }
}

impl<T> PartialEq for FlexFrame<T> {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width
            && self.max_width == other.max_width
            && self.min_height == other.min_height
            && self.max_height == other.max_height
            && self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<V: Layout> Layout for FlexFrame<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        let min_width = self.min_width.unwrap_or(0);
        let max_width = self.max_width.unwrap_or(offer.width);
        let min_height = self.min_height.unwrap_or(0);
        let max_height = self.max_height.unwrap_or(offer.height);

        let modified_offer = Size::new(
            offer.width.min(max_width).max(min_width),
            offer.height.min(max_height).max(min_height),
        );
        let child_layout = self.child.layout(modified_offer, env);

        let width = self
            .max_width
            .unwrap_or(child_layout.resolved_size.width)
            .min(offer.width)
            .max(self.min_width.unwrap_or(child_layout.resolved_size.width));
        let height = self
            .max_height
            .unwrap_or(child_layout.resolved_size.height)
            .min(offer.height)
            .max(self.min_height.unwrap_or(child_layout.resolved_size.height));

        let resolved_size = Size::new(width, height);
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size,
        }
    }
}

impl<Pixel, View: Layout> Render<Pixel> for FlexFrame<View>
where
    View: Render<Pixel>,
    Pixel: PixelColor,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
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
