use buoyant::{
    environment::DefaultEnvironment,
    primitives::{Dimensions, Point, ProposedDimensions, Size},
    render::Render,
    render_target::FixedTextBuffer,
    versioned::Generational,
    view::{
        chart::{
            mark::{BarMark, ChartMark, LineMark, PointMark},
            scale::{ChartScale, DataBounds},
            BarSeries, Chart, ChartContent, LineSeries, PointSeries,
        },
        prelude::*,
    },
};

mod common;
use common::make_render_tree;

// --- DataBounds tests ---

#[test]
fn data_bounds_from_marks() {
    let marks = [
        LineMark::new(1, 10),
        LineMark::new(5, 30),
        LineMark::new(3, 20),
    ];
    let bounds = DataBounds::from_marks(marks.iter().copied()).unwrap();
    assert_eq!(bounds, DataBounds::new(1, 5, 10, 30));
}

#[test]
fn data_bounds_from_empty_iterator() {
    let bounds = DataBounds::from_marks(core::iter::empty::<LineMark>());
    assert!(bounds.is_none());
}

#[test]
fn data_bounds_single_point() {
    let marks = [LineMark::new(5, 10)];
    let bounds = DataBounds::from_marks(marks.iter().copied()).unwrap();
    assert_eq!(bounds, DataBounds::new(5, 5, 10, 10));
}

#[test]
fn data_bounds_union() {
    let a = DataBounds::new(0, 10, 0, 20);
    let b = DataBounds::new(-5, 5, 10, 30);
    let u = a.union(&b);
    assert_eq!(u, DataBounds::new(-5, 10, 0, 30));
}

// --- ChartScale tests ---

#[test]
fn scale_maps_x_linearly() {
    let scale = ChartScale::new(
        DataBounds::new(0, 100, 0, 100),
        Point::new(10, 10),
        Size::new(200, 100),
    );
    assert_eq!(scale.map_x(0), 10);
    assert_eq!(scale.map_x(50), 110);
    assert_eq!(scale.map_x(100), 210);
}

#[test]
fn scale_maps_y_inverted() {
    let scale = ChartScale::new(
        DataBounds::new(0, 100, 0, 100),
        Point::new(0, 0),
        Size::new(100, 100),
    );
    assert_eq!(scale.map_y(0), 100);
    assert_eq!(scale.map_y(100), 0);
    assert_eq!(scale.map_y(50), 50);
}

#[test]
fn scale_centers_when_range_is_zero() {
    let scale = ChartScale::new(
        DataBounds::new(5, 5, 10, 10),
        Point::new(0, 0),
        Size::new(100, 80),
    );
    assert_eq!(scale.map_x(5), 50);
    assert_eq!(scale.map_y(10), 40);
}

#[test]
fn bar_width_calculation() {
    let scale = ChartScale::new(
        DataBounds::new(0, 4, 0, 10),
        Point::zero(),
        Size::new(100, 50),
    );
    assert_eq!(scale.bar_width(5, 2), 18);
    assert_eq!(scale.bar_width(0, 2), 0);
    assert_eq!(scale.bar_width(1, 2), 100);
}

// --- Plottable tests ---

#[test]
fn plottable_mark_conversions() {
    let m1 = LineMark::new(10i32, 20i32);
    assert_eq!(m1.x(), 10);
    assert_eq!(m1.y(), 20);

    let m2 = LineMark::new(5u8, 255u8);
    assert_eq!(m2.x(), 5);
    assert_eq!(m2.y(), 255);

    let m3 = LineMark::new(-100i16, 30000i16);
    assert_eq!(m3.x(), -100);
    assert_eq!(m3.y(), 30000);
}

// --- LineSeries tests ---

#[test]
fn line_series_data_bounds() {
    let data = [(1, 10), (2, 20), (3, 30)];
    let series = LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1));
    let bounds = series.data_bounds().unwrap();
    assert_eq!(bounds, DataBounds::new(1, 3, 10, 30));
}

#[test]
fn line_series_empty_data() {
    let data: [(i32, i32); 0] = [];
    let series = LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1));
    assert!(series.data_bounds().is_none());
}

#[test]
fn line_series_build_renderables() {
    let data = [(0, 0), (100, 100)];
    let series = LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1));
    let scale = ChartScale::new(
        DataBounds::new(0, 100, 0, 100),
        Point::new(0, 0),
        Size::new(100, 100),
    );
    let renderable = series.build_renderables(&scale);
    assert_eq!(renderable.points.len(), 2);
    // (0,0) → pixel (0, 100) because y is inverted
    assert_eq!(renderable.points[0], (0, 100));
    // (100,100) → pixel (100, 0)
    assert_eq!(renderable.points[1], (100, 0));
}

// --- BarSeries tests ---

#[test]
fn bar_series_data_bounds_extends_to_zero() {
    let data = [(1, 10), (2, 20)];
    let series = BarSeries::<10, _, _>::new(&data, |p: &(i32, i32)| BarMark::new(p.0, p.1));
    let bounds = series.data_bounds().unwrap();
    assert_eq!(bounds.y_min, 0);
    assert_eq!(bounds.y_max, 20);
}

#[test]
fn bar_series_build_renderables() {
    let data = [(0, 50)];
    let series = BarSeries::<10, _, _>::new(&data, |p: &(i32, i32)| BarMark::new(p.0, p.1));
    let scale = ChartScale::new(
        DataBounds::new(0, 0, 0, 50),
        Point::new(0, 0),
        Size::new(100, 100),
    );
    let renderable = series.build_renderables(&scale);
    assert_eq!(renderable.bars.len(), 1);
    assert!(renderable.bars[0].height > 0);
}

// --- PointSeries tests ---

#[test]
fn point_series_data_bounds() {
    let data = [(5, 15), (10, 25)];
    let series = PointSeries::<10, _, _>::new(&data, |p: &(i32, i32)| PointMark::new(p.0, p.1));
    let bounds = series.data_bounds().unwrap();
    assert_eq!(bounds, DataBounds::new(5, 10, 15, 25));
}

#[test]
fn point_series_build_renderables() {
    let data = [(0, 0), (100, 100)];
    let series = PointSeries::<10, _, _>::new(&data, |p: &(i32, i32)| PointMark::new(p.0, p.1));
    let scale = ChartScale::new(
        DataBounds::new(0, 100, 0, 100),
        Point::new(0, 0),
        Size::new(100, 100),
    );
    let renderable = series.build_renderables(&scale);
    assert_eq!(renderable.points.len(), 2);
    assert_eq!(renderable.points[0], (0, 100));
    assert_eq!(renderable.points[1], (100, 0));
}

// --- Multi-series tuple tests ---

#[test]
fn tuple_data_bounds_union() {
    let line_data = [(0, 10), (5, 20)];
    let point_data = [(3, 5), (8, 30)];
    let content = (
        LineSeries::<10, _, _>::new(&line_data, |p: &(i32, i32)| LineMark::new(p.0, p.1)),
        PointSeries::<10, _, _>::new(&point_data, |p: &(i32, i32)| PointMark::new(p.0, p.1)),
    );
    let bounds = content.data_bounds().unwrap();
    assert_eq!(bounds, DataBounds::new(0, 8, 5, 30));
}

// --- Chart view tests ---

#[test]
fn chart_greedy_layout() {
    let data = [(0, 10), (5, 20)];
    let chart = Chart::new(LineSeries::<10, _, _>::new(
        &data,
        |p: &(i32, i32)| LineMark::new(p.0, p.1),
    ));
    let env = DefaultEnvironment::non_animated();
    let offer = Size::new(200, 100);
    let mut state = chart.build_state(&mut ());
    let layout = chart.layout(&offer.into(), &env, &mut (), &mut state);
    assert_eq!(layout.resolved_size, Dimensions::new(200, 100));
}

#[test]
fn chart_compact_offer_uses_default_size() {
    let data = [(0, 10)];
    let chart = Chart::new(LineSeries::<10, _, _>::new(
        &data,
        |p: &(i32, i32)| LineMark::new(p.0, p.1),
    ));
    let env = DefaultEnvironment::non_animated();
    let offer = ProposedDimensions::compact();
    let mut state = chart.build_state(&mut ());
    let layout = chart.layout(&offer, &env, &mut (), &mut state);
    assert_eq!(layout.resolved_size, Dimensions::new(100, 100));
}

#[test]
fn chart_render_tree_produces_correct_points() {
    let data = [(0, 0), (100, 100)];
    let chart = Chart::new(
        LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1))
            .with_color('x'),
    );
    let tree = make_render_tree(&chart, Size::new(100, 100), &mut ());
    assert_eq!(tree.subtree.points.len(), 2);
}

#[test]
fn chart_with_colored_series() {
    let data = [(0, 10), (5, 20)];
    let chart = Chart::new(
        LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1))
            .with_color('R'),
    );
    let tree = make_render_tree(&chart, Size::new(100, 100), &mut ());
    assert_eq!(tree.style, 'R');
    assert_eq!(tree.subtree.points.len(), 2);
}

#[test]
fn chart_multi_series() {
    let line_data = [(0, 10), (5, 20)];
    let bar_data = [(1, 15), (3, 25)];
    let chart = Chart::new((
        LineSeries::<10, _, _>::new(&line_data, |p: &(i32, i32)| LineMark::new(p.0, p.1)),
        BarSeries::<10, _, _>::new(&bar_data, |p: &(i32, i32)| BarMark::new(p.0, p.1)),
    ));
    let tree = make_render_tree::<char, _, _>(&chart, Size::new(200, 100), &mut ());
    assert_eq!(tree.0.points.len(), 2);
    assert_eq!(tree.1.bars.len(), 2);
}

#[test]
fn chart_empty_data_no_panic() {
    let data: [(i32, i32); 0] = [];
    let chart = Chart::new(LineSeries::<10, _, _>::new(
        &data,
        |p: &(i32, i32)| LineMark::new(p.0, p.1),
    ));
    let tree = make_render_tree::<char, _, _>(&chart, Size::new(100, 100), &mut ());
    assert_eq!(tree.points.len(), 0);
}

// --- AnimatedJoin tests ---

#[test]
fn line_renderable_animated_join() {
    use buoyant::render::AnimatedJoin;
    use buoyant::render::AnimationDomain;

    let data1 = [(0, 0), (50, 50)];
    let data2 = [(0, 0), (100, 100)];

    let scale = ChartScale::new(
        DataBounds::new(0, 100, 0, 100),
        Point::new(0, 0),
        Size::new(100, 100),
    );

    let series1 = LineSeries::<10, _, _>::new(&data1, |p: &(i32, i32)| LineMark::new(p.0, p.1));
    let series2 = LineSeries::<10, _, _>::new(&data2, |p: &(i32, i32)| LineMark::new(p.0, p.1));

    let source = series1.build_renderables(&scale);
    let mut target = series2.build_renderables(&scale);

    // At factor=0 (start), target should match source
    let domain = AnimationDomain::new(0, core::time::Duration::from_millis(100));
    target.join_from(&source, &domain);
    assert_eq!(target.points[0], source.points[0]);
    assert_eq!(target.points[1], source.points[1]);
}

// --- Versioned/Generational tests ---

#[test]
fn generational_tracks_mutations() {
    use buoyant::versioned::Versioned;

    let mut data_gen = Generational::new(vec![1, 2, 3]);
    let v1 = data_gen.version();

    // Reading doesn't change version
    let _ = data_gen.get();
    assert_eq!(data_gen.version(), v1);

    // Mutating increments version
    data_gen.get_mut().push(4);
    let v2 = data_gen.version();
    assert_ne!(v1, v2);

    // Setting changes version
    data_gen.set(vec![10, 20]);
    let v3 = data_gen.version();
    assert_ne!(v2, v3);
}

// --- Rendering tests ---

#[test]
fn chart_renders_to_buffer() {
    let data = [(1, 1), (3, 3)];
    let chart = Chart::new(
        PointSeries::<10, _, _>::new(&data, |p: &(i32, i32)| PointMark::new(p.0, p.1))
            .with_point_size(2)
            .with_color('*'),
    );
    let mut buffer = FixedTextBuffer::<10, 10>::default();
    let tree = make_render_tree(&chart, buffer.size(), &mut ());
    // Verify rendering completes without panic
    tree.render(&mut buffer, &' ');
    // Verify points were computed
    assert_eq!(tree.subtree.points.len(), 2);
}

#[test]
fn chart_line_renders_to_buffer() {
    let data = [(1, 1), (8, 8)];
    let chart = Chart::new(
        LineSeries::<10, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1))
            .with_line_width(1)
            .with_color('#'),
    );
    let mut buffer = FixedTextBuffer::<10, 10>::default();
    let tree = make_render_tree(&chart, buffer.size(), &mut ());
    // Verify rendering completes without panic
    tree.render(&mut buffer, &' ');
    // Verify line points were computed
    assert_eq!(tree.subtree.points.len(), 2);
}

// --- Series truncation tests ---

#[test]
fn series_truncates_to_capacity() {
    let data: Vec<(i32, i32)> = (0..20).map(|i| (i, i * 2)).collect();
    let series = LineSeries::<5, _, _>::new(&data, |p: &(i32, i32)| LineMark::new(p.0, p.1));
    let scale = ChartScale::new(
        DataBounds::new(0, 19, 0, 38),
        Point::zero(),
        Size::new(100, 100),
    );
    let renderable = series.build_renderables(&scale);
    assert_eq!(renderable.points.len(), 5);
}
