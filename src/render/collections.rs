use super::{CharacterRender, CharacterRenderTarget};
use crate::primitives::Point;

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_impl {
    use crate::primitives::Point;
    use crate::render::AnimationDomain;
    use crate::render::EmbeddedGraphicsRender;
    use embedded_graphics_core::draw_target::DrawTarget;
    use embedded_graphics_core::prelude::PixelColor;

    macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color: PixelColor, $($type: EmbeddedGraphicsRender<Color> ),+> EmbeddedGraphicsRender<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl DrawTarget<Color = Color>,
                style: &Color,
                offset: Point
            ) {
                $(
                    self.$n.render(target, style, offset);
                )+
            }

            fn render_animated(
                render_target: &mut impl DrawTarget<Color = Color>,
                source: &Self,
                target: &Self,
                style: &Color,
                offset: Point,
                domain: &AnimationDomain,
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

            fn join(
                source: Self,
                target: Self,
                domain: &AnimationDomain
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

    // 10 seems like a reasonable number...
    impl_render_for_collections!((0, T0), (1, T1));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
    impl_render_for_collections!(
        (0, T0),
        (1, T1),
        (2, T2),
        (3, T3),
        (4, T4),
        (5, T5),
        (6, T6)
    );
    impl_render_for_collections!(
        (0, T0),
        (1, T1),
        (2, T2),
        (3, T3),
        (4, T4),
        (5, T5),
        (6, T6),
        (7, T7)
    );
    impl_render_for_collections!(
        (0, T0),
        (1, T1),
        (2, T2),
        (3, T3),
        (4, T4),
        (5, T5),
        (6, T6),
        (7, T7),
        (8, T8)
    );
    impl_render_for_collections!(
        (0, T0),
        (1, T1),
        (2, T2),
        (3, T3),
        (4, T4),
        (5, T5),
        (6, T6),
        (7, T7),
        (8, T8),
        (9, T9)
    );

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

        fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
            source
                .into_iter()
                .zip(target)
                .map(|(source, target)| T::join(source, target, domain))
                .collect()
        }
    }
}
// Character

macro_rules! impl_char_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color, $($type: CharacterRender<Color> ),+> CharacterRender<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl CharacterRenderTarget<Color = Color>,
                style: &Color,
                offset: Point
            ) {
                $(
                    self.$n.render(target, style, offset);
                )+
            }

            fn render_animated(
                render_target: &mut impl CharacterRenderTarget<Color = Color>,
                source: &Self,
                target: &Self,
                style: &Color,
                offset: Point,
                config: &super::AnimationDomain,
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

            fn join(
                source: Self,
                target: Self,
                domain: &super::AnimationDomain
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

// 10 seems like a reasonable number...
impl_char_render_for_collections!((0, T0), (1, T1));
impl_char_render_for_collections!((0, T0), (1, T1), (2, T2));
impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_char_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_char_render_for_collections!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_char_render_for_collections!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_char_render_for_collections!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8)
);
impl_char_render_for_collections!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8),
    (9, T9)
);

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

    fn join(source: Self, target: Self, domain: &super::AnimationDomain) -> Self {
        source
            .into_iter()
            .zip(target)
            .map(|(source, target)| T::join(source, target, domain))
            .collect()
    }
}
