use crate::render::{Render, RenderTarget};

use super::AnimatedJoin;

macro_rules! define_branch {
    ($name:ident, $($variant:ident),+) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name<$($variant),+> {
            $(
                $variant($variant),
            )+
        }
        impl <$($variant),+> Default for $name<$($variant),+>
        where
            V0: Default,
        {
            fn default() -> Self {
                $name::V0(V0::default())
            }
        }

        #[allow(unreachable_patterns)]
        impl<$($variant),+>  AnimatedJoin for $name<$($variant),+>
            where $($variant: AnimatedJoin,)+
        {
            fn join_from(&mut self, source: &Self, domain: &crate::render::AnimationDomain) {
                match (source, self) {
                    $(
                        (Self::$variant(source), Self::$variant(target)) => {
                            target.join_from(source, domain)
                        },
                    )+
                    (_, _) => (),
                }
            }
        }

        #[allow(unreachable_patterns)] // OneOf1
        impl<C, $($variant),+> Render<C> for $name<$($variant),+>
            where $($variant: Render<C>,)+
        {
            fn render(&self, target: &mut impl RenderTarget<ColorFormat = C>, color: &C) {
                match self {
                    $(
                        Self::$variant(v) => v.render(target, color),
                    )+
                }
            }

            fn render_animated(
                render_target: &mut impl RenderTarget<ColorFormat = C>,
                source: &Self,
                target: &Self,
                style: &C,
                domain: &crate::render::AnimationDomain,
            ) {
                match (source, target) {
                    $(
                        (Self::$variant(source), Self::$variant(target)) => {
                            $variant::render_animated(render_target, source, target, style, domain);
                        },
                    )+
                    (_, target) => {
                        target.render(render_target, style);
                    }
                }
            }
        }
    }
}

define_branch!(OneOf1, V0);
define_branch!(OneOf2, V0, V1);
define_branch!(OneOf3, V0, V1, V2);
define_branch!(OneOf4, V0, V1, V2, V3);
define_branch!(OneOf5, V0, V1, V2, V3, V4);
define_branch!(OneOf6, V0, V1, V2, V3, V4, V5);
define_branch!(OneOf7, V0, V1, V2, V3, V4, V5, V6);
