use crate::{
    environment::LayoutEnvironment,
    layout::{Alignment, Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

/// A view that uses the layout of the modified view, rendering the overlay
/// on top of it.
#[derive(Debug, Clone)]
pub struct OverlayView<T, U> {
    foreground: T,
    overlay: U,
    alignment: Alignment,
}

impl<T, U> OverlayView<T, U> {
    pub const fn new(foreground: T, overlay: U, alignment: Alignment) -> Self {
        Self {
            foreground,
            overlay,
            alignment,
        }
    }
}

impl<T: Layout, U: Layout> Layout for OverlayView<T, U> {
    type Sublayout = (ResolvedLayout<T::Sublayout>, ResolvedLayout<U::Sublayout>);

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let foreground_layout = self.foreground.layout(offer, env);
        let foreground_size = foreground_layout.resolved_size;
        // Propose the foreground size to the overlay
        // This would benefit from splitting layout into separate functions for the various offers
        let overlay_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let overlay_layout = self.overlay.layout(&overlay_offer, env);

        ResolvedLayout {
            sublayouts: (foreground_layout, overlay_layout),
            resolved_size: foreground_size,
        }
    }

    fn priority(&self) -> i8 {
        self.foreground.priority()
    }

    fn is_empty(&self) -> bool {
        self.foreground.is_empty()
    }
}

impl<T: Renderable, U: Renderable> Renderable for OverlayView<T, U> {
    // Tuples are rendered first to last
    type Renderables = (T::Renderables, U::Renderables);

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let new_origin = origin
            + Point::new(
                self.alignment.horizontal().align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.1.resolved_size.width.into(),
                ),
                self.alignment.vertical().align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.1.resolved_size.height.into(),
                ),
            );

        (
            self.foreground
                .render_tree(&layout.sublayouts.0, origin, env),
            self.overlay
                .render_tree(&layout.sublayouts.1, new_origin, env),
        )
    }
}
