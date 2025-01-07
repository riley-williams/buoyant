use embedded_graphics::prelude::PixelColor;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions, Size},
    render::Renderable,
};

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
pub struct Padding<T> {
    padding: u16,
    child: T,
}

impl<T> Padding<T> {
    pub fn new(padding: u16, child: T) -> Self {
        Self { padding, child }
    }
}

impl<T> PartialEq for Padding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.padding == other.padding
    }
}

impl<V: Layout> Layout for Padding<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let padded_offer = ProposedDimensions {
            width: offer.width - (2 * self.padding),
            height: offer.height - (2 * self.padding),
        };
        let child_layout = self.child.layout(&padded_offer, env);
        let padding_size =
            child_layout.resolved_size + Size::new(2 * self.padding, 2 * self.padding);
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size: padding_size,
        }
    }
}

impl<T: Renderable<C>, C: PixelColor> Renderable<C> for Padding<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        self.child.render_tree(
            &layout.sublayouts,
            origin + Point::new(self.padding as i16, self.padding as i16),
            env,
        )
    }
}
