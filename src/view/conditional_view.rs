use crate::{
    layout::{Layout, ProposedDimensions, ResolvedLayout},
    primitives::Point,
    render::CharacterRender,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalView<U, V> {
    pub condition: bool,
    pub true_view: U,
    pub false_view: V,
}

impl<U, V> ConditionalView<U, V> {
    pub fn new(condition: bool, true_view: U, false_view: V) -> Self {
        Self {
            condition,
            true_view,
            false_view,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalViewLayout<U, V> {
    TrueLayout(U),
    FalseLayout(V),
}

impl<U: Layout, V: Layout> Layout for ConditionalView<U, V> {
    type Sublayout =
        ConditionalViewLayout<ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>>;

    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        if self.condition {
            let child_layout = self.true_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: ConditionalViewLayout::TrueLayout(child_layout),
                resolved_size,
            }
        } else {
            let child_layout = self.false_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: ConditionalViewLayout::FalseLayout(child_layout),
                resolved_size,
            }
        }
    }

    fn priority(&self) -> i8 {
        if self.condition {
            self.true_view.priority()
        } else {
            self.false_view.priority()
        }
    }

    fn is_empty(&self) -> bool {
        if self.condition {
            self.true_view.is_empty()
        } else {
            self.false_view.is_empty()
        }
    }
}

impl<Pixel: Copy, U, V> CharacterRender<Pixel> for ConditionalView<U, V>
where
    U: CharacterRender<Pixel>,
    V: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl crate::render_target::CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<
            ConditionalViewLayout<ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>>,
        >,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
        match &layout.sublayouts {
            ConditionalViewLayout::TrueLayout(true_layout) => {
                self.true_view.render(target, true_layout, origin, env)
            }
            ConditionalViewLayout::FalseLayout(false_layout) => {
                self.false_view.render(target, false_layout, origin, env)
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, U, V> crate::render::PixelRender<Pixel> for ConditionalView<U, V>
where
    U: crate::render::PixelRender<Pixel>,
    V: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
        match &layout.sublayouts {
            ConditionalViewLayout::TrueLayout(true_layout) => {
                self.true_view.render(target, true_layout, origin, env)
            }
            ConditionalViewLayout::FalseLayout(false_layout) => {
                self.false_view.render(target, false_layout, origin, env)
            }
        }
    }
}
