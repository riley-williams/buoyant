use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions, Size},
    render::Renderable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edges {
    All,
    Horizontal,
    Vertical,
    Top,
    Bottom,
    Leading,
    Trailing,
}

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
#[derive(Debug, Clone)]
pub struct Padding<T> {
    edges: Edges,
    padding: u32,
    inner: T,
}

impl<T> Padding<T> {
    pub const fn new(edges: Edges, padding: u32, inner: T) -> Self {
        Self {
            edges,
            padding,
            inner,
        }
    }
}

impl<T> PartialEq for Padding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.padding == other.padding && self.edges == other.edges
    }
}

impl<V: Layout> Layout for Padding<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (leading, trailing, top, bottom) = match self.edges {
            Edges::All => (self.padding, self.padding, self.padding, self.padding),
            Edges::Horizontal => (self.padding, self.padding, 0, 0),
            Edges::Vertical => (0, 0, self.padding, self.padding),
            Edges::Top => (0, 0, self.padding, 0),
            Edges::Bottom => (0, 0, 0, self.padding),
            Edges::Leading => (self.padding, 0, 0, 0),
            Edges::Trailing => (0, self.padding, 0, 0),
        };
        let extra_width = leading + trailing;
        let extra_height = top + bottom;
        let padded_offer = ProposedDimensions {
            width: offer.width - extra_width,
            height: offer.height - extra_height,
        };
        let child_layout = self.inner.layout(&padded_offer, env);
        let padding_size = child_layout.resolved_size + Size::new(extra_width, extra_height);
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size: padding_size,
        }
    }

    fn priority(&self) -> i8 {
        self.inner.priority()
    }
}

impl<T: Renderable> Renderable for Padding<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let (leading, top) = match self.edges {
            Edges::All => (self.padding, self.padding),
            Edges::Horizontal | Edges::Leading => (self.padding, 0),
            Edges::Vertical | Edges::Top => (0, self.padding),
            Edges::Bottom | Edges::Trailing => (0, 0),
        };
        self.inner.render_tree(
            &layout.sublayouts,
            origin + Point::new(leading as i32, top as i32),
            env,
        )
    }
}
