use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{OneOf2, OneOf3, Renderable},
};

/// A view that can conditionally render one of N subtrees based on the enum variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchView<T> {
    branch: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch2<V0, V1> {
    Variant0(V0),
    Variant1(V1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch3<V0, V1, V2> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
}

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
/// If you need variable bindings, use the ``match_view!`` macro instead.
///
/// Example:
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
/// ```
/// use buoyant::match_view;
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::view::{shape::Rectangle, Text};
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
///
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

impl<V0: Layout, V1: Layout> Layout for MatchView<Branch2<V0, V1>> {
    type Sublayout = Branch2<ResolvedLayout<V0::Sublayout>, ResolvedLayout<V1::Sublayout>>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        match &self.branch {
            Branch2::Variant0(v0) => {
                let child_layout = v0.layout(offer, env);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch2::Variant0(child_layout),
                    resolved_size: size,
                }
            }
            Branch2::Variant1(v1) => {
                let child_layout = v1.layout(offer, env);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch2::Variant1(child_layout),
                    resolved_size: size,
                }
            }
        }
    }

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
}

impl<V0: Layout, V1: Layout, V2: Layout> Layout for MatchView<Branch3<V0, V1, V2>> {
    type Sublayout = Branch3<
        ResolvedLayout<V0::Sublayout>,
        ResolvedLayout<V1::Sublayout>,
        ResolvedLayout<V2::Sublayout>,
    >;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        match &self.branch {
            Branch3::Variant0(v0) => {
                let child_layout = v0.layout(offer, env);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant0(child_layout),
                    resolved_size: size,
                }
            }
            Branch3::Variant1(v1) => {
                let child_layout = v1.layout(offer, env);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant1(child_layout),
                    resolved_size: size,
                }
            }
            Branch3::Variant2(v2) => {
                let child_layout = v2.layout(offer, env);
                let size = child_layout.resolved_size;
                ResolvedLayout {
                    sublayouts: Branch3::Variant2(child_layout),
                    resolved_size: size,
                }
            }
        }
    }

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
}

impl<V0: Renderable, V1: Renderable> Renderable for MatchView<Branch2<V0, V1>> {
    type Renderables = OneOf2<V0::Renderables, V1::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        match (&self.branch, &layout.sublayouts) {
            (Branch2::Variant0(v0), Branch2::Variant0(l0)) => {
                OneOf2::Variant0(v0.render_tree(l0, origin, env))
            }
            (Branch2::Variant1(v1), Branch2::Variant1(l1)) => {
                OneOf2::Variant1(v1.render_tree(l1, origin, env))
            }
            // This is reachable if an old layout attempts to be reused
            _ => panic!("An outdated layout was used"),
        }
    }
}

impl<V0: Renderable, V1: Renderable, V2: Renderable> Renderable for MatchView<Branch3<V0, V1, V2>> {
    type Renderables = OneOf3<V0::Renderables, V1::Renderables, V2::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        match (&self.branch, &layout.sublayouts) {
            (Branch3::Variant0(v0), Branch3::Variant0(l0)) => {
                OneOf3::Variant0(v0.render_tree(l0, origin, env))
            }
            (Branch3::Variant1(v1), Branch3::Variant1(l1)) => {
                OneOf3::Variant1(v1.render_tree(l1, origin, env))
            }
            (Branch3::Variant2(v2), Branch3::Variant2(l2)) => {
                OneOf3::Variant2(v2.render_tree(l2, origin, env))
            }
            // This is reachable if an old layout attempts to be reused
            _ => panic!("An outdated layout was used"),
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
