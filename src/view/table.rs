#![allow(clippy::needless_range_loop)]

use core::array;
use core::cmp::max;

use crate::environment::LayoutEnvironment;
use crate::layout::{HorizontalAlignment, LayoutDirection, ResolvedLayout, VerticalAlignment};
use crate::primitives::{Dimension, Dimensions, Point, ProposedDimension, ProposedDimensions};
use crate::render::TableRenderable;
use crate::transition::Opacity;
use crate::view::{ViewLayout, ViewMarker};

use heapless::Vec;
use kinda_array::Array;

/// A trait for indexing into a table-like structure
pub trait TableIndex<'a> {
    /// The output type when indexing into the table
    type Output: 'a;
    /// It is guaranteed  that it will only be called within bounds of the table, not as a safety guarantee.
    fn index(&self, x: usize, y: usize) -> Self::Output;
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

/**
A table view that allows creating table with different layouts, borders and flexible cell contents.

# Examples

Basic table with custom data:

```rust
use buoyant::view::{
    prelude::*,
    Table, TableAlgorithm, TableIndex
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
    fn index(&self, x: usize, y: usize) -> Self::Output {
        let key = &self.keys[y * self.width + x];
        let value = &self.values[y * self.width + x];
        (key, value)
    }
}

fn table_with_borders<'a>(items: Items<'a>) -> impl View<Rgb565, ()> + 'a {
    Table::<6, 6>::new(items.width, items.height, items, |(key, value)| {
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

/// A view of the [`Table`].
#[derive(Debug, Clone)]
pub struct TableView<'a, const R: usize, const C: usize, M, V, F>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
{
    items: M,
    build_view: F,
    width: usize,
    height: usize,
    algorithm: TableAlgorithm,
    row_stroke: u32,
    col_stroke: u32,
    cell_alignment: CellAlignment,
    _marker: core::marker::PhantomData<&'a ()>,
}

/// State of a table view with layout information.
#[derive(Debug, Clone)]
pub struct TableState<VState, const R: usize, const C: usize> {
    pub cell_states: [[VState; R]; C],
    pub col_widths: [u32; C],
    pub row_heights: [u32; R],
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
    pub fn new<'a, M, V, F>(
        mut width: usize,
        mut height: usize,
        items: M,
        build_view: F,
    ) -> TableView<'a, R, C, M, V, F>
    where
        M: TableIndex<'a>,
        F: Fn(M::Output) -> V,
    {
        width = width.min(C);
        height = height.min(R);

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

impl<'a, const R: usize, const C: usize, M, V, F> TableView<'a, R, C, M, V, F>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
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

impl<'a, const R: usize, const C: usize, M, V, F> ViewMarker for TableView<'a, R, C, M, V, F>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    V: ViewMarker,
{
    type Renderables = TableRenderable<V::Renderables, R, C>;
    type Transition = Opacity;
}

impl<'a, const R: usize, const C: usize, M, V, F, Captures> ViewLayout<Captures>
    for TableView<'a, R, C, M, V, F>
where
    M: TableIndex<'a>,
    F: Fn(M::Output) -> V,
    V: ViewLayout<Captures>,
{
    type Sublayout = Array<C, Array<R, ResolvedLayout<V::Sublayout>>>;
    type State = TableState<V::State, R, C>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        let mut state: Self::State = TableState::default();
        for c in 0..self.width {
            for r in 0..self.height {
                state.cell_states[c][r] =
                    (self.build_view)(self.items.index(c, r)).build_state(captures);
            }
        }
        state
    }

    #[allow(clippy::too_many_lines)]
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let mut sublayouts: Array<C, Array<R, ResolvedLayout<V::Sublayout>>> = Array::default();

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

        match self.algorithm {
            TableAlgorithm::FixedBoth => {
                // require exact width and height
                let ProposedDimension::Exact(total_width) = offer.width else {
                    panic!("Table algorithm `FixedBoth` requires an exact width offer")
                };
                let ProposedDimension::Exact(total_height) = offer.height else {
                    panic!("Table algorithm `FixedBoth` requires an exact height offer")
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

                ResolvedLayout {
                    sublayouts,
                    resolved_size: Dimensions {
                        width: Dimension::from(total_width),
                        height: Dimension::from(total_height),
                    },
                }
            }
            TableAlgorithm::FixedWidth => {
                // require exact width and height
                let ProposedDimension::Exact(total_width) = offer.width else {
                    panic!("Table algorithm `FixedBoth` requires an exact width offer")
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
                        &mut |r: usize, offered: ProposedDimensions| {
                            let dims = layout_fn(c, r, offered);
                            state.row_heights[r] = max(state.row_heights[r], dims.height.into());
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

                ResolvedLayout {
                    sublayouts,
                    resolved_size: Dimensions {
                        width: Dimension::from(total_width),
                        height: Dimension::from(total_height),
                    },
                }
            }

            /*

            For general case table layout seems to require a [convergence algorithm].
            HTML's `table-layout: auto;` is cool but isn't specified. Non-normative part of
            the [w3c spec] talks about determining maximing minimums and maximums for each column,
            but it doesn't seem to highlight how to distribute the space between columns.

            So this is like a "single convergence step". `layout_n` is used to get plausible
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
                    &mut |c: usize, offered: ProposedDimensions| {
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
                        state.col_widths[c] = dims.width.into();
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
                        &mut |r: usize, offered: ProposedDimensions| {
                            let dims = layout_fn(c, r, offered);
                            state.row_heights[r] = max(state.row_heights[r], dims.height.into());
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

                ResolvedLayout {
                    sublayouts,
                    resolved_size: Dimensions {
                        width: Dimension::from(total_width),
                        height: Dimension::from(total_height),
                    },
                }
            }
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
        let mut renderables: Vec<Vec<V::Renderables, C>, R> = Vec::new();
        let mut y = origin.y;

        for r in 0..self.height {
            let mut row = Vec::new();
            let mut x = origin.x;
            let cell_h = state.row_heights[r];

            for c in 0..self.width {
                let cell_w = state.col_widths[c];
                let dims = Dimensions {
                    width: Dimension::from(cell_w),
                    height: Dimension::from(cell_h),
                };

                let sublayout = &layout.sublayouts[c][r];
                let offset = self.cell_alignment.align(dims, sublayout.resolved_size);
                let cell_origin = offset + Point::new(x, y);

                let view = (self.build_view)(self.items.index(c, r));
                let res = row.push(view.render_tree(
                    sublayout,
                    cell_origin,
                    env,
                    captures,
                    &mut state.cell_states[c][r],
                ));
                assert!(res.is_ok());

                x += cell_w as i32 + self.col_stroke as i32;
            }
            y += cell_h as i32 + self.row_stroke as i32;
            _ = renderables.push(row);
        }

        TableRenderable {
            renderables,
            origin,
            resolved_size: layout.resolved_size,
            width: self.width,
            height: self.height,
            col_widths: state.col_widths,
            row_heights: state.row_heights,
            col_stroke: self.col_stroke,
            row_stroke: self.row_stroke,
        }
    }
}

// We can't use plain arrays there due to https://github.com/rust-lang/rust/issues/61415
mod kinda_array {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Copy)]
    pub struct Array<const N: usize, T>(pub [T; N]);

    impl<const N: usize, T> Default for Array<N, T>
    where
        T: Default + Clone,
    {
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
}

#[allow(clippy::too_many_lines)]
fn layout_n(
    subviews: &[(i8, bool)],
    subviews_indices_alloc: &mut [usize],
    flexibilities_alloc: &mut [Dimension],
    direction: LayoutDirection,
    offer: ProposedDimensions,
    stroke: u32,
    layout_fn: &mut dyn FnMut(usize, ProposedDimensions) -> Dimensions,
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
            let dimensions = layout_fn(i, offer);
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
        let minimum_dimension = layout_fn(index, min_proposal);
        if subviews[index].1 {
            num_empty_views += 1;
            continue;
        }
        let maximum_dimension = layout_fn(index, max_proposal);
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
