use crate::{
    environment::LayoutEnvironment,
    layout::{Alignment, HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Clone)]
pub struct FixedFrame<T> {
    width: Option<u16>,
    height: Option<u16>,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    child: T,
}

impl<T> FixedFrame<T> {
    pub const fn new(child: T) -> Self {
        Self {
            width: None,
            height: None,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
            child,
        }
    }

    /// Sets the width of the frame.
    pub const fn with_width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the frame.
    pub const fn with_height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the size of the frame.
    pub const fn with_size(mut self, width: u16, height: u16) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// The horizontal alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// The vertical alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// The alignment to apply when the child view is larger or smaller than the frame.
    pub const fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment.horizontal();
        self.vertical_alignment = alignment.vertical();
        self
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
            width: self.width.map_or(offer.width, ProposedDimension::Exact),
            height: self.height.map_or(offer.height, ProposedDimension::Exact),
        };
        let child_layout = self.child.layout(&modified_offer, env);
        let resolved_size = Dimensions {
            width: self
                .width
                .map_or(child_layout.resolved_size.width, Dimension::from),
            height: self
                .height
                .map_or(child_layout.resolved_size.height, Dimension::from),
        };
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size,
        }
    }
}

impl<T: Renderable<C>, C> Renderable<C> for FixedFrame<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let new_origin = origin
            + Point::new(
                self.horizontal_alignment.align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.resolved_size.width.into(),
                ),
                self.vertical_alignment.align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.resolved_size.height.into(),
                ),
            );

        self.child.render_tree(&layout.sublayouts, new_origin, env)
    }
}
