use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

pub struct FixedFrame<T> {
    width: Option<u16>,
    height: Option<u16>,
    horizontal_alignment: Option<HorizontalAlignment>,
    vertical_alignment: Option<VerticalAlignment>,
    child: T,
}

impl<T> FixedFrame<T> {
    pub fn new(child: T) -> Self {
        Self {
            width: None,
            height: None,
            horizontal_alignment: None,
            vertical_alignment: None,
            child,
        }
    }

    pub fn with_width(self, width: u16) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn with_height(self, height: u16) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }

    pub fn with_horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: Some(alignment),
            ..self
        }
    }

    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: Some(alignment),
            ..self
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
                self.horizontal_alignment.unwrap_or_default().align(
                    layout.resolved_size.width.into(),
                    layout.sublayouts.resolved_size.width.into(),
                ),
                self.vertical_alignment.unwrap_or_default().align(
                    layout.resolved_size.height.into(),
                    layout.sublayouts.resolved_size.height.into(),
                ),
            );

        self.child.render_tree(&layout.sublayouts, new_origin, env)
    }
}
