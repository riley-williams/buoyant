//! # Example: Espresso UI
//!
//! A demo UI showing a coffee machine interface with multiple tabs, buttons, and toggles.
//!
//! To run this example using the `embedded_graphics` simulator, you must have the `sdl2` package installed.
//! See [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for installation instructions.

mod view;

use std::time::{Duration, Instant};

use buoyant::environment::DefaultEnvironment;
use buoyant::event::{EventContext, EventResult, simulator::MouseTracker};
use buoyant::primitives::Point;
use buoyant::render::{AnimatedJoin, AnimationDomain, Render};
use buoyant::render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _};
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

    let mut app_data = AppState::default();
    let mut view = root_view(&app_data);
    let mut view_state = view.build_state(&mut app_data);

    // Create initial source and target trees for animation
    let time = app_start.elapsed();
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut view_state);

    let mut source_tree = &mut view.render_tree(
        &layout,
        Point::default(),
        &env,
        &mut app_data,
        &mut view_state,
    );
    let mut target_tree = &mut view.render_tree(
        &layout,
        Point::default(),
        &env,
        &mut app_data,
        &mut view_state,
    );

    // Main event loop
    loop {
        let time = app_start.elapsed();
        let domain = AnimationDomain::top_level(time);
        let context = EventContext::new(time);

        let mut should_exit = false;

        // Handle events, merging into a single result
        let result = window
            .events()
            .filter_map(|event| touch_tracker.process_event(event))
            .fold(EventResult::default(), |result, event| {
                // Manually handle exit events and external events
                if event == buoyant::event::Event::Exit {
                    should_exit = true;
                }
                result.merging(view.handle_event(
                    &event,
                    &context,
                    target_tree,
                    &mut app_data,
                    &mut view_state,
                ))
            });

        if should_exit {
            break;
        }

        // Only recompute the view, layout, and render trees if necessary.
        // Additional handling may be needed to recompute the view in response to external events.
        if result.recompute_view {
            // Join source and target trees at current time, "freezing" animation progress
            target_tree.join_from(source_tree, &domain);
            // Swap trees so the current target becomes the next source.
            // Note this swaps the references instead of the whole section of memory
            core::mem::swap(&mut source_tree, &mut target_tree);
            // Create new view and target tree
            view = root_view(&app_data);
            let env = DefaultEnvironment::new(time);
            let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut view_state);
            *target_tree = view.render_tree(
                &layout,
                Point::default(),
                &env,
                &mut app_data,
                &mut view_state,
            );
        }

        // Only render if active animation was reported, the view changed, or redraw was requested
        if target.clear_animation_status() || result.recompute_view || result.redraw {
            // Render animated transition between source and target trees
            Render::render_animated(
                &mut target,
                source_tree,
                target_tree,
                &color::Space::WHITE,
                &domain,
            );
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
struct AppState {
    pub tab: Tab,
    pub stop_on_weight: bool,
    pub auto_off: bool,
    pub auto_brew: bool,
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
                    Text::new(name, &*font::FONT).with_font_size(font::CAPTION_SIZE),
                ))
                .with_spacing(spacing::ELEMENT)
                .padding(Edges::All, spacing::ELEMENT)
                .hint_background_color(if is_selected || is_pressed {
                    color::BACKGROUND_SECONDARY
                } else {
                    color::BACKGROUND
                }),
            )),
            Rectangle.frame().with_height(bar_height),
        ))
        .foreground_color(text_color)
        .flex_frame()
        .with_min_width(100)
    })
}
