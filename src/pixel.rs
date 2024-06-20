pub trait RenderUnit {}

#[cfg(feature = "crossterm")]
impl<T: core::fmt::Display> RenderUnit for crossterm::style::StyledContent<T> {}

impl RenderUnit for char {}
