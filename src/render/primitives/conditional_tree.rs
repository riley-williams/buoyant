use crate::{
    primitives::Size,
    render::{shade::Shader, Render},
    render_target::RenderTarget,
};

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

impl<C, T: Render<C>, F: Render<C>> Render<C> for ConditionalTree<T, F> {
    fn render(&self, target: &mut impl RenderTarget<Color = C>, shader: &impl Shader<Color = C>) {
        match &self.subtree {
            Subtree::True(true_tree) => true_tree.render(target, shader),
            Subtree::False(false_tree) => false_tree.render(target, shader),
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
