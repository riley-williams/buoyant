//! # Example: Espresso UI
//!
//! A demo UI showing a coffee machine interface with multiple tabs, buttons, and toggles.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

mod view;

use std::process::exit;
use std::time::{Duration, Instant};

use buoyant::app::{App, Harness};
use buoyant::event::{Event, Key, simulator::MouseTracker};
use buoyant::focus::{BoundaryBehavior, FocusAction, Role};
use buoyant::render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _};
use buoyant::{animation::Animation, match_view, view::prelude::*};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

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
    use crate::color;
    use std::sync::LazyLock;

    use embedded_graphics::prelude::RgbColor as _;
    use embedded_ttf::{FontTextStyle, FontTextStyleBuilder};
    use u8g2_fonts::{FontRenderer, fonts};

    pub static MYSTERY_QUEST_28: FontRenderer =
        FontRenderer::new::<fonts::u8g2_font_mystery_quest_28_tr>();

    pub static FONT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
        let bytes = include_bytes!("fonts/LeagueMono-Regular.otf");
        rusttype::Font::try_from_bytes(bytes).unwrap()
    });

    pub static FONT_BOLD: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
        let bytes = include_bytes!("fonts/LeagueMono-Bold.otf");
        rusttype::Font::try_from_bytes(bytes).unwrap()
    });

    pub static CAPTION_SIZE: u32 = 14;
    pub static BODY_SIZE: u32 = 20;
    pub static HEADING_SIZE: u32 = 32;
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
    let mut target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, color::BACKGROUND);
    let mut window = Window::new("Coffeeeee", &OutputSettings::default());
    // Send at least one update to the window so it doesn't panic when fetching events
    window.update(target.display());

    let app_start = Instant::now();
    let mut touch_tracker = MouseTracker::new();

    // Create app with view lifecycle management
    let mut app = App::new(AppState::default(), size.into(), root_view)
        .with_roles(Role::Button | Role::Container);

    // Acquire initial focus
    app.focus_forward();

    // Main event loop
    loop {
        // Sync app time with real wall clock time
        app.set_time(app_start.elapsed());

        // Collect and process simulator events
        window
            .events()
            .filter_map(|event| {
                if event == SimulatorEvent::Quit {
                    exit(0);
                }
                touch_tracker.process_event(event)
            })
            .for_each(|event| {
                app.send(event);
            });

        // Only render if active animation was reported or redraw needed
        if app.should_redraw() || target.clear_animation_status() {
            // Render animated transition between source and target trees
            app.render_animated(&mut target, &color::Space::WHITE);

            // Draws a focus overlay
            // app.draw_focus_overlay(&mut target, color::Space::CSS_YELLOW, 1);

            // Send to the display
            window.update(target.display());
            // Clear for the next frame
            target.clear(color::Space::BLACK);
        } else {
            // limit polling for updates to ~30 fps when idle
            std::thread::sleep(Duration::from_millis(33));
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
struct CleanSettings {
    pub frequency: u32,
    pub time: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct AppState {
    pub tab: Tab,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
    pub clean_overlay: Option<CleanSettings>,
    pub clean_settings: CleanSettings,
}

fn root_view(state: &AppState) -> impl View<color::Space, AppState> + use<> {
    VStack::new((
        Lens::new(tab_bar(state.tab), |state: &mut AppState| &mut state.tab),
        match_view!(state.tab, {
            Tab::Brew => {
                view::brew_tab(state)
            },
            Tab::Clean => {
                view::clean_tab(state)
            },
            Tab::Settings => {
                view::settings_tab(state)
            },
        }),
    ))
    .popover(state.clean_overlay.as_ref(), view::clean::clean_overlay)
    .bound_focus(BoundaryBehavior::Wrap)
    .focus_touches()
    .map_event(|event: Event, _state| match event {
        Event::KeyDown(key) => match key {
            Key::UpArrow | Key::LeftArrow => Some(FocusAction::Previous.into()),
            Key::DownArrow | Key::RightArrow => Some(FocusAction::Next.into()),
            Key::Character(' ' | '\n') => Some(FocusAction::Select.into()),
            Key::Backspace | Key::Delete => Some(FocusAction::Blur.into()),
            _ => Some(event),
        },
        Event::KeyUp(_) => None, // Eat key up events
        _ => Some(event),
    })
}

fn tab_bar(tab: Tab) -> impl View<color::Space, Tab> + use<> {
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
) -> impl View<color::Space, C> + use<C, F> {
    Button::new(on_tap, move |s| {
        let (background, foreground) = match (is_selected, s.is_pressed() || s.is_focused()) {
            (true, true) => (color::BACKGROUND_SECONDARY, color::ACCENT),
            (true, false) => (color::BACKGROUND_SECONDARY, color::FOREGROUND_SECONDARY),
            (false, true) => (color::BACKGROUND, color::ACCENT),
            _ => (color::BACKGROUND, color::FOREGROUND_SECONDARY),
        };
        ZStack::new((
            Rectangle.foreground_color(background),
            VStack::new((
                Circle.frame().with_width(15),
                Text::new(name, &*font::FONT).with_font_size(font::CAPTION_SIZE),
            ))
            .with_spacing(spacing::ELEMENT)
            .padding(Edges::All, spacing::ELEMENT)
            .hint_background_color(background),
        ))
        .with_vertical_alignment(VerticalAlignment::Bottom)
        .foreground_color(foreground)
        .flex_frame()
        .with_min_width(100)
    })
}
