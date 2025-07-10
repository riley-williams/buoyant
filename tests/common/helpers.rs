use std::time::Duration;

use buoyant::{
    environment::DefaultEnvironment,
    primitives::{Point, ProposedDimensions},
    view::View,
};

#[macro_export]
macro_rules! assert_str_grid_eq {
    ($expected:expr, $actual:expr $(,)?) => {{
        use std::fmt::Write as _;

        let expected = &$expected;
        let actual = &$actual;

        // Check that dimensions match
        if expected.len() != actual.len() {
            panic!(
                "View height mismatch: expected {} rows, got {} rows",
                expected.len(),
                actual.len()
            );
        }

        // Find the maximum width of all rows
        let max_left_width = expected.iter().map(|s| s.len()).max().unwrap_or(0);
        let max_right_width = actual.iter().map(|row| row.len()).max().unwrap_or(0);

        // Both sides should have the same width
        if max_left_width != max_right_width {
            panic!(
                "View width mismatch: expected {} columns, got {} columns",
                max_left_width, max_right_width
            );
        }

        let cell_width = max_left_width;
        let mut has_differences = false;
        let mut visualization = String::new();

        _ = write!(visualization, "     {}\n", "-".repeat(cell_width * 2 + 3));

        for (exp_row, act_row) in expected.iter().zip(actual.iter()) {
            let act_str: String = act_row.iter().collect();

            let marker = if exp_row != &act_str {
                has_differences = true;
                ">"
            } else {
                " "
            };
            _ = write!(
                visualization,
                "    {}|{:<width$}|{:<width$}|\n",
                marker,
                exp_row,
                act_str,
                width = cell_width
            );
        }

        _ = write!(visualization, "     {}\n", "-".repeat(cell_width * 2 + 3));

        if has_differences {
            panic!("View comparison failed:\n\n{}", visualization);
        }
    }};
}

#[allow(dead_code)]
pub fn tree<V: View<char, Data>, Data: ?Sized>(
    view: &V,
    captures: &mut Data,
    state: &mut V::State,
    time: Duration,
    size: impl Into<ProposedDimensions>,
) -> V::Renderables {
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&size.into(), &env, captures, state);
    view.render_tree(&layout, Point::zero(), &env, captures, state)
}
