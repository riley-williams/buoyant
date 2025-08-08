use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// The strategy for scaling a view within the available space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentMode {
    /// Scales the child view to fit within the available space while maintaining its aspect ratio.
    Fit,
    /// Scales the child view to fill the available space while maintaining its aspect ratio.
    Fill,
}

/// The aspect ratio to maintain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio {
    /// A fixed aspect ratio defined by width and height
    Fixed(u32, u32),
    /// Maintains the ideal aspect ratio of the child view.
    ///
    /// For most views this will be a square.
    Ideal,
}

/// A modifier that enforces a specific aspect ratio on its child view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectRatio<T> {
    #[allow(clippy::struct_field_names)]
    aspect_ratio: Ratio,
    content_mode: ContentMode,
    child: T,
}

impl<T> AspectRatio<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(child: T, aspect_ratio: Ratio, content_mode: ContentMode) -> Self {
        Self {
            aspect_ratio,
            content_mode,
            child,
        }
    }
}

impl<T> ViewMarker for AspectRatio<T>
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for AspectRatio<T>
where
    T: ViewLayout<Captures>,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

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
        let (ratio_width, ratio_height) = match self.aspect_ratio {
            Ratio::Fixed(width, height) => (width, height),
            Ratio::Ideal => {
                let child_ideal_size = self
                    .child
                    .layout(&ProposedDimensions::compact(), env, captures, state)
                    .resolved_size;
                (
                    child_ideal_size.width.into(),
                    child_ideal_size.height.into(),
                )
            }
        };

        // Avoid division by zero
        if ratio_width == 0 || ratio_height == 0 {
            return self.child.layout(offer, env, captures, state);
        }

        match (offer.width, offer.height) {
            (ProposedDimension::Exact(w), ProposedDimension::Exact(h)) => {
                let aspect_height = w * ratio_height / ratio_width;
                let aspect_width = h * ratio_width / ratio_height;
                let (final_width, final_height) = match self.content_mode {
                    ContentMode::Fit => {
                        // Choose the smaller scale to fit within bounds
                        if aspect_height <= h {
                            (w, aspect_height)
                        } else {
                            (aspect_width, h)
                        }
                    }
                    ContentMode::Fill => {
                        // Choose the larger scale to fill the space
                        if aspect_height >= h {
                            (w, aspect_height)
                        } else {
                            (aspect_width, h)
                        }
                    }
                };
                let new_offer = ProposedDimensions::new(final_width, final_height);
                self.child.layout(&new_offer, env, captures, state)
            }

            // One exact dimension, one infinite - Fill returns infinite, Fit calculates
            (ProposedDimension::Exact(w), ProposedDimension::Infinite) => match self.content_mode {
                ContentMode::Fit => {
                    let height = w * ratio_height / ratio_width;
                    let new_offer = ProposedDimensions::new(w, height);
                    self.child.layout(&new_offer, env, captures, state)
                }
                ContentMode::Fill => {
                    self.child
                        .layout(&ProposedDimensions::infinite(), env, captures, state)
                }
            },
            (ProposedDimension::Infinite, ProposedDimension::Exact(h)) => match self.content_mode {
                ContentMode::Fit => {
                    let width = h * ratio_width / ratio_height;
                    let new_offer = ProposedDimensions::new(width, h);
                    self.child.layout(&new_offer, env, captures, state)
                }
                ContentMode::Fill => {
                    self.child
                        .layout(&ProposedDimensions::infinite(), env, captures, state)
                }
            },

            // One exact dimension, one compact - always calculate the missing dimension
            (ProposedDimension::Exact(w), ProposedDimension::Compact) => {
                let height = w * ratio_height / ratio_width;
                let new_offer = ProposedDimensions::new(w, height);
                self.child.layout(&new_offer, env, captures, state)
            }
            (ProposedDimension::Compact, ProposedDimension::Exact(h)) => {
                let width = h * ratio_width / ratio_height;
                let new_offer = ProposedDimensions::new(width, h);
                self.child.layout(&new_offer, env, captures, state)
            }

            // All other cases delegate to child
            _ => self.child.layout(offer, env, captures, state),
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        // delegate all rendering to the child
        self.child.render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.child
            .handle_event(event, context, render_tree, captures, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::DefaultEnvironment;
    use crate::primitives::{Dimensions, ProposedDimensions};
    use crate::view::prelude::*;

    #[test]
    fn ideal_aspect_ratio_fit() {
        fn make_view() -> impl View<(), ()> {
            Rectangle
                .flex_frame()
                .with_ideal_size(5, 10)
                .aspect_ratio(Ratio::Ideal, ContentMode::Fit)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn ideal_aspect_ratio_fill() {
        fn make_view() -> impl View<(), ()> {
            let child = Rectangle.flex_frame().with_ideal_size(5, 10);
            AspectRatio::new(child, Ratio::Ideal, ContentMode::Fill)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(500, 1000));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn fixed_aspect_ratio_fit() {
        fn make_view() -> impl View<(), ()> {
            let child = Rectangle.flex_frame().with_ideal_size(10, 10);
            AspectRatio::new(child, Ratio::Fixed(1, 2), ContentMode::Fit)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    }

    #[test]
    fn fixed_aspect_ratio_fill() {
        fn make_view() -> impl View<(), ()> {
            let child = Rectangle.flex_frame().with_ideal_size(10, 10);
            AspectRatio::new(child, Ratio::Fixed(1, 2), ContentMode::Fill)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(500, 1000));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    }

    /// Should inherit the size of its child, even if that means it isn't the requested
    /// aspect ratio.
    #[test]
    fn aspect_ratio_fit_inherits_fixed_child_size() {
        fn make_view() -> impl View<(), ()> {
            AspectRatio::new(Circle, Ratio::Fixed(1, 2), ContentMode::Fit)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 50));
    }

    #[test]
    fn aspect_ratio_fill_inherits_fixed_child_size() {
        fn make_view() -> impl View<(), ()> {
            AspectRatio::new(Circle, Ratio::Fixed(1, 2), ContentMode::Fill)
        }

        let env = DefaultEnvironment::default();
        let view = make_view();
        let mut captures = ();
        let mut state = view.build_state(&mut captures);

        let offer = ProposedDimensions::new(100, 100);
        let layout = view.layout(&offer, &env, &mut captures, &mut state);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 100));
    }

    #[test]
    fn zeros_should_not_panic() {
        fn view(ratio: Ratio, mode: ContentMode) -> impl View<(), ()> {
            AspectRatio::new(Circle, ratio, mode)
        }
        let env = DefaultEnvironment::default();
        let mut captures = ();

        let view1 = view(Ratio::Fixed(0, 2), ContentMode::Fill);
        let mut state1 = view1.build_state(&mut captures);
        let layout = view1.layout(
            &ProposedDimensions::new(1, 1),
            &env,
            &mut captures,
            &mut state1,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let view2 = view(Ratio::Fixed(2, 0), ContentMode::Fill);
        let mut state2 = view2.build_state(&mut captures);
        let layout = view2.layout(
            &ProposedDimensions::new(1, 1),
            &env,
            &mut captures,
            &mut state2,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let view3 = view(Ratio::Fixed(0, 0), ContentMode::Fill);
        let mut state3 = view3.build_state(&mut captures);
        let layout = view3.layout(
            &ProposedDimensions::new(1, 1),
            &env,
            &mut captures,
            &mut state3,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let view4 = view(Ratio::Ideal, ContentMode::Fit);
        let mut state4 = view4.build_state(&mut captures);
        let layout = view4.layout(
            &ProposedDimensions::new(1, 0),
            &env,
            &mut captures,
            &mut state4,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));

        let view5 = view(Ratio::Ideal, ContentMode::Fit);
        let mut state5 = view5.build_state(&mut captures);
        let layout = view5.layout(
            &ProposedDimensions::new(0, 1),
            &env,
            &mut captures,
            &mut state5,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));

        let view6 = view(Ratio::Ideal, ContentMode::Fill);
        let mut state6 = view6.build_state(&mut captures);
        let layout = view6.layout(
            &ProposedDimensions::new(0, 0),
            &env,
            &mut captures,
            &mut state6,
        );
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));
    }
}
