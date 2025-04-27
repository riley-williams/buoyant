use embedded_graphics::{image::ImageDrawable, prelude::OriginDimensions};

use crate::{
    layout::{Layout, ResolvedLayout},
    render::{self, Renderable},
};

/// An image that renders raw images conforming to the `ImageDrawable` trait.
///
/// Images are fixed to the size of the image itself.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<'a, T> {
    image: &'a T,
}

impl<'a, T: ImageDrawable> Image<'a, T> {
    #[must_use]
    pub const fn new(image: &'a T) -> Self {
        Self { image }
    }
}

impl<T: OriginDimensions> Layout for Image<'_, T> {
    type Sublayout = ();

    fn layout(
        &self,
        _offer: &crate::primitives::ProposedDimensions,
        _env: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let size = self.image.size().into();
        ResolvedLayout {
            resolved_size: size,
            sublayouts: (),
        }
    }
}

impl<'a, T: ImageDrawable> Renderable for Image<'a, T> {
    type Renderables = render::Image<'a, T>;

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        _env: &impl crate::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        Self::Renderables::new(origin, self.image)
    }
}
