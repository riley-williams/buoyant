use crate::render_target::RenderTarget;

use super::{shade::Shader, Render};

macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color, $($type: Render<Color> ),+> Render<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl RenderTarget<Color = Color>,
                shader: &impl Shader<Color = Color>,
            ) {
                $(
                    self.$n.render(target, shader);
                )+
            }

            fn render_animated(
                render_target: &mut impl RenderTarget<Color = Color>,
                source: &Self,
                source_shader: &impl Shader<Color = Color>,
                target: &Self,
                target_shader: &impl Shader<Color = Color>,
                config: &super::AnimationDomain,
            ) {
                $(
                    $type::render_animated(
                        render_target,
                        &source.$n,
                        source_shader,
                        &target.$n,
                        target_shader,
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

impl<Color, T: Render<Color>, const N: usize> Render<Color> for heapless::Vec<T, N> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<Color = Color>,
        shader: &impl Shader<Color = Color>,
    ) {
        self.iter()
            .for_each(|item| item.render(render_target, shader));
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<Color = Color>,
        source: &Self,
        source_shader: &impl Shader<Color = Color>,
        target: &Self,
        target_shader: &impl Shader<Color = Color>,
        config: &super::AnimationDomain,
    ) {
        source
            .iter()
            .zip(target.iter())
            .for_each(|(source, target)| {
                T::render_animated(
                    render_target,
                    source,
                    source_shader,
                    target,
                    target_shader,
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
