use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

pub struct FlexFrame<T> {
    child: T,
    min_width: Option<u16>,
    ideal_width: Option<u16>,
    max_width: Option<Dimension>,
    min_height: Option<u16>,
    ideal_height: Option<u16>,
    max_height: Option<Dimension>,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T> FlexFrame<T> {
    pub fn new(child: T) -> Self {
        Self {
            child,
            min_width: None,
            ideal_width: None,
            max_width: None,
            min_height: None,
            ideal_height: None,
            max_height: None,
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }

    pub fn with_min_width(mut self, min_width: u16) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn with_ideal_width(mut self, ideal_width: u16) -> Self {
        self.ideal_width = Some(ideal_width);
        self
    }

    pub fn with_max_width(mut self, max_width: u16) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    pub fn with_infinite_max_width(mut self) -> Self {
        self.max_width = Some(Dimension::infinite());
        self
    }

    pub fn with_min_height(mut self, min_height: u16) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn with_ideal_height(mut self, ideal_height: u16) -> Self {
        self.ideal_height = Some(ideal_height);
        self
    }

    pub fn with_max_height(mut self, max_height: u16) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    pub fn with_infinite_max_height(mut self) -> Self {
        self.max_height = Some(Dimension::infinite());
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

fn clamp_optional<T: Ord + Copy>(mut value: T, min: Option<T>, max: Option<T>) -> T {
    value = value.min(max.unwrap_or(value));
    value.max(min.unwrap_or(value))
}

impl<V: Layout> Layout for FlexFrame<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let sublayout_width_offer = match offer.width {
            ProposedDimension::Exact(d) => ProposedDimension::Exact(clamp_optional(
                d,
                self.min_width,
                self.max_width.map(|d| d.into()),
            )),
            ProposedDimension::Compact => match self.ideal_width {
                Some(ideal_width) => ProposedDimension::Exact(
                    self.min_width.map_or(ideal_width, |w| w.max(ideal_width)),
                ),
                None => ProposedDimension::Compact,
            },
            ProposedDimension::Infinite => match self.max_width {
                Some(max_width) if max_width.is_infinite() => ProposedDimension::Infinite,
                Some(max_width) => ProposedDimension::Exact(max_width.into()),
                None => ProposedDimension::Infinite,
            },
        };

        let sublayout_height_offer = match offer.height {
            ProposedDimension::Exact(d) => ProposedDimension::Exact(clamp_optional(
                d,
                self.min_height,
                self.max_height.map(|d| d.into()),
            )),
            ProposedDimension::Compact => match self.ideal_height {
                Some(ideal_height) => ProposedDimension::Exact(
                    self.min_height
                        .map_or(ideal_height, |h| h.max(ideal_height)),
                ),
                None => ProposedDimension::Compact,
            },
            ProposedDimension::Infinite => match self.max_height {
                Some(max_height) if max_height.is_infinite() => ProposedDimension::Infinite,
                Some(max_height) => ProposedDimension::Exact(max_height.into()),
                None => ProposedDimension::Infinite,
            },
        };

        let sublayout_offer = ProposedDimensions {
            width: sublayout_width_offer,
            height: sublayout_height_offer,
        };

        let sublayout = self.child.layout(&sublayout_offer, env);

        // restrict self size to min/max regardless of what the sublayout returns
        let sublayout_width = sublayout.resolved_size.width;
        let sublayout_height = sublayout.resolved_size.height;

        let w = self
            .max_width
            .unwrap_or(sublayout_width)
            .min(greatest_possible(sublayout_width_offer, sublayout_width))
            .max(self.min_width.map_or(sublayout_width, |f| f.into()));

        let h = self
            .max_height
            .unwrap_or(sublayout_height)
            .min(greatest_possible(sublayout_height_offer, sublayout_height))
            .max(self.min_height.map_or(sublayout_height, |f| f.into()));

        let resolved_size = Dimensions {
            width: w,
            height: h,
        };

        ResolvedLayout {
            sublayouts: sublayout,
            resolved_size,
        }
    }
}

fn greatest_possible(proposal: ProposedDimension, ideal: Dimension) -> Dimension {
    match proposal {
        ProposedDimension::Exact(d) => d.into(),
        ProposedDimension::Compact => ideal,
        ProposedDimension::Infinite => Dimension::infinite(),
    }
}

impl<T: Renderable<C>, C> Renderable<C> for FlexFrame<T> {
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
