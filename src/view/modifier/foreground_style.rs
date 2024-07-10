use crate::{
    environment::Environment,
    layout::{Layout, ResolvedLayout},
    pixel::RenderUnit,
    primitives::Size,
    render::Render,
    render_target::RenderTarget,
    style::color_style::ColorStyle,
};

/// Sets a foreground style
#[derive(Debug, PartialEq)]
pub struct ForegroundStyle<V, Style: ColorStyle> {
    style: Style,
    inner: V,
}

impl<V, Style: ColorStyle> ForegroundStyle<V, Style> {
    pub fn new(style: Style, inner: V) -> Self {
        Self { style, inner }
    }
}

impl<Inner: Layout, Style: ColorStyle> Layout for ForegroundStyle<Inner, Style> {
    type Sublayout = Inner::Sublayout;

    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<Self::Sublayout> {
        let modified_env = ForegroundStyleEnv {
            style: self.style,
            wrapped_env: env,
        };
        self.inner.layout(offer, &modified_env)
    }
}

impl<Pixel, Inner, Style> Render<Pixel, Inner::Sublayout> for ForegroundStyle<Inner, Style>
where
    Inner: Layout + Render<Pixel, Inner::Sublayout>,
    Pixel: RenderUnit,
    Style: ColorStyle,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Inner::Sublayout>,
        env: &impl Environment,
    ) {
        let modified_env = ForegroundStyleEnv {
            style: self.style,
            wrapped_env: env,
        };

        self.inner.render(target, layout, &modified_env);
    }
}

struct ForegroundStyleEnv<'a, Env, Style> {
    style: Style,
    wrapped_env: &'a Env,
}

impl<E: Environment, Style: ColorStyle> Environment for ForegroundStyleEnv<'_, E, Style> {
    fn layout_direction(&self) -> crate::layout::LayoutDirection {
        self.wrapped_env.layout_direction()
    }

    fn alignment(&self) -> crate::layout::Alignment {
        self.wrapped_env.alignment()
    }

    fn foreground_style(&self) -> impl ColorStyle {
        self.style
    }
}
