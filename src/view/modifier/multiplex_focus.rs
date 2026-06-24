use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{DefaultFocus, FocusAction, FocusDirection, FocusGroupSet},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiplexFocusTree<T, const N: usize>([Option<T>; N]);

impl<T: DefaultFocus, const N: usize> MultiplexFocusTree<T, N> {
    /// Creates a new `MultiplexFocusTree` with default focus trees.
    #[must_use]
    pub fn new() -> Self {
        Self(core::array::from_fn(|_| None))
    }

    /// Gets or initializes the focus tree for a specific group.
    /// If the tree is None, it initializes it based on the focus direction.
    fn get_or_init(&mut self, group_index: u8, direction: FocusDirection) -> &mut T {
        self.0[group_index as usize].get_or_insert_with(|| match direction {
            FocusDirection::Forward => T::default_first(),
            FocusDirection::Backward => T::default_last(),
        })
    }
}

impl<T: DefaultFocus, const N: usize> DefaultFocus for MultiplexFocusTree<T, N> {
    fn default_first() -> Self {
        Self::new()
    }

    fn default_last() -> Self {
        Self::new()
    }

    fn is_focused(&self) -> bool {
        self.0.iter().flatten().any(DefaultFocus::is_focused)
    }
}

/// A modifier that multiplexes focus into multiple independent focus trees.
///
/// This modifier maintains multiple focus trees (one per group) and selects
/// which one to use based on the focus group of the current event context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiplexFocus<T, const N: usize> {
    child: T,
    groups: [FocusGroupSet; N],
}

impl<T: ViewMarker, const N: usize> MultiplexFocus<T, N> {
    #[must_use]
    pub const fn new(child: T, groups: [FocusGroupSet; N]) -> Self {
        const {
            assert!(N > 0, "MultiplexFocus must have at least one focus group");
        }
        Self { child, groups }
    }
}

impl<T: ViewMarker, const N: usize> ViewMarker for MultiplexFocus<T, N> {
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T: ViewLayout<Captures>, const N: usize> ViewLayout<Captures>
    for MultiplexFocus<T, N>
where
    T::FocusTree: DefaultFocus,
{
    type Sublayout = T::Sublayout;
    type State = T::State;
    type FocusTree = MultiplexFocusTree<T::FocusTree, N>;

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }

    fn transition(&self) -> Self::Transition {
        self.child.transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.child.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.child.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        if let Event::Focus { group, .. } = event {
            // Obtain the index of the first matching set
            let Some(index) = self.groups.iter().position(|set| set.contains(*group)) else {
                return EventResult::Deferred;
            };
            let index = index as u8;

            let group_tree = match event {
                Event::Focus {
                    action: FocusAction::Focus(d),
                    ..
                } => focus.get_or_init(index, *d),
                Event::Focus {
                    action: FocusAction::Next,
                    ..
                } => focus.get_or_init(index, FocusDirection::Forward),
                Event::Focus {
                    action: FocusAction::Previous,
                    ..
                } => focus.get_or_init(index, FocusDirection::Backward),
                Event::Focus {
                    action: FocusAction::Blur | FocusAction::Select | FocusAction::Teardown,
                    ..
                }
                | Event::Scroll(_)
                | Event::KeyUp { .. }
                | Event::KeyDown { .. } => {
                    // Don't auto-initialize for these actions
                    let Some(tree) = focus.0[index as usize].as_mut() else {
                        return EventResult::Deferred;
                    };
                    tree
                }
                Event::Touch(_) => {
                    // Touch events route with depth first search, we can generally expect
                    // child views to want first-focus if uninitialized
                    focus.get_or_init(index, FocusDirection::Forward)
                }
            };

            // Pass the group-specific focus tree to the child
            self.child
                .handle_event(event, context, render_tree, captures, state, group_tree)
        } else {
            // Request is a non-focus event
            let mut candidate_focus = T::FocusTree::default_first();
            let result = self.child.handle_event(
                event,
                context,
                render_tree,
                captures,
                state,
                &mut candidate_focus,
            );

            // For handled events, commit the candidate tree to the reported group
            if let EventResult::Handled { group, .. } = result {
                let focused_index = group.index() as usize;
                if focused_index < N {
                    focus.0[focused_index] = Some(candidate_focus);
                }
            }
            result
        }
    }
}

/// ```compile_fail
/// use buoyant::view::prelude::*;
/// _ = EmptyView.multiplex_focus::<0>();
/// ```
#[expect(unused)]
struct NonZeroGroups;
