use crate::{
    event::{EventContext, EventResult},
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    render::{self, TransitionOption},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

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
    ) => {{ if $value { Some($view0) } else { None } }};

    (
        ($value:expr) {
            $view0:expr
        } else {
            $view1:expr
        }
    ) => {{
        if $value {
            $crate::view::match_view::Branch2::V0($view0)
        } else {
            $crate::view::match_view::Branch2::V1($view1)
        }
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
///     match_view!(state, {
///         State::Message(msg) => Text::new(msg, &font),
///         State::Error => Text::new("Uh oh", &font),
///         State::Redacted => Rectangle,
///     })
/// };
/// ```
///
/// Using 4 or more branches will generate an error. If you need more, consider
/// opening a PR to add macro-derived implementations of more.
///
/// ```compile_fail
/// use buoyant::match_view;
/// use buoyant::view::Rectangle;
///
/// enum FourState {
///     First,
///     Second,
///     Third,
///     Fourth,
/// }
///
/// let view = match_view!(FourState::First, {
///     FourState::First => Rectangle,
///     FourState::Second => Rectangle,
///     FourState::Third => Rectangle,
///     FourState::Fourth => Rectangle,
/// });
/// ```
#[macro_export]
macro_rules! match_view {
    (
        $value:expr => {
            $pattern0:pat => $view0:expr,
            $($pattern:pat => $view:expr),* $(,)?
        }
    ) => {{
        compile_error!("Deprecated syntax. Use `match_view!(expr, { ... => ..., })` instead.");
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch1::V0($view0),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch2::V0($view0),
            $pattern1 => $crate::view::match_view::Branch2::V1($view1),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch3::V0($view0),
            $pattern1 => $crate::view::match_view::Branch3::V1($view1),
            $pattern2 => $crate::view::match_view::Branch3::V2($view2),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr,
            $pattern3:pat => $view3:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch4::V0($view0),
            $pattern1 => $crate::view::match_view::Branch4::V1($view1),
            $pattern2 => $crate::view::match_view::Branch4::V2($view2),
            $pattern3 => $crate::view::match_view::Branch4::V3($view3),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr,
            $pattern3:pat => $view3:expr,
            $pattern4:pat => $view4:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch5::V0($view0),
            $pattern1 => $crate::view::match_view::Branch5::V1($view1),
            $pattern2 => $crate::view::match_view::Branch5::V2($view2),
            $pattern3 => $crate::view::match_view::Branch5::V3($view3),
            $pattern4 => $crate::view::match_view::Branch5::V4($view4),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr,
            $pattern3:pat => $view3:expr,
            $pattern4:pat => $view4:expr,
            $pattern5:pat => $view5:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch6::V0($view0),
            $pattern1 => $crate::view::match_view::Branch6::V1($view1),
            $pattern2 => $crate::view::match_view::Branch6::V2($view2),
            $pattern3 => $crate::view::match_view::Branch6::V3($view3),
            $pattern4 => $crate::view::match_view::Branch6::V4($view4),
            $pattern5 => $crate::view::match_view::Branch6::V5($view5),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr,
            $pattern3:pat => $view3:expr,
            $pattern4:pat => $view4:expr,
            $pattern5:pat => $view5:expr,
            $pattern6:pat => $view6:expr $(,)?
        }
    ) => {{
        match $value {
            $pattern0 => $crate::view::match_view::Branch7::V0($view0),
            $pattern1 => $crate::view::match_view::Branch7::V1($view1),
            $pattern2 => $crate::view::match_view::Branch7::V2($view2),
            $pattern3 => $crate::view::match_view::Branch7::V3($view3),
            $pattern4 => $crate::view::match_view::Branch7::V4($view4),
            $pattern5 => $crate::view::match_view::Branch7::V5($view5),
            $pattern6 => $crate::view::match_view::Branch7::V6($view6),
        }
    }};

    (
        $value:expr, {
            $pattern0:pat => $view0:expr,
            $pattern1:pat => $view1:expr,
            $pattern2:pat => $view2:expr,
            $pattern3:pat => $view3:expr,
            $($pattern:pat => $view:expr),*
        }
    ) => {{
        compile_error!("match_view! implements up to 7 branches.");
    }};
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
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (self, &layout.sublayouts, state) {
            (Some(v), Some(l0), Some(s0)) => TransitionOption::new_some(
                v.render_tree(l0, origin, env, captures, s0),
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
}

macro_rules! define_branch {
    ($name:ident, $render:ident, $($variant:ident),+) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name<$($variant),+> {
            $(
                $variant($variant),
            )+
        }
        impl <$($variant),+> Default for $name<$($variant),+>
        where
            V0: Default,
        {
            fn default() -> Self {
                $name::V0(V0::default())
            }
        }

        impl<$($variant),+> ViewMarker for $name<$($variant),+>
            where $($variant: ViewMarker,)+
        {
            type Renderables = render::$render::<$($variant::Renderables),+>;
            type Transition = Opacity;
        }

        #[allow(unreachable_patterns, irrefutable_let_patterns)] // Branch1
        impl<Captures, $($variant),+> ViewLayout<Captures> for $name<$($variant),+>
            where
                Captures: ?Sized,
                $($variant: ViewLayout<Captures>,)+
        {
            type Sublayout = $name<$(ResolvedLayout<$variant::Sublayout>),+>;
            type State = $name<$( $variant::State ),+>;

            fn priority(&self) -> i8 {
                match self {
                    $( Self::$variant(v) => v.priority(),)+
                }
            }

            fn is_empty(&self) -> bool {
                match self {
                    $( Self::$variant(v) => v.is_empty(),)+
                }
            }

            fn transition(&self) -> Self::Transition {
                Opacity
            }

            fn build_state(&self, captures: &mut Captures) -> Self::State {
                match self {
                    $( Self::$variant(v) => $name::$variant(v.build_state(captures)),)+
                }
            }

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl crate::environment::LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> ResolvedLayout<Self::Sublayout> {
                match self {
                    $(
                        Self::$variant(v) => {
                            let s = if let $name::$variant(s) = state {
                                s
                            } else {
                                *state = $name::$variant(v.build_state(captures));
                                let $name::$variant(s) = state else {
                                    unreachable!("Guaranteed to not be any other variant")
                                };
                                s
                            };

                            let child_layout = v.layout(offer, env, captures, s);
                            let size = child_layout.resolved_size;
                            ResolvedLayout {
                                sublayouts: $name::$variant(child_layout),
                                resolved_size: size,
                            }
                        }
                    )+
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
                match (self, &layout.sublayouts) {
                    // somehow it consumes a lot less stack then if match on state too
                    $(
                    (Self::$variant(v), $name::$variant(l0)) => {
                        if let $name::$variant(s) = state {
                            render::$render::$variant(v.render_tree(l0, origin, env, captures, s))
                        } else {
                            let mut s = v.build_state(captures);
                            let renderables =
                                render::$render::$variant(v.render_tree(l0, origin, env, captures, &mut s));
                            *state = $name::$variant(s);
                            renderables
                        }
                    }
                    )+
                    // This is reachable if an old layout attempts to be reused
                    _ => panic!(
                        "Layout/state branch mismatch in conditional view. Layouts cannot be reused."
                    ),
                }
            }

            fn handle_event(
                &self,
                event: &crate::view::Event,
                context: &EventContext,
                render_tree: &mut Self::Renderables,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> EventResult {
                match (self, render_tree, state) {
                    $(
                    (Self::$variant(v), render::$render::$variant(t), $name::$variant(s)) => {
                        v.handle_event(event, context, t, captures, s)
                    }
                    )+
                    _ => {
                        assert!(
                            !cfg!(debug_assertions),
                            "State branch does not match view branch, likely due to improper reuse of layout."
                        );
                        EventResult::default()
                    }
                }
            }
        }
    };
}

define_branch!(Branch1, OneOf1, V0);
define_branch!(Branch2, OneOf2, V0, V1);
define_branch!(Branch3, OneOf3, V0, V1, V2);
define_branch!(Branch4, OneOf4, V0, V1, V2, V3);
define_branch!(Branch5, OneOf5, V0, V1, V2, V3, V4);
define_branch!(Branch6, OneOf6, V0, V1, V2, V3, V4, V5);
define_branch!(Branch7, OneOf7, V0, V1, V2, V3, V4, V5, V6);

#[cfg(test)]
mod tests {
    use super::Branch2::{self, V0, V1};

    #[test]
    fn match_view() {
        let view = match_view!(1, {
            0 => 0,
            _ => 1,
        });

        assert_eq!(view, Branch2::<_, u8>::V1(1));
    }

    #[test]
    fn match_view_enum() {
        #[allow(unused)]
        enum MyEnum {
            A,
            B,
        }

        let view = match_view!(MyEnum::A, {
            MyEnum::A => 0,
            MyEnum::B => 1,
        });

        assert_eq!(view, V0(0));
    }

    #[test]
    fn match_view_variable_binding_enum() {
        #[allow(unused)]
        enum MyEnum {
            A(u8),
            B(f32),
        }

        let view = match_view!(MyEnum::B(3.0), {
            MyEnum::A(x) => x,
            MyEnum::B(y) => y,
        });
        assert_eq!(view, V1(3.0));
    }

    #[test]
    fn match_view_three_branches() {
        #[allow(unused)]
        enum ThreeState {
            First,
            Second,
            Third,
        }

        let view = match_view!(ThreeState::Second, {
            ThreeState::First => 1,
            ThreeState::Second => 2,
            ThreeState::Third => 3,
        });

        assert_eq!(view, super::Branch3::V1(2));
    }
}
