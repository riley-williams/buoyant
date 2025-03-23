use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render::{OneOf2, OneOf3, OneOf4, Renderable},
};

use super::match_view::{Branch2, Branch3, Branch4};

/// The axis along which the view should be fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitAxis {
    Vertical,
    Horizontal,
    Both,
}

impl FitAxis {
    #[must_use]
    pub const fn components(self) -> (bool, bool) {
        match self {
            Self::Vertical => (false, true),
            Self::Horizontal => (true, false),
            Self::Both => (true, true),
        }
    }
}

/// Picks the first view that fits the available space by comparing each view's
/// preferred size to the available space.
///
/// If no other view fits, the last view is used.
///
/// Unlike other conditional views, [`ViewThatFits`] does not pass through the
/// empty and priority properties of its children.
///
/// Example:
///
/// ```
/// # use embedded_graphics::mono_font::MonoFont;
/// # const FONT: MonoFont<'_> = embedded_graphics::mono_font::ascii::FONT_5X8;
/// use buoyant::view::{ViewThatFits, FitAxis, Text};
///
/// let charge_view = || {
///     ViewThatFits::new(FitAxis::Vertical, {
///         Text::new("12 hours, 16 minutes, and 3 seconds to full charge", &FONT)
///     })
///     .or(Text::new("12h 16m 3s remaining", &FONT))
///     .or(Text::new("12h ⚡️", &FONT))
/// };
/// ```
///
/// In units of character size, this could produce the following results
/// for the given offered size:
///
/// > **100x1**
/// >
/// > 12 hours, 16 minutes, and 3 seconds to full charge
///
/// > **16x5**
/// >
/// > 12 hours, 16
/// > minutes, and 3
/// > seconds to full
/// > charge
///
/// > **10x2**
/// >
/// > 12h 16m 3s
/// > remaining
///
/// > **5x1**
/// >
/// > 12h ⚡️
#[derive(Debug, Clone)]
pub struct ViewThatFits<T> {
    axis: FitAxis,
    choices: T,
}

impl<T> ViewThatFits<(T,)> {
    #[must_use]
    pub const fn new(axis: FitAxis, view: T) -> Self {
        Self {
            axis,
            choices: (view,),
        }
    }
}

impl<T> ViewThatFits<(T,)> {
    #[must_use]
    pub fn or<V>(self, alternate: V) -> ViewThatFits<(T, V)> {
        ViewThatFits {
            axis: self.axis,
            choices: (self.choices.0, alternate),
        }
    }
}

macro_rules! derive_or {
    ($(($n:tt, $type:ident)),*) => {
        impl<T0, $($type),*> ViewThatFits<(T0, $($type),*)> {
            #[must_use]
            pub fn or<V>(self, alternate: V) -> ViewThatFits<(T0, $($type),*, V)> {
                ViewThatFits {
                    axis: self.axis,
                    choices: (self.choices.0, $(self.choices.$n),*, alternate),
                }
            }
        }
    };
}

// 4 is probably enough...? Making derive macros for this seems tricky
derive_or!((1, T1));
derive_or!((1, T1), (2, T2)); // this derives the 4-tuple variant

// The 1-tuple variant does nothing, so delegate everything to the only child.
impl<T: Layout> Layout for ViewThatFits<(T,)> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.choices.0.layout(offer, env)
    }

    fn priority(&self) -> i8 {
        self.choices.0.priority()
    }

    fn is_empty(&self) -> bool {
        self.choices.0.is_empty()
    }
}

const fn make_compact_offer(from_offer: ProposedDimensions, axis: FitAxis) -> ProposedDimensions {
    match axis {
        FitAxis::Vertical => ProposedDimensions {
            width: from_offer.width,
            height: ProposedDimension::Compact,
        },
        FitAxis::Horizontal => ProposedDimensions {
            width: ProposedDimension::Compact,
            height: from_offer.height,
        },
        FitAxis::Both => ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Compact,
        },
    }
}

impl<T0: Layout, T1: Layout> Layout for ViewThatFits<(T0, T1)> {
    type Sublayout = Branch2<ResolvedLayout<T0::Sublayout>, ResolvedLayout<T1::Sublayout>>;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);
        let mut layout = self.choices.0.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.0.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch2::Variant0(layout),
            };
        }

        let layout = self.choices.1.layout(offer, env);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch2::Variant1(layout),
        }
    }
}

impl<T0: Layout, T1: Layout, T2: Layout> Layout for ViewThatFits<(T0, T1, T2)> {
    type Sublayout = Branch3<
        ResolvedLayout<T0::Sublayout>,
        ResolvedLayout<T1::Sublayout>,
        ResolvedLayout<T2::Sublayout>,
    >;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);

        let mut layout = self.choices.0.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.0.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch3::Variant0(layout),
            };
        }

        let mut layout = self.choices.1.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.1.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch3::Variant1(layout),
            };
        }

        let layout = self.choices.2.layout(offer, env);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch3::Variant2(layout),
        }
    }
}

impl<T0: Layout, T1: Layout, T2: Layout, T3: Layout> Layout for ViewThatFits<(T0, T1, T2, T3)> {
    type Sublayout = Branch4<
        ResolvedLayout<T0::Sublayout>,
        ResolvedLayout<T1::Sublayout>,
        ResolvedLayout<T2::Sublayout>,
        ResolvedLayout<T3::Sublayout>,
    >;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);

        let mut layout = self.choices.0.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.0.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch4::Variant0(layout),
            };
        }

        let mut layout = self.choices.1.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.1.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch4::Variant1(layout),
            };
        }

        let mut layout = self.choices.2.layout(&subview_offer, env);
        if offer.contains(layout.resolved_size, horizontal, vertical) {
            layout = self.choices.2.layout(offer, env);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch4::Variant2(layout),
            };
        }

        let layout = self.choices.3.layout(offer, env);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch4::Variant3(layout),
        }
    }
}

// -- Rendering --

// The 1-tuple variant does nothing, so delegate everything to the only child.
impl<T: Renderable> Renderable for ViewThatFits<(T,)> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        position: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        self.choices.0.render_tree(layout, position, env)
    }
}

impl<T0: Renderable, T1: Renderable> Renderable for ViewThatFits<(T0, T1)> {
    type Renderables = OneOf2<T0::Renderables, T1::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        match &layout.sublayouts {
            Branch2::Variant0(l) => OneOf2::Variant0(self.choices.0.render_tree(l, origin, env)),
            Branch2::Variant1(l) => OneOf2::Variant1(self.choices.1.render_tree(l, origin, env)),
        }
    }
}

impl<T0: Renderable, T1: Renderable, T2: Renderable> Renderable for ViewThatFits<(T0, T1, T2)> {
    type Renderables = OneOf3<T0::Renderables, T1::Renderables, T2::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        match &layout.sublayouts {
            Branch3::Variant0(l) => OneOf3::Variant0(self.choices.0.render_tree(l, origin, env)),
            Branch3::Variant1(l) => OneOf3::Variant1(self.choices.1.render_tree(l, origin, env)),
            Branch3::Variant2(l) => OneOf3::Variant2(self.choices.2.render_tree(l, origin, env)),
        }
    }
}

impl<T0: Renderable, T1: Renderable, T2: Renderable, T3: Renderable> Renderable
    for ViewThatFits<(T0, T1, T2, T3)>
{
    type Renderables = OneOf4<T0::Renderables, T1::Renderables, T2::Renderables, T3::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        match &layout.sublayouts {
            Branch4::Variant0(l) => OneOf4::Variant0(self.choices.0.render_tree(l, origin, env)),
            Branch4::Variant1(l) => OneOf4::Variant1(self.choices.1.render_tree(l, origin, env)),
            Branch4::Variant2(l) => OneOf4::Variant2(self.choices.2.render_tree(l, origin, env)),
            Branch4::Variant3(l) => OneOf4::Variant3(self.choices.3.render_tree(l, origin, env)),
        }
    }
}
