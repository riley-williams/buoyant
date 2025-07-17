use embedded_graphics::{image::ImageDrawable, prelude::OriginDimensions};

use crate::{
    layout::ResolvedLayout,
    render::{self},
    view::{ViewLayout, ViewMarker},
};

/// An image that renders raw images conforming to the `ImageDrawable` trait.
///
/// Images are fixed to the size of the image itself.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<'a, T: ?Sized> {
    image: &'a T,
}

impl<'a, T: ImageDrawable + ?Sized> Image<'a, T> {
    #[must_use]
    pub const fn new(image: &'a T) -> Self {
        Self { image }
    }
}

impl<'a, T: ?Sized> ViewMarker for Image<'a, T> {
    type Renderables = render::Image<'a, T>;
}

impl<Captures: ?Sized, T> ViewLayout<Captures> for Image<'_, T>
where
    T: OriginDimensions + ImageDrawable + ?Sized,
{
    type Sublayout = ();
    type State = ();

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}
    fn layout(
        &self,
        _offer: &crate::primitives::ProposedDimensions,
        _env: &impl crate::environment::LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let size = self.image.size().into();
        ResolvedLayout {
            resolved_size: size,
            sublayouts: (),
        }
    }

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        origin: crate::primitives::Point,
        _env: &impl crate::environment::LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        Self::Renderables::new(origin, self.image)
    }
}
