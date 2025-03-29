//! # Example: Espresso UI
//!
//! This example allows you to switch between three tabs using the left and right arrow keys.
//! The settings can be toggled using the `b`, `w`, and `o` keys.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use std::time::{Duration, Instant};

use buoyant::render_target::EmbeddedGraphicsRenderTarget;
use buoyant::view::View;
use buoyant::{
    animation::Animation,
    environment::DefaultEnvironment,
    if_view,
    layout::{HorizontalAlignment, Layout},
    match_view,
    primitives::ProposedDimensions,
    render::{AnimatedJoin, AnimationDomain, Render, Renderable as _},
    view::{
        padding::Edges,
        shape::{Circle, Rectangle},
        HStack, HorizontalTextAlignment, Text, VStack, ViewExt as _, ZStack,
    },
};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};

#[allow(unused)]
mod spacing {
    /// Spacing between sections / groups
    pub const SECTION: u16 = 24;
    /// Outer padding to the edge of the screen
    pub const SECTION_MARGIN: u16 = 16;
    /// Spacing between distinct visual components in a section / group
    pub const COMPONENT: u16 = 16;
    /// Spacing between elements within a component
    pub const ELEMENT: u16 = 8;
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
    let display: SimulatorDisplay<color::Space> = SimulatorDisplay::new(size);
    let mut target = EmbeddedGraphicsRenderTarget::new(display, color::BACKGROUND);
    let mut window = Window::new("Coffeeeee", &OutputSettings::default());
    let app_start = Instant::now();

    let mut app = App::default();

    let mut source_tree = app.tree(size, app_start.elapsed());
    let mut target_tree = app.tree(size, app_start.elapsed());

    'running: loop {
        // Create a new target tree if the state changes
        if app.reset_dirty() {
            source_tree = AnimatedJoin::join(
                source_tree,
                target_tree,
                &AnimationDomain::top_level(app_start.elapsed()),
            );
            target_tree = app.tree(size, app_start.elapsed());
        }

        target.target.clear(color::BACKGROUND).unwrap();

        // Render frame
        Render::render_animated(
            &mut target,
            &source_tree,
            &target_tree,
            &color::Space::WHITE,
            buoyant::primitives::Point::zero(),
            &AnimationDomain::top_level(app_start.elapsed()),
        );

        // Flush to window
        window.update(&target.target);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Left => app.state_mut().tab.previous(),
                    Keycode::Right => app.state_mut().tab.next(),
                    Keycode::B => app.state_mut().auto_brew = !app.state.auto_brew,
                    Keycode::W => app.state_mut().stop_on_weight = !app.state.stop_on_weight,
                    Keycode::O => app.state_mut().auto_off = !app.state.auto_off,
                    _ => {}
                },
                _ => {}
            }
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

impl Tab {
    fn next(&mut self) {
        *self = match self {
            Self::Brew => Self::Clean,
            Self::Clean => Self::Settings,
            Self::Settings => Self::Brew,
        };
    }

    fn previous(&mut self) {
        *self = match self {
            Self::Brew => Self::Settings,
            Self::Clean => Self::Brew,
            Self::Settings => Self::Clean,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct AppState {
    pub tab: Tab,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct App {
    state: AppState,
    is_dirty: bool,
}

impl App {
    fn state_mut(&mut self) -> &mut AppState {
        self.is_dirty = true;
        &mut self.state
    }

    fn reset_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }

    fn tree(
        &self,
        dimensions: impl Into<ProposedDimensions>,
        app_time: Duration,
    ) -> impl Render<color::Space> {
        let env = DefaultEnvironment::new(app_time);
        let view = Self::view(&self.state);
        let layout = view.layout(&dimensions.into(), &env);
        view.render_tree(&layout, buoyant::primitives::Point::zero(), &env)
    }

    fn view(state: &AppState) -> impl View<color::Space> {
        VStack::new((
            Self::tab_bar(state.tab),
            match_view!(state.tab => {
                Tab::Brew => {
                    Self::brew_tab(state)
                },
                Tab::Clean => {
                    Text::new("Clean", &font::BODY).foreground_color(color::Space::CSS_ORANGE_RED)
                    .padding(Edges::All, spacing::SECTION_MARGIN)
                },
                Tab::Settings => {
                    Self::settings_tab(state)
                },
            }),
        ))
    }

    fn tab_bar(tab: Tab) -> impl View<color::Space> {
        HStack::new((
            Self::tab_item("Brew", tab == Tab::Brew),
            Self::tab_item("Clean", tab == Tab::Clean),
            Self::tab_item("Settings", tab == Tab::Settings),
        ))
        .fixed_size(false, true)
        .animated(Animation::linear(Duration::from_millis(125)), tab)
    }

    fn tab_item(name: &str, is_selected: bool) -> impl View<color::Space> + use<'_> {
        let (text_color, bar_height) = if is_selected {
            (color::ACCENT, 4)
        } else {
            (color::FOREGROUND_SECONDARY, 0)
        };

        VStack::new((
            ZStack::new((
                if_view!((is_selected) {
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
    }

    fn brew_tab(_state: &AppState) -> impl View<color::Space> {
        VStack::new((
            Text::new("Good morning", &font::BODY),
            Text::new(
                "Use the arrow keys to navigate to the settings tab",
                &font::CAPTION_BOLD,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Leading),
            Text::new("abcdefghijklmnopqrstuvwxyz", &font::CAPTION)
                .multiline_text_alignment(HorizontalTextAlignment::Leading),
            Text::new("abcdefghijklmnopqrstuvwxyz", &font::CAPTION_BOLD)
                .multiline_text_alignment(HorizontalTextAlignment::Leading),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Leading)
        .flex_infinite_width(HorizontalAlignment::Leading)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Space::WHITE)
    }

    fn settings_tab(state: &AppState) -> impl View<color::Space> {
        VStack::new((
            toggle_text(
                "Auto (b)rew",
                state.auto_brew,
                "Automatically brew coffee at 7am",
                true,
            ),
            toggle_text(
                "Stop on (w)eight",
                state.stop_on_weight,
                "Stop the machine automatically when the target weight is reached",
                false,
            ),
            toggle_text(
                "Auto (o)ff",
                state.auto_off,
                "The display will go to sleep after 5 minutes of inactivity",
                true,
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Trailing)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .animated(Animation::linear(Duration::from_millis(200)), state.clone())
    }
}

fn toggle_text<'a>(
    label: &'a str,
    is_on: bool,
    description: &'a str,
    hides_description: bool,
) -> impl View<color::Space> + use<'a> {
    VStack::new((
        HStack::new((
            Text::new(label, &font::BODY).foreground_color(color::Space::WHITE),
            toggle_button(is_on),
        ))
        .with_spacing(spacing::ELEMENT),
        if_view!(( is_on || !hides_description) {
            Text::new(description, &font::CAPTION)
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .foreground_color(color::Space::WHITE)
        }),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Trailing)
    .flex_infinite_width(HorizontalAlignment::Trailing)
}

fn toggle_button(is_on: bool) -> impl View<color::Space> {
    let (color, alignment) = if is_on {
        (color::ACCENT, HorizontalAlignment::Trailing)
    } else {
        (color::Space::CSS_LIGHT_GRAY, HorizontalAlignment::Leading)
    };

    ZStack::new((
        buoyant::view::shape::Capsule.foreground_color(color),
        buoyant::view::shape::Circle
            .foreground_color(color::Space::WHITE)
            .padding(Edges::All, 2)
            .animated(Animation::linear(Duration::from_millis(125)), is_on),
    ))
    .with_horizontal_alignment(alignment)
    .frame_sized(50, 25)
    .geometry_group()
}
