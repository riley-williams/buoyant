use super::{AnimatedJoin, AnimationDomain, CharacterRender, CharacterRenderTarget};
use crate::primitives::Point;

macro_rules! impl_join_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<$($type: crate::render::AnimatedJoin),+> crate::render::AnimatedJoin for ($($type,)+) {
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
    impl_join_for_collections!((0, T0));
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

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::primitives::Point;
    use crate::render::AnimationDomain;
    use crate::render::EmbeddedGraphicsRender;
    use embedded_graphics_core::draw_target::DrawTarget;
    use embedded_graphics_core::prelude::PixelColor;

    macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color: embedded_graphics_core::pixelcolor::PixelColor, $($type: crate::render::EmbeddedGraphicsRender<Color> ),+> crate::render::EmbeddedGraphicsRender<Color> for ($($type,)+) {
            fn render(
                &self,
                target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Color>,
                style: &Color,
                offset: crate::primitives::Point
            ) {
                $(
                    self.$n.render(target, style, offset);
                )+
            }

            fn render_animated(
                render_target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Color>,
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
        impl_render_for_collections!((0, T0));
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

    impl<Color: PixelColor, T: EmbeddedGraphicsRender<Color>, const N: usize>
        EmbeddedGraphicsRender<Color> for heapless::Vec<T, N>
    {
        fn render(
            &self,
            render_target: &mut impl DrawTarget<Color = Color>,
            style: &Color,
            offset: Point,
        ) {
            self.iter()
                .for_each(|item| item.render(render_target, style, offset));
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = Color>,
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
}

macro_rules! impl_char_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color, $($type: crate::render::CharacterRender<Color> ),+> crate::render::CharacterRender<Color> for ($($type,)+) {
            fn render(
                &self,
                target: &mut impl crate::render::CharacterRenderTarget<Color = Color>,
                style: &Color,
                offset: crate::primitives::Point
            ) {
                $(
                    self.$n.render(target, style, offset);
                )+
            }

            fn render_animated(
                render_target: &mut impl crate::render::CharacterRenderTarget<Color = Color>,
                source: &Self,
                target: &Self,
                style: &Color,
                offset: crate::primitives::Point,
                config: &crate::render::AnimationDomain,
            ) {
                $(
                    $type::render_animated(
                        render_target,
                        &source.$n,
                        &target.$n,
                        style,
                        offset,
                        config,
                    );
                )+
            }
        }
    };
}

#[rustfmt::skip]
mod impl_char_render {
    impl_char_render_for_collections!((0, T0));
    impl_char_render_for_collections!((0, T0), (1, T1));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
    impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
}

impl<Color, T: CharacterRender<Color>, const N: usize> CharacterRender<Color>
    for heapless::Vec<T, N>
{
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = Color>,
        style: &Color,
        offset: Point,
    ) {
        self.iter()
            .for_each(|item| item.render(render_target, style, offset));
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        config: &super::AnimationDomain,
    ) {
        source
            .iter()
            .zip(target.iter())
            .for_each(|(source, target)| {
                T::render_animated(render_target, source, target, style, offset, config);
            });
    }
}
