use crate::{primitives::Size, render::Render};

use embedded_graphics::{prelude::PixelColor, primitives::PrimitiveStyle};
use embedded_graphics_core::draw_target::DrawTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalTree<T, F> {
    pub subtree: Subtree<T, F>,
    pub size: Size,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subtree<T, F> {
    True(T),
    False(F),
}

impl<C: PixelColor, T: Render<C>, F: Render<C>> Render<C> for ConditionalTree<T, F> {
    fn render(&self, target: &mut impl DrawTarget<Color = C>, style: &PrimitiveStyle<C>) {
        match &self.subtree {
            Subtree::True(true_tree) => true_tree.render(target, style),
            Subtree::False(false_tree) => false_tree.render(target, style),
        }
    }

    fn join(source: Self, target: Self, config: &crate::render::AnimationDomain) -> Self {
        match (source.subtree, target.subtree) {
            (Subtree::True(source_tree), Subtree::True(target_tree)) => Self {
                subtree: Subtree::True(T::join(source_tree, target_tree, config)),
                size: target.size,
            },
            (Subtree::False(source_tree), Subtree::False(target_tree)) => Self {
                subtree: Subtree::False(F::join(source_tree, target_tree, config)),
                size: target.size,
            },
            (_, target_tree) => Self {
                subtree: target_tree,
                size: target.size,
            },
        }
    }
}
