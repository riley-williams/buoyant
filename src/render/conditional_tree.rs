use crate::{
    primitives::{Point, Size},
    render::{CharacterRender, CharacterRenderTarget},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConditionalTree<T, F> {
    pub subtree: Subtree<T, F>,
    pub size: Size,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subtree<T, F> {
    True(T),
    False(F),
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::primitives::Point;
    use crate::render::EmbeddedGraphicsRender;

    use super::{ConditionalTree, Subtree};
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics_core::draw_target::DrawTarget;

    impl<C: PixelColor, T: EmbeddedGraphicsRender<C>, F: EmbeddedGraphicsRender<C>>
        EmbeddedGraphicsRender<C> for ConditionalTree<T, F>
    {
        #[inline]
        fn render(&self, target: &mut impl DrawTarget<Color = C>, style: &C, offset: Point) {
            match &self.subtree {
                Subtree::True(true_tree) => true_tree.render(target, style, offset),
                Subtree::False(false_tree) => false_tree.render(target, style, offset),
            }
        }

        #[inline]
        fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
            match (source.subtree, target.subtree) {
                (Subtree::True(source_tree), Subtree::True(target_tree)) => Self {
                    subtree: Subtree::True(T::join(source_tree, target_tree, domain)),
                    size: target.size,
                },
                (Subtree::False(source_tree), Subtree::False(target_tree)) => Self {
                    subtree: Subtree::False(F::join(source_tree, target_tree, domain)),
                    size: target.size,
                },
                (_, target_tree) => Self {
                    subtree: target_tree,
                    size: target.size,
                },
            }
        }
    }
}

impl<C, T: CharacterRender<C>, F: CharacterRender<C>> CharacterRender<C> for ConditionalTree<T, F> {
    #[inline]
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, style: &C, offset: Point) {
        match &self.subtree {
            Subtree::True(true_tree) => true_tree.render(target, style, offset),
            Subtree::False(false_tree) => false_tree.render(target, style, offset),
        }
    }

    #[inline]
    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,

        config: &crate::render::AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), config);
        intermediate.render(render_target, style, offset);
    }

    #[inline]
    fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
        match (source.subtree, target.subtree) {
            (Subtree::True(s), Subtree::True(t)) => Self {
                subtree: Subtree::True(T::join(s, t, domain)),
                size: target.size,
            },
            (Subtree::False(s), Subtree::False(t)) => Self {
                subtree: Subtree::False(F::join(s, t, domain)),
                size: target.size,
            },
            (_, target_tree) => Self {
                subtree: target_tree,
                size: target.size,
            },
        }
    }
}
