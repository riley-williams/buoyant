use crate::{
    event::{EventContext, EventResult},
    focus::{DefaultFocus, FocusEvent, FocusStateChange},
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    render::TransitionOption,
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

impl<T: DefaultFocus> DefaultFocus for Option<T> {
    fn default_first() -> Self {
        Some(T::default_first())
    }

    fn default_last() -> Self {
        Some(T::default_last())
    }
}

impl<V> ViewMarker for Option<V>
where
    V: ViewMarker,
{
    type Renderables = TransitionOption<V::Renderables, V::Transition>;
    type Transition = Opacity;
}

impl<Captures, V> ViewLayout<Captures> for Option<V>
where
    V: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type Sublayout = Option<ResolvedLayout<V::Sublayout>>;
    type State = Option<V::State>;
    type FocusTree = Option<V::FocusTree>;

    fn priority(&self) -> i8 {
        self.as_ref().map_or(i8::MIN, ViewLayout::priority)
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_none_or(ViewLayout::is_empty)
    }

    fn transition(&self) -> Self::Transition {
        // transition is not inherited from a child
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.as_ref().map(|v| v.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.as_ref().map_or(
            ResolvedLayout {
                sublayouts: None,
                resolved_size: Dimensions::zero(),
            },
            |v| {
                let s0 = if let Some(s) = state {
                    s
                } else {
                    *state = Some(v.build_state(captures));
                    let Some(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };

                let child_layout = v.layout(offer, env, captures, s0);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Some(child_layout),
                    resolved_size: size,
                }
            },
        )
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (self, layout, state) {
            (Some(v), Some(l0), Some(s0)) => TransitionOption::new_some(
                v.render_tree(&l0.sublayouts, origin, env, captures, s0),
                l0.resolved_size.into(),
                v.transition(),
            ),
            (None, _, _) => TransitionOption::None,
            // This is reachable if an old layout attempts to be reused
            _ => panic!(
                "Layout/state branch mismatch in conditional view. Layouts cannot be reused."
            ),
        }
    }

    #[expect(clippy::assertions_on_constants)]
    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        match (self, render_tree, state) {
            (Some(v), TransitionOption::Some { subtree, .. }, Some(s)) => {
                v.handle_event(event, context, subtree, captures, s)
            }
            (None, _, _) => EventResult::default(),
            _ => {
                assert!(
                    !cfg!(debug_assertions),
                    "State branch does not match view branch, likely due to improper reuse of layout."
                );
                EventResult::default()
            }
        }
    }

    fn focus(
        &self,
        event: &FocusEvent,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        match (self, render_tree, state, focus) {
            (Some(v), TransitionOption::Some { subtree, .. }, Some(s), Some(f)) => {
                v.focus(event, context, subtree, captures, s, f)
            }
            _ => FocusStateChange::Exhausted,
        }
    }
}
