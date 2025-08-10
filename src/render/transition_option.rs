use crate::{
    primitives::{Interpolate, Size},
    render::{AnimatedJoin, Render},
    render_target::RenderTarget,
    transition::{Direction, Transition},
};

use super::AnimationDomain;

/// An optional subtree that can be rendered with a transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionOption<Subtree, T> {
    Some {
        subtree: Subtree,
        /// Size of the subtree, used for computing offsets
        size: Size,
        /// The transition to apply
        transition: T,
    },
    None,
}

impl<Subtree, T> TransitionOption<Subtree, T> {
    /// Constructs a new [`Self::Some`] variant
    #[must_use]
    pub const fn new_some(subtree: Subtree, size: Size, transition: T) -> Self {
        Self::Some {
            subtree,
            size,
            transition,
        }
    }
}

impl<Subtree: AnimatedJoin + Clone, T: Transition> AnimatedJoin for TransitionOption<Subtree, T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        if let (
            Self::Some {
                subtree: source_subtree,
                ..
            },
            Self::Some {
                subtree: target_subtree,
                ..
            },
        ) = (source, self)
        {
            target_subtree.join_from(source_subtree, domain);
        }
    }
}

impl<Subtree: Render<Color> + Clone, T: Transition, Color: Interpolate + Copy> Render<Color>
    for TransitionOption<Subtree, T>
{
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        if let Self::Some { subtree, .. } = self {
            subtree.render(render_target, style);
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        match (source, target) {
            (
                Self::Some {
                    subtree: source, ..
                },
                Self::Some {
                    subtree: target, ..
                },
            ) => {
                Subtree::render_animated(render_target, source, target, style, domain);
            }
            (
                Self::Some {
                    subtree: source_subtree,
                    size,
                    transition,
                    ..
                },
                Self::None,
            ) => {
                if !domain.is_complete() {
                    let opacity = transition.opacity(Direction::Out, domain.factor);
                    let offset = transition.transform(Direction::Out, domain.factor, *size);
                    render_target.with_layer(
                        |l| l.offset(offset).opacity(opacity),
                        |render_target| {
                            source_subtree.render(render_target, style);
                        },
                    );
                }
            }
            (
                Self::None,
                Self::Some {
                    subtree: target_subtree,
                    size,
                    transition,
                    ..
                },
            ) => {
                if domain.is_complete() {
                    target_subtree.render(render_target, style);
                } else {
                    let alpha = transition.opacity(Direction::In, domain.factor);
                    let offset = transition.transform(Direction::In, domain.factor, *size);
                    render_target.with_layer(
                        |l| l.offset(offset).opacity(alpha),
                        |render_target| {
                            target_subtree.render(render_target, style);
                        },
                    );
                }
            }
            (Self::None, Self::None) => {}
        }
    }
}
