use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::Point,
    render::Render,
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
        offer: crate::primitives::Size,
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
}

impl<Pixel: PixelColor, U, V> Render<Pixel> for ConditionalView<U, V>
where
    U: Render<Pixel>,
    V: Render<Pixel>,
{
    fn render(
        &self,
        target: &mut impl crate::render_target::RenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<
            ConditionalViewLayout<ResolvedLayout<U::Sublayout>, ResolvedLayout<V::Sublayout>>,
        >,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Pixel>,
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
