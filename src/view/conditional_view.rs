use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{AnimationConfiguration, CharacterRender},
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
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        if self.condition {
            let child_layout = self.true_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: ConditionalViewLayout::TrueLayout(child_layout),
                resolved_size,
                origin: Point::zero(),
            }
        } else {
            let child_layout = self.false_view.layout(offer, env);
            let resolved_size = child_layout.resolved_size;
            ResolvedLayout {
                sublayouts: ConditionalViewLayout::FalseLayout(child_layout),
                resolved_size,
                origin: Point::zero(),
            }
        }
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) {
        layout.origin = origin;
        match &mut layout.sublayouts {
            ConditionalViewLayout::TrueLayout(ref mut true_layout) => {
                self.true_view.place_subviews(true_layout, origin, env)
            }
            ConditionalViewLayout::FalseLayout(ref mut false_layout) => {
                self.false_view.place_subviews(false_layout, origin, env)
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
        env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
        match &layout.sublayouts {
            ConditionalViewLayout::TrueLayout(true_layout) => {
                self.true_view.render(target, true_layout, env)
            }
            ConditionalViewLayout::FalseLayout(false_layout) => {
                self.false_view.render(target, false_layout, env)
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
        env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
    ) {
        match &layout.sublayouts {
            ConditionalViewLayout::TrueLayout(true_layout) => {
                self.true_view.render(target, true_layout, env)
            }
            ConditionalViewLayout::FalseLayout(false_layout) => {
                self.false_view.render(target, false_layout, env)
            }
        }
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
        target_env: &impl crate::environment::RenderEnvironment<Color = Pixel>,
        config: &AnimationConfiguration,
    ) {
        // TODO: This should result in a transition
        match (&source_layout.sublayouts, &target_layout.sublayouts) {
            (
                ConditionalViewLayout::TrueLayout(source_layout),
                ConditionalViewLayout::TrueLayout(target_layout),
            ) => crate::render::PixelRender::render_animated(
                target,
                &source_view.true_view,
                source_layout,
                &target_view.true_view,
                target_layout,
                source_env,
                target_env,
                config,
            ),
            (
                ConditionalViewLayout::FalseLayout(source_layout),
                ConditionalViewLayout::FalseLayout(target_layout),
            ) => crate::render::PixelRender::render_animated(
                target,
                &source_view.false_view,
                source_layout,
                &target_view.false_view,
                target_layout,
                source_env,
                target_env,
                config,
            ),
            (
                ConditionalViewLayout::TrueLayout(_),
                ConditionalViewLayout::FalseLayout(target_layout),
            ) => target_view
                .false_view
                .render(target, target_layout, target_env),
            (
                ConditionalViewLayout::FalseLayout(_),
                ConditionalViewLayout::TrueLayout(target_layout),
            ) => target_view
                .true_view
                .render(target, target_layout, target_env),
        }
    }
}
