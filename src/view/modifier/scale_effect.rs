use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{
        transform::{LinearTransform, ScaleFactor},
        Frame, Point, ProposedDimensions, UnitPoint,
    },
    render,
    view::{ViewLayout, ViewMarker},
};

/// Applies a scale effect to the inner view.
///
/// The `scale` parameter determines how much the view should be scaled.
/// A value of 1.0 means no scaling, less than 1.0 shrinks the view, and greater than 1.0 enlarges it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaleEffect<V> {
    pub inner: V,
    pub scale: ScaleFactor,
    pub anchor: UnitPoint,
}

impl<V> ScaleEffect<V> {
    pub fn new(inner: V, scale: ScaleFactor, anchor: UnitPoint) -> Self {
        Self {
            inner,
            scale,
            anchor,
        }
    }
}

impl<V> ViewMarker for ScaleEffect<V>
where
    V: ViewMarker,
{
    type Renderables = render::Transform<V::Renderables>;
    type Transition = V::Transition;
}

impl<Captures: ?Sized, V> ViewLayout<Captures> for ScaleEffect<V>
where
    V: ViewLayout<Captures>,
{
    type Sublayout = V::Sublayout;
    type State = V::State;

    fn priority(&self) -> i8 {
        self.inner.priority()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.inner.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.inner.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.inner.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let frame = Frame::new(Point::zero(), layout.resolved_size.into());
        let anchor = self.anchor.in_view_bounds(&frame);
        render::Transform::new(
            self.inner
                .render_tree(layout, -anchor, env, captures, state),
            LinearTransform::new(origin + anchor, self.scale),
        )
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        // FIXME: this is wrong
        let event = event.offset(-render_tree.transform.offset);
        self.inner
            .handle_event(&event, context, &mut render_tree.inner, captures, state)
    }
}
