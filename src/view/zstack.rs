use crate::{
    environment::Environment,
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    pixel::RenderUnit,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};

pub struct ZStack<T> {
    items: T,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T> PartialEq for ZStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<T> ZStack<T> {
    pub fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: alignment,
            ..self
        }
    }

    pub fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: alignment,
            ..self
        }
    }
}

impl<U, V> ZStack<(U, V)> {
    pub fn two(item0: U, item1: V) -> Self {
        ZStack {
            items: (item0, item1),
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }
}

impl<U: Layout, V: Layout> Layout for ZStack<(U, V)> {
    type Sublayout<'a> = (
        ResolvedLayout<U::Sublayout<'a>>,
        ResolvedLayout<V::Sublayout<'a>>,
    ) where U: 'a, V: 'a;

    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<Self::Sublayout<'_>> {
        let layout0 = self.items.0.layout(offer, env);
        let layout1 = self.items.1.layout(offer, env);
        let size = layout0.resolved_size.union(layout1.resolved_size);

        ResolvedLayout {
            sublayouts: (layout0, layout1),
            resolved_size: size.intersection(offer),
        }
    }
}

impl<'a, Pixel, U: Layout, V: Layout>
    Render<
        Pixel,
        (
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
        ),
    > for ZStack<(U, V)>
where
    U: Render<Pixel, U::Sublayout<'a>>,
    V: Render<Pixel, V::Sublayout<'a>>,
    Pixel: RenderUnit,
{
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<(
            ResolvedLayout<U::Sublayout<'a>>,
            ResolvedLayout<V::Sublayout<'a>>,
        )>,
        env: &impl Environment,
    ) {
        let original_window = target.window();

        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.horizontal_alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.0.resolved_size.width as i16,
                    ),
                    self.vertical_alignment.align(
                        layout.resolved_size.height as i16,
                        layout.sublayouts.0.resolved_size.height as i16,
                    ),
                ),
        );

        self.items.0.render(target, &layout.sublayouts.0, env);

        target.set_window_origin(
            original_window.origin
                + Point::new(
                    self.horizontal_alignment.align(
                        layout.resolved_size.width as i16,
                        layout.sublayouts.1.resolved_size.width as i16,
                    ),
                    self.vertical_alignment.align(
                        layout.resolved_size.height as i16,
                        layout.sublayouts.1.resolved_size.height as i16,
                    ),
                ),
        );
        self.items.1.render(target, &layout.sublayouts.1, env);
        target.set_window(original_window);
    }
}
