use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
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

impl<Pixel, Inner, Style> CharacterRender<Pixel> for ForegroundStyle<Inner, Style>
where
    Inner: CharacterRender<Pixel>,
    Pixel: PixelColor,
    Style: ColorStyle<Color = Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
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

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, Inner, Style> crate::render::EmbeddedRender<Pixel> for ForegroundStyle<Inner, Style>
where
    Inner: crate::render::EmbeddedRender<Pixel>,
    Pixel: PixelColor,
    Style: ColorStyle<Color = Pixel>,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
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
