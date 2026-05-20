#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(missing_docs)]

use core::array;
use core::cmp::max;

use crate::environment::LayoutEnvironment;
use crate::event::{Event, EventContext, EventResult, Key};
use crate::focus::{DefaultFocus, FocusAction, FocusDirection, FocusGroup};
use crate::layout::{HorizontalAlignment, LayoutDirection, ResolvedLayout, VerticalAlignment};
use crate::primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions};
use crate::transition::Opacity;
use crate::view::{ViewLayout, ViewMarker};

use FocusAction::{Blur, Select};

use render::TableRenderable;

use kinda_array::Array;

/// A trait for indexing into a table-like structure
pub trait TableIndex<'a> {
    /// The output type when indexing into the table
    type Output: 'a;

    fn cols(&self) -> usize;
    /// It is guaranteed  that it will only be called within bounds of the table, not as a safety guarantee.
    /// Returns the number of rows in the table
    fn rows(&self) -> usize;
    /// Returns the number of columns in the table
    fn index(&self, x: usize, y: usize) -> Self::Output;
    fn index_row_major(&self, i: usize) -> Self::Output {
        self.index(i % self.cols(), i / self.cols())
    }
}

/// Table algorithm enum
#[derive(Default, Debug, Clone, Copy)]
pub enum TableAlgorithm {
    /// Each column has fixed width; row heights are computed based on content
    #[default]
    FixedWidth,
    /// Both column widths and row heights are fixed and equal
    FixedBoth,
    /// Column widths and row heights are computed based on content
    Auto,
}

#[derive(Debug, Clone, Copy)]
struct CellAlignment(pub HorizontalAlignment, pub VerticalAlignment);

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

/**
A table view that allows creating table with different layouts, borders and flexible cell contents.

# Examples

Basic table with custom data:

```rust
use buoyant::view::{
    prelude::*,
    table::{Table, TableAlgorithm, TableIndex},
};
use embedded_graphics::{pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};

struct Items<'a> {
    width: usize,
    height: usize,
    keys: &'a [&'a str],
    values: &'a [&'a str],
}

impl<'a> TableIndex<'a> for Items<'a> {
    type Output = (&'a str, &'a str);
    fn cols(&self) -> usize {
        self.width
    }
    fn rows(&self) -> usize {
        self.height
    }
    fn index(&self, x: usize, y: usize) -> (&'a str, &'a str) {
        let key = &self.keys[y * self.width + x];
        let value = &self.values[y * self.width + x];
        (key, value)
    }
}

fn table_with_borders<'a>(items: Items<'a>) -> impl View<Rgb565, ()> + 'a {
    Table::<6, 6>::new(items, |(key, value)| {
        HStack::new((
            Text::new(key, &FONT_9X15_BOLD),
            Text::new(": ", &FONT_9X15_BOLD),
            Text::new(value, &FONT_9X15_BOLD),
        ))
    })
    .with_algorithm(TableAlgorithm::FixedWidth)
    .with_stroke(1)
    .padding(Edges::All, 1)
    .background(Alignment::Center, Rectangle.stroked(1))
    .background(
        Alignment::Center,
        Rectangle.foreground_color(Rgb565::new(169, 169, 169)),
    )
}
```
*/
#[derive(Debug, Clone, Copy)]
pub struct Table<const R: usize, const C: usize>;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TableLayout<L: Clone + PartialEq, const R: usize, const C: usize> {
    sublayouts: Array<C, Array<R, ResolvedLayout<L>>>,
    resolved_size_cache: Dimensions,
}

pub type TableRender<L, const R: usize, const C: usize> = TableRenderable<L, R, C>;

/// A view of the [`Table`].
#[derive(Debug)]
pub struct TableView<'a, const R: usize, const C: usize, M, V, F, Captures>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    Captures: ?Sized,
    V: ViewLayout<Captures>,
{
    items: M,
    build_view: F,
    width: usize,
    height: usize,
    algorithm: TableAlgorithm,
    row_stroke: u32,
    col_stroke: u32,
    cell_alignment: CellAlignment,
    _marker: core::marker::PhantomData<(&'a (), Captures)>,
}

/// State of a table view with layout information.
#[derive(Debug, Clone)]
pub struct TableState<VState, const R: usize, const C: usize> {
    pub cell_states: [[VState; R]; C],
    pub col_widths: [u32; C],
    pub row_heights: [u32; R],
    pub captive: Option<FocusGroup>,
}

#[derive(Debug, Clone)]
pub struct Focus<T: DefaultFocus> {
    x: usize,
    y: usize,
    tree: T,
}

impl<L: Clone + PartialEq + Default, const R: usize, const C: usize> TableLayout<L, R, C> {
    pub fn new(
        sublayouts: Array<C, Array<R, ResolvedLayout<L>>>,
        size: Dimensions,
    ) -> ResolvedLayout<Self> {
        ResolvedLayout {
            resolved_size: size,
            sublayouts: Self {
                sublayouts,
                resolved_size_cache: size,
            },
        }
    }
    pub fn default() -> Array<C, Array<R, ResolvedLayout<L>>> {
        Array(array::from_fn(|_| {
            Array(array::from_fn(|_| ResolvedLayout {
                sublayouts: L::default(),
                resolved_size: Dimensions::new(0, 0),
            }))
        }))
    }
}

impl<T: DefaultFocus> DefaultFocus for Focus<T> {
    fn default_first() -> Self {
        Self {
            x: 0,
            y: 0,
            tree: T::default_first(),
        }
    }
    fn default_last() -> Self {
        Self {
            x: usize::MAX,
            y: usize::MAX,
            tree: T::default_last(),
        }
    }
}

impl<T: DefaultFocus> Focus<T> {
    fn normalize_sentinels(&mut self, width: usize, height: usize) {
        if self.x == usize::MAX {
            self.x = width.saturating_sub(1);
        }
        if self.y == usize::MAX {
            self.y = height.saturating_sub(1);
        }
    }
}

impl<VState: Default, const R: usize, const C: usize> Default for TableState<VState, R, C> {
    fn default() -> Self {
        Self::new(array::from_fn(|_| array::from_fn(|_| VState::default())))
    }
}

impl<VState, const R: usize, const C: usize> TableState<VState, R, C> {
    pub fn new(cell_states: [[VState; R]; C]) -> Self {
        const {
            assert!(
                R.checked_mul(C).is_some(),
                "TableView R * C must not overflow"
            );
            assert!(
                R.checked_add(C).is_some(),
                "TableView R + C must not overflow"
            );
            assert!(R > 0 && C > 0, "TableView requires R > 0 and C > 0");
        };

        Self {
            cell_states,
            col_widths: [0; C],
            row_heights: [0; R],
            captive: None,
        }
    }
}

impl CellAlignment {
    pub fn align(self, cell: Dimensions, item: Dimensions) -> Point {
        let h = self.0.align(cell.width.into(), item.width.into());
        let v = self.1.align(cell.height.into(), item.height.into());
        Point::new(h, v)
    }
}

impl<const R: usize, const C: usize> Table<R, C> {
    /// Creates a new table view with the given dimensions, items and view builder function.
    #[expect(clippy::new_ret_no_self)]
    pub fn new<'a, M, V, F, Captures>(
        items: M,
        build_view: F,
    ) -> TableView<'a, R, C, M, V, F, Captures>
    where
        M: TableIndex<'a>,
        F: Fn(M::Output) -> V,
        Captures: ?Sized,
        V: ViewLayout<Captures>,
    {
        let width = items.cols().min(C);
        let height = items.rows().min(R);

        TableView {
            items,
            build_view,
            width,
            height,
            algorithm: TableAlgorithm::default(),
            row_stroke: 0,
            col_stroke: 0,
            cell_alignment: CellAlignment(HorizontalAlignment::Center, VerticalAlignment::Center),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, const R: usize, const C: usize, M, V, F, Captures> TableView<'a, R, C, M, V, F, Captures>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    Captures: ?Sized,
    V: ViewLayout<Captures>,
{
    /// Sets the table layout algorithm.
    #[must_use]
    pub const fn with_algorithm(mut self, algorithm: TableAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Sets both row and column stroke.
    #[must_use]
    pub const fn with_stroke(mut self, stroke: u32) -> Self {
        self.row_stroke = stroke;
        self.col_stroke = stroke;
        self
    }

    /// Sets the row stroke.
    #[must_use]
    pub const fn with_row_stroke(mut self, stroke: u32) -> Self {
        self.row_stroke = stroke;
        self
    }

    /// Sets the column stroke.
    #[must_use]
    pub const fn with_col_stroke(mut self, stroke: u32) -> Self {
        self.col_stroke = stroke;
        self
    }

    /// Sets both horizontal and vertical cell alignment.
    #[must_use]
    pub const fn with_cell_alignment(
        mut self,
        h: HorizontalAlignment,
        v: VerticalAlignment,
    ) -> Self {
        self.cell_alignment = CellAlignment(h, v);
        self
    }

    /// Sets the horizontal cell alignment.
    #[must_use]
    pub const fn with_horizontal_cell_alignment(mut self, h: HorizontalAlignment) -> Self {
        self.cell_alignment.0 = h;
        self
    }

    /// Sets the vertical cell alignment.
    #[must_use]
    pub const fn with_vertical_cell_alignment(mut self, v: VerticalAlignment) -> Self {
        self.cell_alignment.1 = v;
        self
    }
}

impl<'a, const R: usize, const C: usize, M, V, F, Captures> ViewMarker
    for TableView<'a, R, C, M, V, F, Captures>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    Captures: ?Sized,
    V: ViewLayout<Captures>,
{
    type Renderables = render::TableRenderable<V::Renderables, R, C>;
    type Transition = Opacity;
}

impl<'a, const R: usize, const C: usize, M, V, F, Captures> ViewLayout<Captures>
    for TableView<'a, R, C, M, V, F, Captures>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    Captures: ?Sized,
    V: ViewLayout<Captures>,
    V::Sublayout: Default,
{
    type Sublayout = TableLayout<V::Sublayout, R, C>;
    type State = TableState<V::State, R, C>;
    type FocusTree = Focus<V::FocusTree>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let cell_states = array::from_fn(|c| {
            array::from_fn(|r| {
                if c >= self.width || r >= self.height {
                    V::State::default()
                } else {
                    let view = (self.build_view)(self.items.index(c, r));
                    view.build_state(captures)
                }
            })
        });
        TableState {
            cell_states,
            col_widths: [0; C],
            row_heights: [0; R],
            captive: None,
        }
    }

    #[allow(clippy::too_many_lines)]
    #[inline(never)]
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        // hopefully will not construct on stack
        let mut sublayouts = TableLayout::<V::Sublayout, R, C>::default();

        let mut layout_fn = |c: usize, r: usize, offer: ProposedDimensions| {
            debug_assert!(c < self.width && r < self.height);
            if c >= self.width || r >= self.height {
                return Dimensions {
                    width: 0u32.into(),
                    height: 0u32.into(),
                };
            }
            let view = (self.build_view)(self.items.index(c, r));
            let cell_state = &mut state.cell_states[c][r];
            let layout = view.layout(&offer, env, captures, cell_state);
            sublayouts[c][r] = layout;
            sublayouts[c][r].resolved_size
        };

        state.col_widths = [0; C];
        state.row_heights = [0; R];

        let offer_dims = Dimensions {
            width: offer.width.resolve_most_flexible(0, 0),
            height: offer.height.resolve_most_flexible(0, 0),
        };

        match self.algorithm {
            TableAlgorithm::FixedBoth => {
                // require exact width and height
                let ProposedDimension::Exact(total_width) = offer.width else {
                    return TableLayout::new(sublayouts, offer_dims);
                };
                let ProposedDimension::Exact(total_height) = offer.height else {
                    return TableLayout::new(sublayouts, offer_dims);
                };

                let stroke_w = self.col_stroke * self.width.saturating_sub(1) as u32;
                let stroke_h = self.row_stroke * self.height.saturating_sub(1) as u32;
                let div_w = self.width.max(1) as u32;
                let div_h = self.height.max(1) as u32;
                let base_w = total_width.saturating_sub(stroke_w) / div_w;
                let base_h = total_height.saturating_sub(stroke_h) / div_h;
                let rem_w = (total_width.saturating_sub(stroke_w) % div_w) as usize;
                let rem_h = (total_height.saturating_sub(stroke_h) % div_h) as usize;

                for c in 0..self.width {
                    state.col_widths[c] = base_w + u32::from(c < rem_w);
                }
                for r in 0..self.height {
                    state.row_heights[r] = base_h + u32::from(r < rem_h);
                }

                for c in 0..self.width {
                    for r in 0..self.height {
                        let final_offer = ProposedDimensions {
                            width: ProposedDimension::Exact(state.col_widths[c]),
                            height: ProposedDimension::Exact(state.row_heights[r]),
                        };
                        let _ = layout_fn(c, r, final_offer);
                    }
                }

                TableLayout::new(
                    sublayouts,
                    Dimensions {
                        width: Dimension::from(total_width),
                        height: Dimension::from(total_height),
                    },
                )
            }
            TableAlgorithm::FixedWidth => {
                // require exact width and height
                let ProposedDimension::Exact(total_width) = offer.width else {
                    return TableLayout::new(sublayouts, offer_dims);
                };

                let stroke_w = self.col_stroke * self.width.saturating_sub(1) as u32;
                let div_w = self.width.max(1) as u32;
                let base_w = total_width.saturating_sub(stroke_w) / div_w;
                let rem_w = (total_width.saturating_sub(stroke_w) % div_w) as usize;

                for c in 0..self.width {
                    state.col_widths[c] = base_w + u32::from(c < rem_w);
                }

                for c in 0..self.width {
                    let mut subviews_col: [(i8, bool); R] = [(i8::MIN, true); R];
                    for r in 0..self.height {
                        let view = (self.build_view)(self.items.index(c, r));
                        subviews_col[r] = (view.priority(), view.is_empty());
                    }

                    let _ = layout_n(
                        &subviews_col[..self.height],
                        &mut [0usize; R][..self.height],
                        &mut [0u32.into(); R][..self.height],
                        LayoutDirection::Vertical,
                        ProposedDimensions {
                            width: ProposedDimension::Exact(state.col_widths[c]),
                            height: offer.height,
                        },
                        self.row_stroke,
                        &mut |r: usize, offered: ProposedDimensions, commit: bool| {
                            let dims = layout_fn(c, r, offered);
                            if commit {
                                state.row_heights[r] =
                                    max(state.row_heights[r], dims.height.into());
                            }
                            dims
                        },
                    );
                }

                /*
                // I think final pass would be needed to give smaller component a chanse to grow
                // bigger due to its siblings, but `layout_n` gave them the highest possible heights
                // already, thus there is nowhere to grow anymore.
                for c in 0..self.width {
                    for r in 0..self.height {
                        let final_offer = ProposedDimensions {
                            width: ProposedDimension::Exact(state.col_widths[c]),
                            height: ProposedDimension::Exact(state.row_heights[r]),
                        };
                        let _ = layout_fn(c, r, final_offer);
                    }
                }
                */

                let mut widths = state.col_widths.iter().copied();
                let total_width = widths
                    .try_fold(0u32, |a, b| a.checked_add(b))
                    .and_then(|w| {
                        w.checked_add(self.col_stroke * self.width.saturating_sub(1) as u32)
                    });

                let mut heights = state.row_heights.iter().copied();
                let total_height = heights
                    .try_fold(0u32, |a, b| a.checked_add(b))
                    .and_then(|w| {
                        w.checked_add(self.row_stroke * self.height.saturating_sub(1) as u32)
                    });

                TableLayout::new(
                    sublayouts,
                    Dimensions {
                        width: Dimension::from(total_width.unwrap_or(u32::MAX)),
                        height: Dimension::from(total_height.unwrap_or(u32::MAX)),
                    },
                )
            }

            /*

            For general case table layout seems to require a [convergence algorithm].
            HTML's `table-layout: auto;` is cool but isn't specified. Non-normative part of
            the [w3c spec] talks about determining maximing minimums and maximums for each column,
            but it doesn't seem to highlight how to distribute the space between columns.

            So this is like a "single covergence step". `layout_n` is used to get plausible
            heights by giving columns equal (±1) width. Then those height are used in the
            second `layout_n` which computes widths based on those heights. Finally,
            it uses computed width and height to actually do the layout of each cell. It
            doesn't give perfect widths, but seems good enough. I believe there would be a
            better algorithm to get good results and with less work, but this is for further
            work.

            [convergence algorithm](https://math.stackexchange.com/questions/3305027/optimize-table-layout)
            [w3c spec](https://www.w3.org/TR/2006/WD-CSS21-20061106/tables.html#auto-table-layout)

            */
            TableAlgorithm::Auto => {
                let mut subviews_agg_row: [(i8, bool); C] = [(i8::MIN, true); C];
                for c in 0..self.width {
                    let mut is_empty = true;
                    let mut priority = i8::MIN;
                    for r in 0..self.height {
                        let view = (self.build_view)(self.items.index(c, r));
                        is_empty &= view.is_empty();
                        priority = max(priority, view.priority());
                    }
                    subviews_agg_row[c] = (priority, is_empty);
                }

                // If the offered height is Exact, offer a fraction of it to get more realistic widths
                let height_offer = match offer.height {
                    ProposedDimension::Exact(h) => {
                        let div = self.height.max(1) as u32;
                        let spaces = self.row_stroke * self.height.saturating_sub(1) as u32;
                        let h = h.saturating_sub(spaces);
                        ProposedDimension::Exact(h / div)
                    }
                    other => other,
                };

                let _ = layout_n(
                    &subviews_agg_row[..self.width],
                    &mut [0; C][..self.width],
                    &mut [0u32.into(); C][..self.width],
                    LayoutDirection::Horizontal,
                    ProposedDimensions {
                        width: offer.width,
                        height: height_offer,
                    },
                    self.col_stroke,
                    &mut |c: usize, offered: ProposedDimensions, commit: bool| {
                        use ProposedDimension as PD;
                        let mut dims = Dimensions {
                            width: match offered.width {
                                PD::Infinite | PD::Exact(_) => 0u32.into(),
                                PD::Compact => u32::MAX.into(),
                            },
                            height: match offered.height {
                                PD::Infinite | PD::Exact(_) => 0u32.into(),
                                PD::Compact => u32::MAX.into(),
                            },
                        };
                        for r in 0..self.height {
                            let cell_dims = layout_fn(c, r, offered);
                            dims.width = dims.width.max(cell_dims.width);
                            dims.height = dims.height.max(cell_dims.height);
                        }
                        if commit {
                            state.col_widths[c] = dims.width.into();
                        }
                        dims
                    },
                );

                for c in 0..self.width {
                    let mut subviews_col: [(i8, bool); R] = [(i8::MIN, true); R];
                    for r in 0..self.height {
                        let view = (self.build_view)(self.items.index(c, r));
                        subviews_col[r] = (view.priority(), view.is_empty());
                    }

                    let _ = layout_n(
                        &subviews_col[..self.height],
                        &mut [0usize; R][..self.height],
                        &mut [0u32.into(); R][..self.height],
                        LayoutDirection::Vertical,
                        ProposedDimensions {
                            width: ProposedDimension::Exact(state.col_widths[c]),
                            height: offer.height,
                        },
                        self.row_stroke,
                        &mut |r: usize, offered: ProposedDimensions, commit: bool| {
                            let dims = layout_fn(c, r, offered);
                            if commit {
                                state.row_heights[r] =
                                    max(state.row_heights[r], dims.height.into());
                            }
                            dims
                        },
                    );
                }

                /*
                // I think final pass would be needed to give smaller component a chanse to grow
                // bigger due to its siblings, but `layout_n` gave them the highest possible heights
                // already, thus there is nowhere to grow anymore.
                for c in 0..self.width {
                    for r in 0..self.height {
                        let final_offer = ProposedDimensions {
                            width: ProposedDimension::Exact(state.col_widths[c]),
                            height: ProposedDimension::Exact(state.row_heights[r]),
                        };
                        let _ = layout_fn(c, r, final_offer);
                    }
                }
                */

                let total_width: u32 = state.col_widths.iter().copied().sum::<u32>()
                    + self.col_stroke * self.width.saturating_sub(1) as u32;
                let total_height: u32 = state.row_heights.iter().copied().sum::<u32>()
                    + self.row_stroke * self.height.saturating_sub(1) as u32;

                TableLayout::new(
                    sublayouts,
                    Dimensions {
                        width: Dimension::from(total_width),
                        height: Dimension::from(total_height),
                    },
                )
            }
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let mut x = origin.x;
        let mut render = TableRenderable::<V::Renderables, R, C>::default();
        let renderables = &mut render.renderables;
        let sublayouts = &layout.sublayouts;

        #[inline(never)]
        fn push_empty<T: Default>(vec: &mut heapless::VecView<T>) {
            _ = vec.push(T::default());
        }

        for c in 0..self.width {
            push_empty(&mut *renderables);
            let col = &mut renderables[c];
            let mut y = origin.y;
            let cell_w = state.col_widths[c];

            for r in 0..self.height {
                let cell_h = state.row_heights[r];
                let dims = Dimensions {
                    width: Dimension::from(cell_w),
                    height: Dimension::from(cell_h),
                };

                let sublayout = &sublayouts[c][r];
                let offset = self.cell_alignment.align(dims, sublayout.resolved_size);
                let cell_origin = offset + Point::new(x, y);

                let view = (self.build_view)(self.items.index(c, r));
                let res = col.push(view.render_tree(
                    &sublayout.sublayouts,
                    cell_origin,
                    env,
                    captures,
                    &mut state.cell_states[c][r],
                ));
                assert!(res.is_ok());

                y += cell_h as i32 + self.row_stroke as i32;
            }
            x += cell_w as i32 + self.col_stroke as i32;
        }

        render.origin = origin;
        render.resolved_size = layout.resolved_size_cache;
        render.width = self.width;
        render.height = self.height;
        render.col_widths = state.col_widths;
        render.row_heights = state.row_heights;
        render.col_stroke = self.col_stroke;
        render.row_stroke = self.row_stroke;

        render
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
        focus: &mut Self::FocusTree,
    ) -> EventResult {
        focus.normalize_sentinels(self.width, self.height);

        let mut last_result = EventResult::deferred();

        let (x, y) = (focus.x, focus.y);

        let mut event = event.clone();

        let render_tree = &mut render_tree.renderables;

        let mut mov = Move::from_event(&event);
        let mut captive = state.captive;

        let is_blur = matches!(event, Event::Focus { action: Blur, .. });
        if is_blur && state.captive.is_some() {
            state.captive = None;
        }

        match (state.captive, mov, &mut event) {
            (Some(_), _, _) => (),
            (_, Some(mov), _) => {
                // FIXME: all events shall carry group
                let group = const { FocusGroup::new(0).expect("const") };
                state.captive = Some(group);
                captive = state.captive;
                event = mov.focus_event(group);
            }
            (_, _, Event::Focus { action, group }) if *action == Select => {
                state.captive = Some(*group);
                captive = state.captive;
                *action = FocusAction::Focus(FocusDirection::Forward);
                mov = Some(Move::Right);
            }
            _ => (),
        }

        let state = &mut state.cell_states;

        if let Some(mov) = mov
            && let Some(group) = captive
        {
            let [dx, dy, odx, ody]: [isize; 4] = match mov {
                Move::Up => [0, -1, -1, 0],
                Move::Down => [0, 1, 1, 0],
                Move::Left => [-1, 0, 0, -1],
                Move::Right => [1, 0, 0, 1],
            };

            assert_eq!(dx, ody);
            assert_eq!(dy, odx);

            let mut step_count = (self.width * self.height).saturating_sub(1);
            let dir = dx + dy;
            let m = |p: usize, d: isize| (p as isize + d).cast_unsigned();

            let mut view = (self.build_view)(self.items.index(focus.x, focus.y));
            last_result = view.handle_event(
                &event,
                context,
                &mut render_tree[focus.x][focus.y],
                captures,
                &mut state[focus.x][focus.y],
                &mut focus.tree,
            );

            if !matches!(last_result, EventResult::Deferred { .. }) {
                return last_result;
            }

            while step_count != 0 {
                step_count -= 1;

                let _ = view.handle_event(
                    &Event::Focus {
                        action: FocusAction::Blur,
                        group,
                    },
                    context,
                    &mut render_tree[focus.x][focus.y],
                    captures,
                    &mut state[focus.x][focus.y],
                    &mut focus.tree,
                );

                (focus.x, focus.y) = if (focus.x, focus.y) == (0, 0) && dir < 0 {
                    (self.width - 1, self.height - 1)
                } else if (focus.x, focus.y) == (self.width - 1, self.height - 1) && dir > 0 {
                    (0, 0)
                } else {
                    focus.x = m(focus.x, dx);
                    focus.y = m(focus.y, dy);

                    match (focus.x, focus.y) {
                        _ if focus.x == self.width => (0, m(focus.y, ody)),
                        _ if focus.y == self.height => (m(focus.x, odx), 0),
                        _ if focus.x == usize::MAX => (self.width - 1, m(focus.y, ody)),
                        _ if focus.y == usize::MAX => (m(focus.x, odx), self.height - 1),
                        _ => (focus.x, focus.y),
                    }
                };

                view = (self.build_view)(self.items.index(focus.x, focus.y));
                last_result = view.handle_event(
                    &mov.focus_event(group),
                    context,
                    &mut render_tree[focus.x][focus.y],
                    captures,
                    &mut state[focus.x][focus.y],
                    &mut focus.tree,
                );

                if !matches!(last_result, EventResult::Deferred { .. }) {
                    return last_result;
                }
            }

            return last_result;
        }

        if captive.is_some() {
            let view = (self.build_view)(self.items.index(x, y));
            return view.handle_event(
                &event,
                context,
                &mut render_tree[x][y],
                captures,
                &mut state[x][y],
                &mut focus.tree,
            );
        }

        if captive.is_none() && mov.is_some() {
            return EventResult::deferred();
        }

        for r in 0..self.height {
            for c in 0..self.width {
                let view = (self.build_view)(self.items.index(c, r));
                let mut default_focus = DefaultFocus::default_first();
                last_result = view.handle_event(
                    &event,
                    context,
                    &mut render_tree[c][r],
                    captures,
                    &mut state[c][r],
                    if (c, r) == (x, y) {
                        &mut focus.tree
                    } else {
                        &mut default_focus
                    },
                );
                if last_result.is_handled() {
                    return last_result;
                }
            }
        }

        return last_result;
    }
}

// We can't use plain arrays there due to https://github.com/rust-lang/rust/issues/61415
mod kinda_array {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Copy)]
    pub struct Array<const N: usize, T>(pub [T; N]);

    impl<const N: usize, T: Default> Default for Array<N, T> {
        fn default() -> Self {
            Self(core::array::from_fn(|_| T::default()))
        }
    }
    impl<const N: usize, T> core::ops::Index<usize> for Array<N, T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }
    impl<const N: usize, T> core::ops::IndexMut<usize> for Array<N, T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }
    impl<const N: usize, const M: usize, T: Default> Array<N, Array<M, T>> {
        pub fn to_default(&mut self) {
            for n in 0..N {
                for m in 0..M {
                    self.0[n].0[m] = T::default();
                }
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn layout_n(
    subviews: &[(i8, bool)],
    subviews_indices_alloc: &mut [usize],
    flexibilities_alloc: &mut [Dimension],
    direction: LayoutDirection,
    offer: ProposedDimensions,
    stroke: u32,
    layout_fn: &mut dyn FnMut(usize, ProposedDimensions, bool) -> Dimensions,
) -> Dimensions {
    let proposed_dimension = match direction {
        LayoutDirection::Horizontal => offer.width,
        LayoutDirection::Vertical => offer.height,
    };

    let ProposedDimension::Exact(size) = proposed_dimension else {
        // Compact or infinite offer
        let mut total_size: Dimension = 0u32.into();
        let mut max_cross_size: Dimension = 0u32.into();
        let mut non_empty_views: u32 = 0;
        for (i, (_, is_empty)) in subviews.iter().enumerate() {
            let dimensions = layout_fn(i, offer, true);
            if *is_empty {
                continue;
            }

            let (size, cross_size) = match direction {
                LayoutDirection::Vertical => (dimensions.height, dimensions.width),
                LayoutDirection::Horizontal => (dimensions.width, dimensions.height),
            };
            total_size += size;
            max_cross_size = max(max_cross_size, cross_size);
            non_empty_views += 1;
        }
        return match direction {
            LayoutDirection::Horizontal => Dimensions {
                width: total_size + stroke * (non_empty_views.saturating_sub(1)),
                height: max_cross_size,
            },
            LayoutDirection::Vertical => Dimensions {
                width: max_cross_size,
                height: total_size + stroke * (non_empty_views.saturating_sub(1)),
            },
        };
    };

    // compute the "flexibility" of each view on the vertical axis and sort by decreasing
    // flexibility
    flexibilities_alloc.fill(Dimension::from(0u32));
    let mut num_empty_views = 0;
    let (min_proposal, max_proposal) = match direction {
        LayoutDirection::Horizontal => (
            ProposedDimensions {
                width: ProposedDimension::Exact(0),
                height: offer.height,
            },
            ProposedDimensions {
                width: ProposedDimension::Infinite,
                height: offer.height,
            },
        ),
        LayoutDirection::Vertical => (
            ProposedDimensions {
                width: offer.width,
                height: ProposedDimension::Exact(0),
            },
            ProposedDimensions {
                width: offer.width,
                height: ProposedDimension::Infinite,
            },
        ),
    };

    for index in 0..subviews.len() {
        let is_empty = subviews[index].1;
        let minimum_dimension = layout_fn(index, min_proposal, is_empty);
        if is_empty {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = layout_fn(index, max_proposal, false);
        flexibilities_alloc[index] = match direction {
            LayoutDirection::Horizontal => maximum_dimension.width - minimum_dimension.width,
            LayoutDirection::Vertical => maximum_dimension.height - minimum_dimension.height,
        };
    }

    let len = subviews.len() as u32;
    let mut remaining_size = size.saturating_sub(stroke * len.saturating_sub(num_empty_views + 1));
    let mut last_priority_group: Option<i8> = None;
    let mut max_cross_size: Dimension = 0u32.into();
    loop {
        subviews_indices_alloc.fill(0);
        let mut max = i8::MIN;
        let mut slice_start: usize = 0;
        let mut slice_len: usize = 0;
        for (i, (priority, is_empty)) in subviews.iter().enumerate() {
            if last_priority_group.is_some_and(|p| p <= *priority) || *is_empty {
                continue;
            }
            match max.cmp(priority) {
                core::cmp::Ordering::Less => {
                    max = *priority;
                    slice_start = i;
                    slice_len = 1;
                    subviews_indices_alloc[slice_start] = i;
                }
                core::cmp::Ordering::Equal => {
                    if slice_len == 0 {
                        slice_start = i;
                    }

                    subviews_indices_alloc[slice_start + slice_len] = i;
                    slice_len += 1;
                }
                core::cmp::Ordering::Greater => {}
            }
        }
        last_priority_group = Some(max);

        if slice_len == 0 {
            break;
        }

        let group_indices = &mut subviews_indices_alloc[slice_start..slice_start + slice_len];
        group_indices.sort_unstable_by_key(|&i| flexibilities_alloc[i]);

        let mut remaining_group_size = group_indices.len() as u32;

        match direction {
            LayoutDirection::Horizontal => {
                for index in group_indices {
                    let width_fraction = remaining_size / remaining_group_size
                        + remaining_size % remaining_group_size;
                    let size = layout_fn(
                        *index,
                        ProposedDimensions {
                            width: ProposedDimension::Exact(width_fraction),
                            height: offer.height,
                        },
                        true,
                    );
                    remaining_size = remaining_size.saturating_sub(size.width.into());
                    remaining_group_size -= 1;
                    max_cross_size = max_cross_size.max(size.height);
                }
            }
            LayoutDirection::Vertical => {
                for index in group_indices {
                    let height_fraction = remaining_size / remaining_group_size
                        + remaining_size % remaining_group_size;
                    let size = layout_fn(
                        *index,
                        ProposedDimensions {
                            width: offer.width,
                            height: ProposedDimension::Exact(height_fraction),
                        },
                        true,
                    );
                    remaining_size = remaining_size.saturating_sub(size.height.into());
                    remaining_group_size -= 1;
                    max_cross_size = max_cross_size.max(size.width);
                }
            }
        }
    }

    match direction {
        LayoutDirection::Horizontal => Dimensions {
            width: (size.saturating_sub(remaining_size)).into(),
            height: max_cross_size,
        },
        LayoutDirection::Vertical => Dimensions {
            width: max_cross_size,
            height: (size.saturating_sub(remaining_size)).into(),
        },
    }
}

mod render {
    use heapless::Vec;

    use crate::{
        primitives::{
            Dimensions, Interpolate, Point, Size, geometry::Rectangle, transform::LinearTransform,
        },
        render::{AnimatedJoin, AnimationDomain, ContentShape, IntrinsicShape, Render},
        render_target::{RenderTarget, SolidBrush},
    };

    #[derive(Debug, Clone)]
    pub struct TableRenderable<T, const R: usize, const C: usize> {
        pub renderables: Vec<Vec<T, R>, C>,
        pub origin: Point,
        pub resolved_size: Dimensions,
        pub width: usize,
        pub height: usize,
        pub col_widths: [u32; C],
        pub row_heights: [u32; R],
        pub col_stroke: u32,
        pub row_stroke: u32,
    }

    impl<T, const R: usize, const C: usize> Default for TableRenderable<T, R, C> {
        fn default() -> Self {
            Self {
                renderables: Vec::new(),
                origin: Point::new(0, 0),
                resolved_size: Dimensions::new(0, 0),
                width: 0,
                height: 0,
                col_widths: [0; C],
                row_heights: [0; R],
                col_stroke: 0,
                row_stroke: 0,
            }
        }
    }

    impl<const R: usize, const C: usize, V: AnimatedJoin> AnimatedJoin for TableRenderable<V, R, C> {
        fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
            self.renderables.join_from(&source.renderables, domain);
            self.col_stroke = u32::interpolate(source.col_stroke, self.col_stroke, domain.factor);
            self.row_stroke = u32::interpolate(source.row_stroke, self.row_stroke, domain.factor);
        }
    }

    impl<const R: usize, const C: usize, T> IntrinsicShape for TableRenderable<T, R, C> {
        fn content_shape(&self) -> ContentShape {
            ContentShape::Rectangle(Rectangle::new(
                self.origin,
                Size::new(self.resolved_size.width.0, self.resolved_size.height.0),
            ))
        }
    }

    impl<const R: usize, const C: usize, Color: Copy, T> Render<Color> for TableRenderable<T, R, C>
    where
        T: Clone + Render<Color>,
    {
        fn render(
            &self,
            render_target: &mut impl RenderTarget<ColorFormat = Color>,
            style: &Color,
        ) {
            let mut x = self.origin.x + self.col_widths[0] as i32;
            for c in 1..self.width {
                render_target.fill(
                    LinearTransform::default(),
                    &SolidBrush::new(*style),
                    None,
                    &Rectangle::new(
                        Point::new(x, self.origin.y),
                        Size::new(self.col_stroke, u32::from(self.resolved_size.height)),
                    ),
                );
                x += self.col_stroke as i32 + self.col_widths[c] as i32;
            }

            let mut y = self.origin.y + self.row_heights[0] as i32;
            for r in 1..self.height {
                render_target.fill(
                    LinearTransform::default(),
                    &SolidBrush::new(*style),
                    None,
                    &Rectangle::new(
                        Point::new(self.origin.x, y),
                        Size::new(u32::from(self.resolved_size.width), self.row_stroke),
                    ),
                );
                y += self.row_stroke as i32 + self.row_heights[r] as i32;
            }

            self.renderables.render(render_target, style);
        }

        fn render_animated(
            render_target: &mut impl RenderTarget<ColorFormat = Color>,
            source: &Self,
            target: &Self,
            style: &Color,
            domain: &AnimationDomain,
        ) {
            let mut joined_shape = target.clone();
            joined_shape.join_from(source, domain);
            joined_shape.render(render_target, style);
        }
    }
}

impl Move {
    const fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::KeyDown(key) => Self::from_key(*key),
            Event::Focus { action, group: _ } => match action {
                FocusAction::Next => Some(Self::Right),
                FocusAction::Previous => Some(Self::Left),
                FocusAction::Focus(f) => match f {
                    FocusDirection::Forward => Some(Self::Right),
                    FocusDirection::Backward => Some(Self::Left),
                },
                _ => None,
            },
            _ => None,
        }
    }

    const fn from_key(key: Key) -> Option<Self> {
        Some(match key {
            Key::UpArrow => Self::Up,
            Key::DownArrow => Self::Down,
            Key::LeftArrow => Self::Left,
            Key::RightArrow => Self::Right,
            _ => return None,
        })
    }

    fn focus_event(self, group: FocusGroup) -> Event {
        Event::Focus {
            action: FocusAction::Focus(match self {
                Self::Up | Self::Left => FocusDirection::Backward,
                Self::Down | Self::Right => FocusDirection::Forward,
            }),
            group,
        }
    }
}
