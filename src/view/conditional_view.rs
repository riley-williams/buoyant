use embedded_graphics::prelude::PixelColor;

use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{
        primitives::{ConditionalTree, Subtree},
        Renderable,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalView<U, V> {
    pub condition: bool,
    pub true_view: U,
    pub false_view: V,
}

impl<U, V> ConditionalView<U, V> {
    pub fn new(condition: bool, true_view: U, false_view: V) -> Self {
        Self {
            condition,
            true_view,
            false_view,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conditional<U, V> {
    True(U),
    False(V),
}

impl<U: Layout, V: Layout> Layout for ConditionalView<U, V> {
    type Sublayout = Conditional<ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        if self.condition {
            let child_layout = self.true_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: Conditional::True(child_layout),
                resolved_size,
            }
        } else {
            let child_layout = self.false_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: Conditional::False(child_layout),
                resolved_size,
            }
        }
    }

    fn priority(&self) -> i8 {
        if self.condition {
            self.true_view.priority()
        } else {
            self.false_view.priority()
        }
    }

    fn is_empty(&self) -> bool {
        if self.condition {
            self.true_view.is_empty()
        } else {
            self.false_view.is_empty()
        }
    }
}

impl<U: Renderable<C>, V: Renderable<C>, C: PixelColor> Renderable<C> for ConditionalView<U, V> {
    type Renderables = ConditionalTree<U::Renderables, V::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        let subtree = match &layout.sublayouts {
            Conditional::True(ref true_layout) => {
                Subtree::True(self.true_view.render_tree(true_layout, origin, env))
            }
            Conditional::False(ref false_layout) => {
                Subtree::False(self.false_view.render_tree(false_layout, origin, env))
            }
        };

        ConditionalTree {
            subtree,
            size: layout.resolved_size.into(),
        }
    }
}
