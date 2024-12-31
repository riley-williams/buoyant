use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

/// Sets a foreground style
#[derive(Debug, PartialEq)]
pub struct ForegroundStyle<V, Style> {
    style: Style,
    inner: V,
}

impl<V, Color: Copy> ForegroundStyle<V, Color> {
    pub fn new(style: Color, inner: V) -> Self {
        Self { style, inner }
    }
}

impl<Inner: Layout, Color: Copy> Layout for ForegroundStyle<Inner, Color> {
    type Sublayout = Inner::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let modified_env = ForegroundStyleEnv {
            color: self.style,
            wrapped_env: env,
        };
        self.inner.layout(offer, &modified_env)
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) {
        let modified_env = ForegroundStyleEnv {
            color: self.style,
            wrapped_env: env,
        };
        self.inner.place_subviews(layout, origin, &modified_env);
    }
}

impl<Color: Copy, Inner> CharacterRender<Color> for ForegroundStyle<Inner, Color>
where
    Inner: CharacterRender<Color>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Color>,
        layout: &ResolvedLayout<Inner::Sublayout>,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        let modified_env = ForegroundStyleEnv {
            color: self.style,
            wrapped_env: env,
        };

        self.inner.render(target, layout, &modified_env);
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Color, Inner> crate::render::PixelRender<Color> for ForegroundStyle<Inner, Color>
where
    Inner: crate::render::PixelRender<Color>,
    Color: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Color>,
        layout: &ResolvedLayout<Inner::Sublayout>,
        env: &impl RenderEnvironment<Color = Color>,
    ) {
        let modified_env = ForegroundStyleEnv {
            color: self.style,
            wrapped_env: env,
        };

        self.inner.render(target, layout, &modified_env);
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Color>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl RenderEnvironment<Color = Color>,
        target_env: &impl RenderEnvironment<Color = Color>,
        config: &crate::render::AnimationConfiguration,
    ) {
        let source_env = &ForegroundStyleEnv {
            color: source_view.style,
            wrapped_env: source_env,
        };
        let target_env = &ForegroundStyleEnv {
            color: target_view.style,
            wrapped_env: target_env,
        };

        crate::render::PixelRender::render_animated(
            target,
            &source_view.inner,
            source_layout,
            &target_view.inner,
            target_layout,
            source_env,
            target_env,
            config,
        );
    }
}

struct ForegroundStyleEnv<'a, Env, Style> {
    color: Style,
    wrapped_env: &'a Env,
}

impl<E: LayoutEnvironment, C: Copy> LayoutEnvironment for ForegroundStyleEnv<'_, E, C> {
    fn layout_direction(&self) -> crate::layout::LayoutDirection {
        self.wrapped_env.layout_direction()
    }

    fn alignment(&self) -> crate::layout::Alignment {
        self.wrapped_env.alignment()
    }
}

impl<E: RenderEnvironment<Color = Color>, Color: Copy> RenderEnvironment
    for ForegroundStyleEnv<'_, E, Color>
{
    type Color = Color;
    fn foreground_color(&self) -> Color {
        self.color
    }
}
