use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
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

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        let modified_env = ForegroundStyleEnv {
            style: self.style,
            wrapped_env: env,
        };
        self.inner.layout(offer, &modified_env)
    }
}

impl<Pixel, Inner, Style> Render<Pixel> for ForegroundStyle<Inner, Style>
where
    Inner: Render<Pixel>,
    Pixel: PixelColor,
    Style: ColorStyle<Color = Pixel>,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Inner::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    ) {
        let modified_env = ForegroundStyleEnv {
            style: self.style,
            wrapped_env: env,
        };

        self.inner.render(target, layout, origin, &modified_env);
    }
}

struct ForegroundStyleEnv<'a, Env, Style> {
    style: Style,
    wrapped_env: &'a Env,
}

impl<E: LayoutEnvironment, Style: ColorStyle> LayoutEnvironment
    for ForegroundStyleEnv<'_, E, Style>
{
    fn layout_direction(&self) -> crate::layout::LayoutDirection {
        self.wrapped_env.layout_direction()
    }

    fn alignment(&self) -> crate::layout::Alignment {
        self.wrapped_env.alignment()
    }
}

impl<E: RenderEnvironment<Style::Color>, Style: ColorStyle> RenderEnvironment<Style::Color>
    for ForegroundStyleEnv<'_, E, Style>
{
    fn foreground_style(&self) -> impl ColorStyle<Color = Style::Color> {
        self.style
    }
}
