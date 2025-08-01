//! # Example: Espresso UI
//!
//! This example allows you to switch between three tabs using the left and right arrow keys.
//! The settings can be toggled using the `b`, `w`, and `o` keys.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use std::time::{Duration, Instant};

use buoyant::render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _};
use buoyant::view::scroll_view::ScrollDirection;
use buoyant::{animation::Animation, if_view, match_view, view::prelude::*};

use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use smol::lock::Mutex;

#[allow(unused)]
mod spacing {
    /// Spacing between sections / groups
    pub const SECTION: u32 = 24;
    /// Outer padding to the edge of the screen
    pub const SECTION_MARGIN: u32 = 16;
    /// Spacing between distinct visual components in a section / group
    pub const COMPONENT: u32 = 16;
    /// Spacing between elements within a component
    pub const ELEMENT: u32 = 8;
}

#[allow(unused)]
mod font {
    use u8g2_fonts::{fonts, FontRenderer};

    pub static MYSTERY_QUEST_28: FontRenderer =
        FontRenderer::new::<fonts::u8g2_font_mystery_quest_28_tr>();
    /// Font for body text
    pub static BODY: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fur14_tr>();
    pub static BODY_BOLD: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fur14_tr>();
    pub static HEADING: FontRenderer = FontRenderer::new::<fonts::u8g2_font_bubble_tr>();
    /// Font for captions, smaller text
    pub static CAPTION: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fur11_tr>();
    pub static CAPTION_BOLD: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub11_tr>();
}

#[allow(unused)]
mod color {
    use embedded_graphics::prelude::*;

    /// Use this alias instead of directly referring to a specific `embedded_graphics`
    /// color type to allow portability between displays
    pub type Space = embedded_graphics::pixelcolor::Rgb888;
    pub const ACCENT: Space = Space::CSS_LIGHT_SKY_BLUE;
    pub const BACKGROUND: Space = Space::BLACK;
    pub const BACKGROUND_SECONDARY: Space = Space::CSS_DARK_SLATE_GRAY;
    pub const FOREGROUND_SECONDARY: Space = Space::CSS_LIGHT_SLATE_GRAY;
}

fn main() {
    let size = Size::new(480, 320);
    let mut display: SimulatorDisplay<color::Space> = SimulatorDisplay::new(size);
    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
    let window = Mutex::new(Window::new("Coffeeeee", &OutputSettings::default()));
    let app_start = Instant::now();

    let captures = AppState::default();

    let app_time = || app_start.elapsed();

    let view_fn = |app_data: &mut AppState, _, _| {
        root_view(app_data).background(
            Alignment::default(),
            Rectangle.foreground_color(color::BACKGROUND),
        )
    };
    let flush_fn = async |target: &mut EmbeddedGraphicsRenderTarget<_>, _, _| {
        window.lock().await.update(target.display());
        // clear buffer for next frame
        target.clear(color::Space::BLACK);
    };

    let render_loop = buoyant::render_loop::render_loop(
        &mut target,
        captures,
        color::Space::WHITE,
        app_time,
        view_fn,
        flush_fn,
        async |handler| {
            window
                .lock()
                .await
                .events()
                .filter_map(|event| event.try_into().ok())
                .for_each(|event| {
                    println!("handled {event:?}");
                    handler.handle_event(&event);
                });
        },
    );
    smol::block_on(render_loop);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Tab {
    #[default]
    Brew,
    Clean,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct AppState {
    pub tab: Tab,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
}

fn root_view(state: &AppState) -> impl View<color::Space, AppState> {
    VStack::new((
        Lens::new(tab_bar(state.tab), |state: &mut AppState| &mut state.tab),
        match_view!(state.tab => {
            Tab::Brew => {
                brew_tab(state)
            },
            Tab::Clean => {
                Text::new("Clean", &font::BODY)
                    .foreground_color(color::Space::CSS_ORANGE_RED)
                    .padding(Edges::All, spacing::SECTION_MARGIN)
            },
            Tab::Settings => {
                settings_tab(state)
            },
        }),
    ))
}

fn tab_bar(tab: Tab) -> impl View<color::Space, Tab> {
    HStack::new((
        tab_item("Brew", tab == Tab::Brew, |tab: &mut Seal<Tab>| {
            *tab.as_mut() = Tab::Brew;
        }),
        tab_item("Clean", tab == Tab::Clean, |tab: &mut Seal<Tab>| {
            *tab.as_mut() = Tab::Clean;
        }),
        tab_item("Settings", tab == Tab::Settings, |tab: &mut Seal<Tab>| {
            *tab.as_mut() = Tab::Settings;
        }),
    ))
    .fixed_size(false, true)
    .animated(Animation::linear(Duration::from_millis(125)), tab)
}

fn tab_item<C, F: Fn(&mut Seal<C>)>(
    name: &'static str,
    is_selected: bool,
    on_tap: F,
) -> impl View<color::Space, C> {
    let (text_color, bar_height) = if is_selected {
        (color::ACCENT, 4)
    } else {
        (color::FOREGROUND_SECONDARY, 0)
    };

    Button::new(on_tap, move |is_pressed: bool| {
        VStack::new((
            ZStack::new((
                if_view!((is_selected || is_pressed) {
                    Rectangle.foreground_color(color::BACKGROUND_SECONDARY)
                }),
                VStack::new((
                    Circle.frame().with_width(15),
                    Text::new(name, &font::CAPTION),
                ))
                .with_spacing(spacing::ELEMENT)
                .padding(Edges::All, spacing::ELEMENT),
            )),
            Rectangle.frame().with_height(bar_height),
        ))
        .foreground_color(text_color)
        .flex_frame()
        .with_min_width(100)
    })
}

fn brew_tab<C>(_state: &AppState) -> impl View<color::Space, C> {
    ScrollView::new(
        VStack::new((
            Text::new("Good morning", &font::HEADING),
            Text::new(
                "You can't brew coffee in a simulator, but you can pretend.",
                &font::MYSTERY_QUEST_28,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Center)
        .flex_infinite_width(HorizontalAlignment::Center)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Space::WHITE),
    )
    .with_direction(ScrollDirection::Both)
}

fn settings_tab(state: &AppState) -> impl View<color::Space, AppState> {
    ScrollView::new(
        VStack::new((
            toggle_text(
                "Auto brew",
                state.auto_brew,
                "Automatically brew coffee at 7am",
                true,
                |state: &mut Seal<AppState>| {
                    state.auto_brew = !state.auto_brew;
                },
            ),
            toggle_text(
                "Stop on weight",
                state.stop_on_weight,
                "Stop the machine automatically when the target weight is reached",
                false,
                |state: &mut Seal<AppState>| {
                    state.stop_on_weight = !state.stop_on_weight;
                },
            ),
            toggle_text(
                "Auto off",
                state.auto_off,
                "The display will go to sleep after 5 minutes of inactivity",
                true,
                |state: &mut Seal<AppState>| {
                    state.auto_off = !state.auto_off;
                },
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Trailing)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .animated(Animation::linear(Duration::from_millis(200)), state.clone()),
    )
    .with_overlapping_bar(true) // we already applied padding
}

fn toggle_text<C>(
    label: &'static str,
    is_on: bool,
    description: &'static str,
    hides_description: bool,
    action: fn(&mut Seal<C>),
) -> impl View<color::Space, C> {
    VStack::new((
        HStack::new((
            Text::new(label, &font::BODY).foreground_color(color::Space::WHITE),
            toggle_button(is_on, action),
        ))
        .with_spacing(spacing::ELEMENT),
        if_view!((is_on || !hides_description) {
            Text::new(description, &font::CAPTION)
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .foreground_color(color::Space::WHITE)
        }),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Trailing)
    .flex_infinite_width(HorizontalAlignment::Trailing)
}

fn toggle_button<C>(is_on: bool, on_tap: fn(&mut Seal<C>)) -> impl View<color::Space, C> {
    let (color, alignment) = if is_on {
        (color::ACCENT, HorizontalAlignment::Trailing)
    } else {
        (color::Space::CSS_LIGHT_GRAY, HorizontalAlignment::Leading)
    };

    Button::new(on_tap, move |is_pressed: bool| {
        ZStack::new((
            buoyant::view::shape::Capsule.foreground_color(color),
            buoyant::view::shape::Circle
                .foreground_color(if is_pressed {
                    color::Space::CSS_LIGHT_GRAY
                } else {
                    color::Space::WHITE
                })
                .padding(Edges::All, 2)
                .animated(Animation::linear(Duration::from_millis(125)), is_on),
        ))
        .with_horizontal_alignment(alignment)
        .frame_sized(50, 25)
        .geometry_group()
    })
}
