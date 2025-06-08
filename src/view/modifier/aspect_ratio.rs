use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{ProposedDimension, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentMode {
    /// Scales the child view to fit within the available space while maintaining its aspect ratio.
    Fit,
    /// Scales the child view to fill the available space while maintaining its aspect ratio.
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio {
    /// A fixed aspect ratio defined by width and height
    Fixed(u32, u32),
    /// Maintains the ideal aspect ratio of the child view.
    ///
    /// For most views this will be a square.
    Ideal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectRatio<T> {
    #[allow(clippy::struct_field_names)]
    aspect_ratio: Ratio,
    content_mode: ContentMode,
    child: T,
}

impl<T> AspectRatio<T> {
    #[must_use]
    pub const fn new(child: T, aspect_ratio: Ratio, content_mode: ContentMode) -> Self {
        Self {
            aspect_ratio,
            content_mode,
            child,
        }
    }
}

impl<T: Layout> Layout for AspectRatio<T> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (ratio_width, ratio_height) = match self.aspect_ratio {
            Ratio::Fixed(width, height) => (width, height),
            Ratio::Ideal => {
                // Use child's ideal size to determine aspect ratio
                let child_ideal_size = self
                    .child
                    .layout(&ProposedDimensions::compact(), env)
                    .resolved_size;
                (
                    child_ideal_size.width.into(),
                    child_ideal_size.height.into(),
                )
            }
        };

        // Avoid division by zero
        if ratio_width == 0 || ratio_height == 0 {
            return self.child.layout(offer, env);
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
                self.child.layout(&new_offer, env)
            }

            // One exact dimension, one infinite - Fill returns infinite, Fit calculates
            (ProposedDimension::Exact(w), ProposedDimension::Infinite) => match self.content_mode {
                ContentMode::Fit => {
                    let height = w * ratio_height / ratio_width;
                    let new_offer = ProposedDimensions::new(w, height);
                    self.child.layout(&new_offer, env)
                }
                ContentMode::Fill => self.child.layout(&ProposedDimensions::infinite(), env),
            },
            (ProposedDimension::Infinite, ProposedDimension::Exact(h)) => match self.content_mode {
                ContentMode::Fit => {
                    let width = h * ratio_width / ratio_height;
                    let new_offer = ProposedDimensions::new(width, h);
                    self.child.layout(&new_offer, env)
                }
                ContentMode::Fill => self.child.layout(&ProposedDimensions::infinite(), env),
            },

            // One exact dimension, one compact - always calculate the missing dimension
            (ProposedDimension::Exact(w), ProposedDimension::Compact) => {
                let height = w * ratio_height / ratio_width;
                let new_offer = ProposedDimensions::new(w, height);
                self.child.layout(&new_offer, env)
            }
            (ProposedDimension::Compact, ProposedDimension::Exact(h)) => {
                let width = h * ratio_width / ratio_height;
                let new_offer = ProposedDimensions::new(width, h);
                self.child.layout(&new_offer, env)
            }

            // All other cases delegate to child
            _ => self.child.layout(offer, env),
        }
    }

    fn priority(&self) -> i8 {
        self.child.priority()
    }

    fn is_empty(&self) -> bool {
        self.child.is_empty()
    }
}

impl<T: Renderable> Renderable for AspectRatio<T> {
    type Renderables = T::Renderables;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        // delegate all rendering to the child
        self.child.render_tree(layout, origin, env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::DefaultEnvironment;
    use crate::layout::Layout;
    use crate::primitives::{Dimensions, ProposedDimensions};
    use crate::view::prelude::*;

    #[test]
    fn ideal_aspect_ratio_fit() {
        let env = DefaultEnvironment::default();
        let child = Rectangle.flex_frame().with_ideal_size(5, 10);
        let aspect_ratio = AspectRatio::new(child, Ratio::Ideal, ContentMode::Fit);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn ideal_aspect_ratio_fill() {
        let env = DefaultEnvironment::default();
        let child = Rectangle.flex_frame().with_ideal_size(5, 10);
        let aspect_ratio = AspectRatio::new(child, Ratio::Ideal, ContentMode::Fill);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(500, 1000));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn fixed_aspect_ratio_fit() {
        let env = DefaultEnvironment::default();
        let child = Rectangle.flex_frame().with_ideal_size(10, 10);
        let aspect_ratio = AspectRatio::new(child, Ratio::Fixed(1, 2), ContentMode::Fit);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    }

    #[test]
    fn fixed_aspect_ratio_fill() {
        let env = DefaultEnvironment::default();
        let child = Rectangle.flex_frame().with_ideal_size(10, 10);
        let aspect_ratio = AspectRatio::new(child, Ratio::Fixed(1, 2), ContentMode::Fill);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::new(100, 1000);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(500, 1000));

        let offer = ProposedDimensions::new(ProposedDimension::Infinite, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(ProposedDimension::Compact, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 100));

        let offer = ProposedDimensions::new(100, ProposedDimension::Infinite);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::infinite());

        let offer = ProposedDimensions::new(100, ProposedDimension::Compact);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 200));

        let offer = ProposedDimensions::compact();
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    }

    /// Should inherit the size of its child, even if that means it isn't the requested
    /// aspect ratio.
    #[test]
    fn aspect_ratio_fit_inherits_fixed_child_size() {
        let env = DefaultEnvironment::default();
        let aspect_ratio = AspectRatio::new(Circle, Ratio::Fixed(1, 2), ContentMode::Fit);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(50, 50));
    }

    #[test]
    fn aspect_ratio_fill_inherits_fixed_child_size() {
        let env = DefaultEnvironment::default();
        let aspect_ratio = AspectRatio::new(Circle, Ratio::Fixed(1, 2), ContentMode::Fill);

        let offer = ProposedDimensions::new(100, 100);
        let layout = aspect_ratio.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(100, 100));
    }

    #[test]
    fn zeros_should_not_panic() {
        let env = DefaultEnvironment::default();
        let layout = AspectRatio::new(Circle, Ratio::Fixed(0, 2), ContentMode::Fill)
            .layout(&ProposedDimensions::new(1, 1), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let layout = AspectRatio::new(Circle, Ratio::Fixed(2, 0), ContentMode::Fill)
            .layout(&ProposedDimensions::new(1, 1), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let layout = AspectRatio::new(Circle, Ratio::Fixed(0, 0), ContentMode::Fill)
            .layout(&ProposedDimensions::new(1, 1), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(1, 1));

        let layout = AspectRatio::new(Circle, Ratio::Ideal, ContentMode::Fit)
            .layout(&ProposedDimensions::new(1, 0), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));

        let layout = AspectRatio::new(Circle, Ratio::Ideal, ContentMode::Fit)
            .layout(&ProposedDimensions::new(0, 1), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));

        let layout = AspectRatio::new(Circle, Ratio::Ideal, ContentMode::Fill)
            .layout(&ProposedDimensions::new(0, 0), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(0, 0));
    }
}
