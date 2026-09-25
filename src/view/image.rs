use core::marker::PhantomData;

use embedded_graphics::{image::ImageDrawable, prelude::OriginDimensions};

use crate::{
    event::EventResult,
    layout::ResolvedLayout,
    render::{self},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

pub use render::image::{Original, Template};

/// A view that renders raw images conforming to [`ImageDrawable`].
///
/// Images are fixed to the size of the image itself.
///
/// The `Mode` parameter selects how the image is rendered. It defaults to
/// [`Original`], which draws the image in its own colors. Use
/// [`Image::as_template`] to obtain an [`Image`] backed by the [`Template`]
/// mode, which renders the image as a template by replacing white pixels with
/// the foreground color and rendering black pixels transparent.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<'a, T: ?Sized, Mode = Original> {
    image: &'a T,
    _mode: PhantomData<Mode>,
}

impl<'a, T: ImageDrawable + ?Sized> Image<'a, T, Original> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(image: &'a T) -> Self {
        Self {
            image,
            _mode: PhantomData,
        }
    }

    /// Treats the image as a template, rendering white pixels
    /// with the foreground color and black pixels transparent
    #[must_use]
    pub const fn as_template(self) -> Image<'a, T, Template> {
        Image {
            image: self.image,
            _mode: PhantomData,
        }
    }
}

impl<'a, T: ?Sized, Mode> ViewMarker for Image<'a, T, Mode> {
    type Renderables = render::Image<'a, T, Mode>;
    type Transition = Opacity;
}

impl<Captures: ?Sized, T, Mode> ViewLayout<Captures> for Image<'_, T, Mode>
where
    T: OriginDimensions + ImageDrawable + ?Sized,
{
    type Sublayout = ();
    type State = ();
    type FocusTree = ();

    fn transition(&self) -> Self::Transition {
        Opacity
    }

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
        _layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        _env: &impl crate::environment::LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        Self::Renderables::new(origin, self.image)
    }

    fn handle_event(
        &self,
        _event: &crate::view::Event,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> EventResult {
        EventResult::default()
    }
}
