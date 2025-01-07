use super::Render;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::prelude::PixelColor;

macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color: PixelColor, $($type: Render<Color> ),+> Render<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl DrawTarget<Color = Color>,
                style: &PrimitiveStyle<Color>,
            ) {
                $(
                    self.$n.render(target, style);
                )+
            }

            fn render_animated(
                render_target: &mut impl DrawTarget<Color = Color>,
                source: &Self,
                source_style: &PrimitiveStyle<Color>,
                target: &Self,
                target_style: &PrimitiveStyle<Color>,
                config: &super::AnimationDomain,
            ) {
                $(
                    $type::render_animated(
                        render_target,
                        &source.$n,
                        source_style,
                        &target.$n,
                        target_style,
                        config,
                    );
                )+
            }

            fn join(source: Self, target: Self, config: &super::AnimationDomain) -> Self {
                (
                    $(
                        $type::join(source.$n, target.$n, config),
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

impl<Color: PixelColor, T: Render<Color>, const N: usize> Render<Color> for heapless::Vec<T, N> {
    fn render(
        &self,
        render_target: &mut impl DrawTarget<Color = Color>,
        style: &PrimitiveStyle<Color>,
    ) {
        self.iter()
            .for_each(|item| item.render(render_target, style));
    }

    fn render_animated(
        render_target: &mut impl DrawTarget<Color = Color>,
        source: &Self,
        source_style: &PrimitiveStyle<Color>,
        target: &Self,
        target_style: &PrimitiveStyle<Color>,
        config: &super::AnimationDomain,
    ) {
        source
            .iter()
            .zip(target.iter())
            .for_each(|(source, target)| {
                T::render_animated(
                    render_target,
                    source,
                    source_style,
                    target,
                    target_style,
                    config,
                )
            });
    }

    fn join(source: Self, target: Self, config: &super::AnimationDomain) -> Self {
        source
            .into_iter()
            .zip(target)
            .map(|(source, target)| T::join(source, target, config))
            .collect()
    }
}
