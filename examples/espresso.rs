//! # Example: Espresso UI
//!
//! This example allows you to switch between three tabs using the left and right arrow keys.
//! The settings can be toggled using the `b`, `w`, and `o` keys.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

use std::time::{Duration, Instant};

use buoyant::{
    environment::DefaultEnvironment,
    layout::{HorizontalAlignment, Layout},
    match_view,
    primitives::ProposedDimensions,
    render::{AnimationDomain, EmbeddedGraphicsRender, Renderable},
    view::{
        padding::Edges,
        shape::{Circle, Rectangle},
        ConditionalView, HStack, HorizontalTextAlignment, LayoutExtensions as _, RenderExtensions,
        Text, VStack, ZStack,
    },
};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};

mod spacing {
    /// Outer padding to the edge of the screen
    pub const SECTION_MARGIN: u16 = 16;
    /// Spacing between distinct visual components in a section / group
    pub const COMPONENT: u16 = 16;
    /// Spacing between elements within a component
    pub const ELEMENT: u16 = 8;
}

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
    let mut display: SimulatorDisplay<color::Space> = SimulatorDisplay::new(Size::new(480, 320));
    let mut window = Window::new("Coffeeeee", &OutputSettings::default());
    let app_start = Instant::now();

    let mut app = App::default();

    let mut source_tree = app.tree(display.size(), app_start.elapsed());
    let mut target_tree = app.tree(display.size(), app_start.elapsed());

    'running: loop {
        // Create a new target tree if the state changes
        if app.reset_dirty() {
            source_tree = EmbeddedGraphicsRender::join(
                source_tree,
                target_tree,
                &AnimationDomain::top_level(app_start.elapsed()),
            );
            target_tree = app.tree(display.size(), app_start.elapsed());
        }

        display.clear(color::BACKGROUND).unwrap();

        // Render frame
        EmbeddedGraphicsRender::render_animated(
            &mut display,
            &source_tree,
            &target_tree,
            &color::Space::WHITE,
            buoyant::primitives::Point::zero(),
            &AnimationDomain::top_level(app_start.elapsed()),
        );

        // Flush to window
        window.update(&display);

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
            Tab::Brew => Tab::Clean,
            Tab::Clean => Tab::Settings,
            Tab::Settings => Tab::Brew,
        };
    }

    fn previous(&mut self) {
        *self = match self {
            Tab::Brew => Tab::Settings,
            Tab::Clean => Tab::Brew,
            Tab::Settings => Tab::Clean,
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
    ) -> impl EmbeddedGraphicsRender<color::Space> {
        let env = DefaultEnvironment::new(app_time);
        let view = Self::view(&self.state);
        let layout = view.layout(&dimensions.into(), &env);
        view.render_tree(&layout, buoyant::primitives::Point::zero(), &env)
    }

    fn view(
        state: &AppState,
    ) -> impl buoyant::render::Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>>
    {
        VStack::new((
            Self::tab_bar(state.tab),
            match_view!(state.tab => {
                Tab::Brew => {
                    Self::brew_tab(state)
                },
                Tab::Clean => {
                    Text::str("Clean", &font::BODY).foreground_color(color::Space::CSS_ORANGE_RED)
                    .padding(Edges::All, spacing::SECTION_MARGIN)
                },
                Tab::Settings => {
                    Self::settings_tab(state)
                },
            }),
        ))
    }

    fn tab_bar(
        tab: Tab,
    ) -> impl buoyant::render::Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>>
    {
        HStack::new((
            Self::tab_item("Brew", tab == Tab::Brew),
            Self::tab_item("Clean", tab == Tab::Clean),
            Self::tab_item("Settings", tab == Tab::Settings),
        ))
        .fixed_size(false, true)
        .animated(buoyant::Animation::Linear(Duration::from_millis(125)), tab)
    }

    fn tab_item(
        name: &str,
        is_selected: bool,
    ) -> impl buoyant::render::Renderable<
        color::Space,
        Renderables: EmbeddedGraphicsRender<color::Space>,
    > + use<'_> {
        let (text_color, bar_height) = if is_selected {
            (color::ACCENT, 4)
        } else {
            (color::FOREGROUND_SECONDARY, 0)
        };

        VStack::new((
            ZStack::new((
                ConditionalView::if_view(
                    is_selected,
                    Rectangle.foreground_color(color::BACKGROUND_SECONDARY),
                ),
                VStack::new((
                    Circle.frame().with_width(15),
                    Text::str(name, &font::CAPTION_BOLD),
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

    fn brew_tab(
        _state: &AppState,
    ) -> impl buoyant::render::Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>>
    {
        VStack::new((
            Text::str("Good morning", &font::BODY),
            Text::str(
                "Use the arrow keys to navigate to the settings tab",
                &font::CAPTION_BOLD,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Leading),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Leading)
        .flex_frame()
        .with_infinite_max_width()
        .with_horizontal_alignment(HorizontalAlignment::Leading)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Space::WHITE)
    }

    fn settings_tab(
        state: &AppState,
    ) -> impl buoyant::render::Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>>
    {
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
        .animated(
            buoyant::Animation::Linear(Duration::from_millis(200)),
            state.clone(),
        )
    }
}

fn toggle_text<'a>(
    label: &'a str,
    is_on: bool,
    description: &'a str,
    hides_description: bool,
) -> impl Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>> + use<'a> {
    VStack::new((
        HStack::new((
            Text::str(label, &font::BODY).foreground_color(color::Space::WHITE),
            toggle_button(is_on),
        ))
        .with_spacing(spacing::ELEMENT),
        ConditionalView::if_view(
            is_on || !hides_description,
            Text::str(description, &font::CAPTION)
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .foreground_color(color::Space::WHITE),
        ),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Trailing)
    .flex_frame()
    .with_infinite_max_width()
    .with_horizontal_alignment(HorizontalAlignment::Trailing)
}

fn toggle_button(
    is_on: bool,
) -> impl Renderable<color::Space, Renderables: EmbeddedGraphicsRender<color::Space>> {
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
            .animated(
                buoyant::Animation::Linear(Duration::from_millis(125)),
                is_on,
            ),
    ))
    .with_horizontal_alignment(alignment)
    .frame()
    .with_width(50)
    .with_height(25)
    .geometry_group()
}
