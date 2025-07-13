//! # Example: Espresso UI
//!
//! This example allows you to switch between three tabs using the left and right arrow keys.
//! The settings can be toggled using the `b`, `w`, and `o` keys.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use std::thread::sleep;
use std::time::{Duration, Instant};

use buoyant::app::App;
use buoyant::render_target::EmbeddedGraphicsRenderTarget;
use buoyant::{animation::Animation, if_view, match_view, view::prelude::*};

use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

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
    /// Font for body text
    pub const BODY: embedded_graphics::mono_font::MonoFont<'_> =
        embedded_graphics::mono_font::ascii::FONT_10X20;
    /// Font for captions, smaller text
    pub const CAPTION: embedded_graphics::mono_font::MonoFont<'_> =
        embedded_graphics::mono_font::ascii::FONT_9X15;
    /// Font for bold captions
    pub const CAPTION_BOLD: embedded_graphics::mono_font::MonoFont<'_> =
        embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
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
    let target = EmbeddedGraphicsRenderTarget::new(&mut display);
    let mut window = Window::new("Coffeeeee", &OutputSettings::default());
    let app_start = Instant::now();

    let captures = AppState::default();
    let mut app = App::new(root_view, target, captures, app_start.elapsed());

    loop {
        app.target.display_mut().clear(color::BACKGROUND).unwrap();
        app.render(app_start.elapsed(), &color::Space::WHITE);

        // Flush to window
        window.update(app.target.display());

        let should_exit = app.handle_events(
            app_start.elapsed(),
            window.events().filter_map(|event| event.try_into().ok()),
        );
        sleep(Duration::from_millis(15));

        if should_exit {
            break;
        }
    }
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
        tab_item("Brew", tab == Tab::Brew, |tab: &mut Tab| {
            *tab = Tab::Brew;
        }),
        tab_item("Clean", tab == Tab::Clean, |tab: &mut Tab| {
            *tab = Tab::Clean;
        }),
        tab_item("Settings", tab == Tab::Settings, |tab: &mut Tab| {
            *tab = Tab::Settings;
        }),
    ))
    .fixed_size(false, true)
    .animated(Animation::linear(Duration::from_millis(125)), tab)
}

fn tab_item<C, F: Fn(&mut C)>(
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
                    Text::new(name, &font::CAPTION_BOLD),
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
    VStack::new((
        Text::new("Good morning", &font::BODY),
        Text::new(
            "Use the arrow keys to navigate to the settings tab",
            &font::CAPTION_BOLD,
        )
        .multiline_text_alignment(HorizontalTextAlignment::Leading),
    ))
    .with_spacing(spacing::COMPONENT)
    .with_alignment(HorizontalAlignment::Leading)
    .flex_infinite_width(HorizontalAlignment::Leading)
    .padding(Edges::All, spacing::SECTION_MARGIN)
    .foreground_color(color::Space::WHITE)
}

fn settings_tab(state: &AppState) -> impl View<color::Space, AppState> {
    ScrollView::new(
        VStack::new((
            toggle_text(
                "Auto (b)rew",
                state.auto_brew,
                "Automatically brew coffee at 7am",
                true,
                |state: &mut AppState| {
                    state.auto_brew = !state.auto_brew;
                },
            ),
            toggle_text(
                "Stop on (w)eight",
                state.stop_on_weight,
                "Stop the machine automatically when the target weight is reached",
                false,
                |state: &mut AppState| {
                    state.stop_on_weight = !state.stop_on_weight;
                },
            ),
            Text::new(
                "This is a bunch of bullshit to make the view longer",
                &font::CAPTION,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Trailing)
            .foreground_color(color::Space::WHITE)
            .frame()
            .with_width(40),
            toggle_text(
                "Auto (o)ff",
                state.auto_off,
                "The display will go to sleep after 5 minutes of inactivity",
                true,
                |state: &mut AppState| {
                    state.auto_off = !state.auto_off;
                },
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Trailing)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .animated(Animation::linear(Duration::from_millis(200)), state.clone()),
    )
}

fn toggle_text<C>(
    label: &'static str,
    is_on: bool,
    description: &'static str,
    hides_description: bool,
    action: fn(&mut C),
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

fn toggle_button<C>(is_on: bool, on_tap: fn(&mut C)) -> impl View<color::Space, C> {
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
