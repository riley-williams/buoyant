use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{ProposedDimension, ProposedDimensions},
    render::Renderable,
};

/// A view that uses the layout of the foreground view, but renders the background
/// behind it.
#[derive(Debug, Clone)]
pub struct BackgroundView<T, U> {
    foreground: T,
    background: U,
}

impl<T, U> BackgroundView<T, U> {
    pub const fn new(foreground: T, background: U) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

impl<T: Layout, U: Layout> Layout for BackgroundView<T, U> {
    type Sublayout = (ResolvedLayout<T::Sublayout>, ResolvedLayout<U::Sublayout>);

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let foreground_layout = self.foreground.layout(offer, env);
        let foreground_size = foreground_layout.resolved_size;
        // Propose the foreground size to the background
        // This would benefit from splitting layout into separate functions for the various offers
        let background_offer = ProposedDimensions {
            width: ProposedDimension::Exact(foreground_size.width.into()),
            height: ProposedDimension::Exact(foreground_size.height.into()),
        };
        let background_layout = self.background.layout(&background_offer, env);

        ResolvedLayout {
            sublayouts: (foreground_layout, background_layout),
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

impl<T: Renderable, U: Renderable> Renderable for BackgroundView<T, U> {
    // Tuples are rendered first to last
    type Renderables = (U::Renderables, T::Renderables);

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        (
            self.background
                .render_tree(&layout.sublayouts.1, origin, env),
            self.foreground
                .render_tree(&layout.sublayouts.0, origin, env),
        )
    }
}
