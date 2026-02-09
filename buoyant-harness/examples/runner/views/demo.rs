//! Multi-tab demo view with counter, toggle, and info tabs.

use std::time::Duration;

use buoyant::match_view;
use buoyant::view::prelude::*;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use serde::Serialize;

/// Colors for the demo views.
pub mod color {
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::{RgbColor, WebColors};

    pub const FOREGROUND: Rgb888 = Rgb888::WHITE;
    pub const ACCENT: Rgb888 = Rgb888::CSS_LIGHT_SKY_BLUE;
    pub const SECONDARY: Rgb888 = Rgb888::CSS_DIM_GRAY;
    pub const SUCCESS: Rgb888 = Rgb888::CSS_LIME_GREEN;
    pub const ERROR: Rgb888 = Rgb888::CSS_INDIAN_RED;
}

/// State for the full demo application with tabs.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DemoState {
    pub counter: i32,
    pub selected_tab: usize,
    pub toggle_enabled: bool,
}

/// Demo view with tabs, counter, and toggle.
pub fn demo_view(state: &DemoState) -> impl View<Rgb888, DemoState> + use<> {
    VStack::new((
        // Header
        Text::new("Workflow Demo", &FONT_10X20).foreground_color(color::ACCENT),
        // Tab bar
        HStack::new((
            tab_button("Counter", state.selected_tab == 0, |s: &mut DemoState| {
                s.selected_tab = 0;
            }),
            tab_button("Toggle", state.selected_tab == 1, |s: &mut DemoState| {
                s.selected_tab = 1;
            }),
            tab_button("Info", state.selected_tab == 2, |s: &mut DemoState| {
                s.selected_tab = 2;
            }),
        ))
        .with_spacing(8),
        // Content based on selected tab
        match_view!(state.selected_tab, {
            0 => counter_tab_content(state),
            1 => toggle_content(state),
            _ => info_content(),
        }),
        Spacer::default(),
    ))
    .with_spacing(16)
    .padding(Edges::All, 20)
}

fn tab_button<F: Fn(&mut DemoState)>(
    label: &'static str,
    is_selected: bool,
    on_tap: F,
) -> impl View<Rgb888, DemoState> + use<F> {
    let bg_color = if is_selected {
        color::ACCENT
    } else {
        color::SECONDARY
    };
    let text_color = if is_selected {
        Rgb888::BLACK
    } else {
        color::FOREGROUND
    };

    Button::new(on_tap, move |_button_state| {
        Text::new(label, &FONT_10X20)
            .foreground_color(text_color)
            .padding(Edges::Horizontal, 12)
            .padding(Edges::Vertical, 6)
            .background_color(bg_color, Rectangle)
    })
}

fn counter_tab_content(state: &DemoState) -> impl View<Rgb888, DemoState> + use<> {
    VStack::new((
        Text::new("Counter", &FONT_10X20),
        HStack::new((
            Button::new(
                |s: &mut DemoState| s.counter -= 1,
                |_| {
                    Text::new("-", &FONT_10X20)
                        .padding(Edges::All, 8)
                        .background_color(color::SECONDARY, Rectangle)
                },
            ),
            Text::new_fmt::<32>(format_args!("{}", state.counter), &FONT_10X20)
                .foreground_color(color::ACCENT),
            Button::new(
                |s: &mut DemoState| s.counter += 1,
                |_| {
                    Text::new("+", &FONT_10X20)
                        .padding(Edges::All, 8)
                        .background_color(color::SECONDARY, Rectangle)
                },
            ),
        ))
        .with_spacing(16),
    ))
    .with_spacing(12)
}

/// Toggle content for `DemoState`.
pub fn toggle_content(state: &DemoState) -> impl View<Rgb888, DemoState> + use<> {
    let status = if state.toggle_enabled { "ON" } else { "OFF" };
    let status_color = if state.toggle_enabled {
        color::SUCCESS
    } else {
        color::ERROR
    };

    VStack::new((
        Text::new("Feature Toggle", &FONT_10X20),
        HStack::new((
            Text::new("Status:", &FONT_10X20),
            Button::new(
                |s: &mut DemoState| s.toggle_enabled = !s.toggle_enabled,
                move |_| {
                    Text::new(status, &FONT_10X20)
                        .foreground_color(status_color)
                        .padding(Edges::All, 8)
                        .background_color(color::SECONDARY, Rectangle)
                },
            ),
        ))
        .with_spacing(12)
        .animated(
            Animation::linear(Duration::from_millis(500)),
            state.toggle_enabled,
        ),
    ))
    .with_spacing(12)
}

/// Info content panel.
pub fn info_content() -> impl View<Rgb888, DemoState> {
    VStack::new((
        Text::new("Info", &FONT_10X20),
        Text::new("This demonstrates the", &FONT_10X20).foreground_color(color::SECONDARY),
        Text::new("workflow harness.", &FONT_10X20).foreground_color(color::SECONDARY),
    ))
    .with_spacing(8)
}
