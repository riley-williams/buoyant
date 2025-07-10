use crate::{
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::{OneOf2, OneOf3},
    view::{ViewLayout, ViewMarker},
};

/// A view that can conditionally render one of N subtrees based on the enum variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchView<T> {
    branch: T,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch2<V0, V1> {
    Variant0(V0),
    Variant1(V1),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch3<V0, V1, V2> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch4<V0, V1, V2, V3> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
    Variant3(V3),
}

// and so on...maybe up to N=10?

impl<V0, V1> MatchView<Branch2<V0, V1>> {
    /// Use the `if_view!` or `match_view!` macros instead of manually instantiating this struct.
    pub const fn new(branch: Branch2<V0, V1>) -> Self {
        Self { branch }
    }
}

impl<V0, V1, V2> MatchView<Branch3<V0, V1, V2>> {
    /// Use the `if_view!` or `match_view!` macros instead of manually instantiating this struct.
    pub const fn new(branch: Branch3<V0, V1, V2>) -> Self {
        Self { branch }
    }
}

/// A view that conditionally renders its arms based on a boolean expression.
///
/// If you need variable bindings, use the [`match_view!`] macro instead.
///
/// # Examples
///
/// ```
/// use buoyant::if_view;
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::view::{shape::Rectangle, Text};
///
/// let font = CharacterBufferFont;
///
/// let view = |value: u32| {
///     if_view!((value % 2 == 0) {
///         Text::new("Even", &font)
///     } else {
///         Text::new("Odd", &font)
///     })
/// };
///
/// let view = |notification_count: u32| {
///     if_view!((notification_count > 0) {
///         Text::new("You have mail", &font)
///     })
/// };
/// ```
#[macro_export]
macro_rules! if_view {
    (
        ($value:expr) {
            $view0:expr
        }
    ) => {{
        let branch = if $value {
            $crate::view::match_view::Branch2::Variant0($view0)
        } else {
            $crate::view::match_view::Branch2::Variant1($crate::view::EmptyView)
        };
        $crate::view::match_view::MatchView::<$crate::view::match_view::Branch2<_, _>>::new(branch)
    }};

    (
        ($value:expr) {
            $view0:expr
        } else {
            $view1:expr
        }
    ) => {{
        let branch = if $value {
            $crate::view::match_view::Branch2::Variant0($view0)
        } else {
            $crate::view::match_view::Branch2::Variant1($view1)
        };
        $crate::view::match_view::MatchView::<$crate::view::match_view::Branch2<_, _>>::new(branch)
    }};
}

/// A view that can conditionally render one of N heterogeneous subtrees based on the enum variant.
/// Enum associated values can be unwrapped in the match arms.
///
/// # Examples
///
/// ```
/// use buoyant::match_view;
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::view::prelude::*;
///
/// #[derive(Clone)]
/// enum State {
///     Message(&'static str),
///     Error,
///     Redacted,
/// }
///
/// let font = CharacterBufferFont;
///
/// let view = |state| {
///     match_view!(state => {
///         State::Message(msg) => Text::new(msg, &font),
///         State::Error => Text::new("Uh oh", &font),
///         State::Redacted => Rectangle,
///     })
/// };
/// ```
#[macro_export]
macro_rules! match_view {
    (
        $value:expr => {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr $(,)?
        }
    ) => {{
        let branch = match $value {
            $pattern0 => $crate::view::match_view::Branch2::Variant0($view0),
            $pattern1 => $crate::view::match_view::Branch2::Variant1($view1),
        };
        $crate::view::match_view::MatchView::<$crate::view::match_view::Branch2<_, _>>::new(branch)
    }};

    (
        $value:expr => {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr $(,)?
        }
    ) => {{
        let branch = match $value {
            $pattern0 => $crate::view::match_view::Branch3::Variant0($view0),
            $pattern1 => $crate::view::match_view::Branch3::Variant1($view1),
            $pattern2 => $crate::view::match_view::Branch3::Variant2($view2),
        };
        $crate::view::match_view::MatchView::<$crate::view::match_view::Branch3<_, _, _>>::new(
            branch,
        )
    }};
}

impl<V0, V1> ViewMarker for MatchView<Branch2<V0, V1>>
where
    V0: ViewMarker,
    V1: ViewMarker,
{
    type Renderables = OneOf2<V0::Renderables, V1::Renderables>;
}

impl<Captures, V0, V1> ViewLayout<Captures> for MatchView<Branch2<V0, V1>>
where
    V0: ViewLayout<Captures>,
    V1: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type Sublayout = Branch2<ResolvedLayout<V0::Sublayout>, ResolvedLayout<V1::Sublayout>>;
    type State = Branch2<V0::State, V1::State>;

    fn priority(&self) -> i8 {
        match &self.branch {
            Branch2::Variant0(v0) => v0.priority(),
            Branch2::Variant1(v1) => v1.priority(),
        }
    }

    fn is_empty(&self) -> bool {
        match &self.branch {
            Branch2::Variant0(v0) => v0.is_empty(),
            Branch2::Variant1(v1) => v1.is_empty(),
        }
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        match &self.branch {
            Branch2::Variant0(v0) => Branch2::Variant0(v0.build_state(captures)),
            Branch2::Variant1(v1) => Branch2::Variant1(v1.build_state(captures)),
        }
    }
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        match &self.branch {
            Branch2::Variant0(v0) => {
                let s0 = if let Branch2::Variant0(s) = state {
                    s
                } else {
                    *state = Branch2::Variant0(v0.build_state(captures));
                    let Branch2::Variant0(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };

                let child_layout = v0.layout(offer, env, captures, s0);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch2::Variant0(child_layout),
                    resolved_size: size,
                }
            }
            Branch2::Variant1(v1) => {
                let s1 = if let Branch2::Variant1(s) = state {
                    s
                } else {
                    *state = Branch2::Variant1(v1.build_state(captures));
                    let Branch2::Variant1(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };
                let child_layout = v1.layout(offer, env, captures, s1);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch2::Variant1(child_layout),
                    resolved_size: size,
                }
            }
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (&self.branch, &layout.sublayouts, state) {
            (Branch2::Variant0(v0), Branch2::Variant0(l0), Branch2::Variant0(s0)) => {
                OneOf2::Variant0(v0.render_tree(l0, origin, env, captures, s0))
            }
            (Branch2::Variant1(v1), Branch2::Variant1(l1), Branch2::Variant1(s1)) => {
                OneOf2::Variant1(v1.render_tree(l1, origin, env, captures, s1))
            }
            // This is reachable if an old layout attempts to be reused
            _ => panic!(
                "Layout/state branch mismatch in conditional view. Layouts cannot be reused."
            ),
        }
    }

    fn handle_event(
        &mut self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        match (&mut self.branch, render_tree, state) {
            (Branch2::Variant0(v0), OneOf2::Variant0(t0), Branch2::Variant0(s0)) => {
                v0.handle_event(event, t0, captures, s0)
            }
            (Branch2::Variant1(v1), OneOf2::Variant1(t1), Branch2::Variant1(s1)) => {
                v1.handle_event(event, t1, captures, s1)
            }
            _ => {
                assert!(
                    !cfg!(debug_assertions),
                    "State branch does not match view branch, likely due to improper reuse of layout."
                );
                false
            }
        }
    }
}

impl<V0, V1, V2> ViewMarker for MatchView<Branch3<V0, V1, V2>>
where
    V0: ViewMarker,
    V1: ViewMarker,
    V2: ViewMarker,
{
    type Renderables = OneOf3<V0::Renderables, V1::Renderables, V2::Renderables>;
}

impl<Captures, V0, V1, V2> ViewLayout<Captures> for MatchView<Branch3<V0, V1, V2>>
where
    V0: ViewLayout<Captures>,
    V1: ViewLayout<Captures>,
    V2: ViewLayout<Captures>,
    Captures: ?Sized,
{
    type Sublayout = Branch3<
        ResolvedLayout<V0::Sublayout>,
        ResolvedLayout<V1::Sublayout>,
        ResolvedLayout<V2::Sublayout>,
    >;
    type State = Branch3<V0::State, V1::State, V2::State>;

    fn priority(&self) -> i8 {
        match &self.branch {
            Branch3::Variant0(v0) => v0.priority(),
            Branch3::Variant1(v1) => v1.priority(),
            Branch3::Variant2(v2) => v2.priority(),
        }
    }

    fn is_empty(&self) -> bool {
        match &self.branch {
            Branch3::Variant0(v0) => v0.is_empty(),
            Branch3::Variant1(v1) => v1.is_empty(),
            Branch3::Variant2(v2) => v2.is_empty(),
        }
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        match &self.branch {
            Branch3::Variant0(v0) => Branch3::Variant0(v0.build_state(captures)),
            Branch3::Variant1(v1) => Branch3::Variant1(v1.build_state(captures)),
            Branch3::Variant2(v2) => Branch3::Variant2(v2.build_state(captures)),
        }
    }
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        match &self.branch {
            Branch3::Variant0(v0) => {
                let s0 = if let Branch3::Variant0(s) = state {
                    s
                } else {
                    *state = Branch3::Variant0(v0.build_state(captures));
                    let Branch3::Variant0(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };
                let child_layout = v0.layout(offer, env, captures, s0);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant0(child_layout),
                    resolved_size: size,
                }
            }
            Branch3::Variant1(v1) => {
                let s1 = if let Branch3::Variant1(s) = state {
                    s
                } else {
                    *state = Branch3::Variant1(v1.build_state(captures));
                    let Branch3::Variant1(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };
                let child_layout = v1.layout(offer, env, captures, s1);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant1(child_layout),
                    resolved_size: size,
                }
            }
            Branch3::Variant2(v2) => {
                let s2 = if let Branch3::Variant2(s) = state {
                    s
                } else {
                    *state = Branch3::Variant2(v2.build_state(captures));
                    let Branch3::Variant2(s) = state else {
                        unreachable!("Guaranteed to not be any other variant")
                    };
                    s
                };
                let child_layout = v2.layout(offer, env, captures, s2);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant2(child_layout),
                    resolved_size: size,
                }
            }
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (&self.branch, &layout.sublayouts, state) {
            (Branch3::Variant0(v0), Branch3::Variant0(l0), Branch3::Variant0(s0)) => {
                OneOf3::Variant0(v0.render_tree(l0, origin, env, captures, s0))
            }
            (Branch3::Variant1(v1), Branch3::Variant1(l1), Branch3::Variant1(s1)) => {
                OneOf3::Variant1(v1.render_tree(l1, origin, env, captures, s1))
            }
            (Branch3::Variant2(v2), Branch3::Variant2(l2), Branch3::Variant2(s2)) => {
                OneOf3::Variant2(v2.render_tree(l2, origin, env, captures, s2))
            }
            // This is reachable if an old layout attempts to be reused
            _ => panic!(
                "Layout/state branch mismatch in conditional view. Layouts cannot be reused."
            ),
        }
    }

    fn handle_event(
        &mut self,
        event: &crate::view::Event,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> bool {
        match (&mut self.branch, render_tree, state) {
            (Branch3::Variant0(v0), OneOf3::Variant0(t0), Branch3::Variant0(s0)) => {
                v0.handle_event(event, t0, captures, s0)
            }
            (Branch3::Variant1(v1), OneOf3::Variant1(t1), Branch3::Variant1(s1)) => {
                v1.handle_event(event, t1, captures, s1)
            }
            (Branch3::Variant2(v2), OneOf3::Variant2(t2), Branch3::Variant2(s2)) => {
                v2.handle_event(event, t2, captures, s2)
            }
            _ => {
                assert!(
                    !cfg!(debug_assertions),
                    "State branch does not match view branch, likely due to improper reuse of layout."
                );
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Branch2::{self, Variant0, Variant1};

    #[test]
    fn match_view() {
        let view = match_view!(1 => {
            0 => 0,
            _ => 1,
        });

        assert_eq!(view.branch, Branch2::<_, u8>::Variant1(1));
    }

    #[test]
    fn match_view_enum() {
        #[allow(unused)]
        enum MyEnum {
            A,
            B,
        }

        let view = match_view!(MyEnum::A => {
            MyEnum::A => 0,
            MyEnum::B => 1,
        });

        assert_eq!(view.branch, Variant0(0));
    }

    #[test]
    fn match_view_variable_binding_enum() {
        #[allow(unused)]
        enum MyEnum {
            A(u8),
            B(f32),
        }

        let view = match_view!(MyEnum::B(3.0) => {
            MyEnum::A(x) => x,
            MyEnum::B(y) => y,
        });
        assert_eq!(view.branch, Variant1(3.0));
    }

    #[test]
    fn match_view_three_branches() {
        #[allow(unused)]
        enum ThreeState {
            First,
            Second,
            Third,
        }

        let view = match_view!(ThreeState::Second => {
            ThreeState::First => 1,
            ThreeState::Second => 2,
            ThreeState::Third => 3,
        });

        assert_eq!(view.branch, super::Branch3::Variant1(2));
    }
}
