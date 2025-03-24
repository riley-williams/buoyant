use super::{AnimatedJoin, AnimationDomain, Render, RenderTarget};
use crate::primitives::Point;

macro_rules! impl_join_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<$($type: crate::render::AnimatedJoin),+> crate::render::AnimatedJoin for ($($type),+) {
            fn join(
                source: Self,
                target: Self,
                domain: &crate::render::AnimationDomain
            ) -> Self {
                (
                    $(
                        $type::join(
                            source.$n,
                            target.$n,
                            domain
                        ),
                    )+
                )
            }
        }
    };
}

#[rustfmt::skip]
mod impl_join {
    impl_join_for_collections!((0, T0), (1, T1));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
}

impl<T: AnimatedJoin, const N: usize> AnimatedJoin for heapless::Vec<T, N> {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        source
            .into_iter()
            .zip(target)
            .map(|(source, target)| T::join(source, target, domain))
            .collect()
    }
}
macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color, $($type: crate::render::Render<Color> ),+> crate::render::Render<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl crate::render_target::RenderTarget<ColorFormat = Color>,
                style: &Color,
                offset: crate::primitives::Point
            ) {
                $(
                    self.$n.render(target, style, offset);
                )+
            }

            fn render_animated(
                render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = Color>,
                source: &Self,
                target: &Self,
                style: &Color,
                offset: crate::primitives::Point,
                domain: &crate::render::AnimationDomain,
            ) {
                $(
                    $type::render_animated(
                        render_target,
                        &source.$n,
                        &target.$n,
                        style,
                        offset,
                        domain,
                    );
                )+
            }
        }
    };
}

#[rustfmt::skip]
    mod impl_render {
        impl_render_for_collections!((0, T0), (1, T1));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
        impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
    }

impl<Color, T: Render<Color>, const N: usize> Render<Color> for heapless::Vec<T, N> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        style: &Color,
        offset: Point,
    ) {
        self.iter()
            .for_each(|item| item.render(render_target, style, offset));
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        source
            .iter()
            .zip(target.iter())
            .for_each(|(source, target)| {
                T::render_animated(render_target, source, target, style, offset, domain);
            });
    }
}
