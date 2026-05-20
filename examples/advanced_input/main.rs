//! A networking interface demonstrating focus-based navigation with multiple pages.
//!
//! The arrow keys and h/j/k/l can be used to navigate between focusable elements,
//! and space/enter can be used to select them. Pressing 'e' or backspace will
//! navigate back.
//!
//! Author: Oleksandr Babak (@Ddystopia)
//!

#![allow(clippy::match_same_arms)]

mod definitions;
mod hardware_input_input_line;
mod mock_data;
mod settings;
mod table;

use std::process::exit;
use std::time::{Duration, Instant};

use buoyant::{
    app::{App, Harness},
    event::{Event, Key, simulator::MouseTracker},
    focus::{self, BoundaryBehavior, FocusAction, Role},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget},
    view::prelude::*,
};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::{
    definitions::{GoodPixelColor, Page, PageAction, RenderData, State},
    mock_data::{PAGE_1, PAGE_2, SETTINGS},
};

const FONT: u8g2_fonts::FontRenderer =
    u8g2_fonts::FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_t0_13_tf>();

fn root_view(state: &State) -> impl View<Rgb888, State> + use<> {
    let page = state.page;
    let render_data = RenderData {
        palette: &PALETTE,
        page,
    };

    let save_settings = |app_state: &State| {
        println!("Saving settings");
        println!("  IP: {}", app_state.static_ip);
        println!("  Gateway: {}", app_state.gateway);
        println!("  Net Mask: {}", app_state.net_mask);
        println!("  DNS: {}", app_state.dns);
        println!("  DHCP: {}", app_state.dhcp);
    };

    view(render_data, state, save_settings)
}

pub fn view<'a, 'b, C: GoodPixelColor, F: Fn(&State) + 'a + Copy>(
    data: RenderData<'a, C>,
    state: &'b State,
    save_settings: F,
) -> impl View<C, State> + use<'a, C, F> {
    let paginate = move |s: &mut State, a: buoyant::view::paginate::PageEvent| {
        s.page_action = Some(match a {
            buoyant::view::paginate::PageEvent::Next => definitions::PageAction::Next,
            buoyant::view::paginate::PageEvent::Previous => definitions::PageAction::Prev,
        });
        // Close any open inputs when changing pages
        s.opened_input = None;
        s.opened_cell_input = None;
        s.focused_table = false;
    };

    let state = state.clone();
    let is_settings = matches!(data.page, Page::Settings { header, footer });

    buoyant::view::Paginate::new(focus::GROUP_1, is_settings, paginate, {
        buoyant::match_view!(data.page, {
            Page::IeTable {
                header,
                footer,
                names,
                ie,
                eu,
                table_dimensions: (r, c),
            } => VStack::new((
                hardware_input_input_line::hw_line(header, data.palette, false),
                table::table(data, &state, (r, c), names, ie, eu),
                hardware_input_input_line::hw_line(footer, data.palette, true),
            )).bound_focus(BoundaryBehavior::Stop),
            Page::Settings { header, footer } => VStack::new((
                hardware_input_input_line::hw_line(header, data.palette, false),
                settings::settings(data, &state, save_settings),
                hardware_input_input_line::hw_line(footer, data.palette, true),
            )).bound_focus(BoundaryBehavior::Wrap),
        })
        .background_color(data.palette.dark_blue(), Rectangle)
    })
    .focus_touches()
    .map_event::<(), _>(|event: &Event, _state| match event {
        Event::KeyDown(key) => match key {
            Key::Character('h') | Key::LeftArrow => {
                Some(FocusAction::Previous.into_event(focus::GROUP_1))
            }
            Key::Character('l') | Key::RightArrow => {
                Some(FocusAction::Next.into_event(focus::GROUP_1))
            }
            Key::Character('k') | Key::UpArrow => {
                Some(FocusAction::Previous.into_event(focus::GROUP_0))
            }
            Key::Character('j') | Key::DownArrow => {
                Some(FocusAction::Next.into_event(focus::GROUP_0))
            }
            Key::Character(' ' | '\n') => Some(FocusAction::Select.into_event(focus::GROUP_0)),
            Key::Character('e') | Key::Backspace => {
                Some(FocusAction::Blur.into_event(focus::GROUP_0))
            }
            // Ignore all other key down events, don't allow children to handle
            _ => None,
        },
        Event::KeyUp(_) => None,
        _ => Some(event.clone()),
    })
}

const PALETTE: definitions::Palette<Rgb888> = definitions::Palette::from_array([
    Rgb888::new(0x00, 0x00, 0x00),
    Rgb888::new(0x47, 0x47, 0xff),
    Rgb888::new(0x00, 0x00, 0x80),
    Rgb888::new(0x66, 0x66, 0x66),
    Rgb888::new(0x00, 0xbc, 0x10),
    Rgb888::new(0xd6, 0xd6, 0xd6),
    Rgb888::new(0xe3, 0x87, 0x0e),
    Rgb888::new(0xd1, 0x00, 0x00),
    Rgb888::new(0xff, 0xff, 0xff),
    Rgb888::new(0xe8, 0xf0, 0x00),
    Rgb888::new(0x9b, 0x30, 0xff),
]);

fn main() {
    let size = Size::new(320, 240);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size);
    let mut target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, PALETTE.black());
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut window = Window::new("Coffeeeee", &output_settings);
    // Send at least one update to the window so it doesn't panic when fetching events
    window.update(target.display());

    let app_start = Instant::now();
    let mut touch_tracker = MouseTracker::new();

    let initial_state = State {
        static_ip: core::net::Ipv4Addr::new(192, 168, 11, 100),
        gateway: core::net::Ipv4Addr::new(192, 168, 11, 1),
        dns: core::net::Ipv4Addr::new(192, 168, 11, 137),
        dhcp: true,
        net_mask: 24,
        page: SETTINGS,
        ..Default::default()
    };

    // Create app with view lifecycle management
    let mut app =
        App::new(initial_state, size.into(), root_view).with_roles(Role::Button | Role::Container);

    // Acquire initial focus
    app.focus_forward();

    // Main event loop
    loop {
        // Sync app time with real wall clock time
        app.set_time(app_start.elapsed());

        // Collect and process simulator events
        let events: Vec<_> = window
            .events()
            .filter_map(|event| {
                if event == SimulatorEvent::Quit {
                    exit(0);
                }
                touch_tracker.process_event(event)
            })
            .collect();

        for event in events {
            app.send(event);
        }

        // Handle IE value updates
        if let Some((i, ie)) = app.state().ie_value_update {
            println!("IE value update: {i}: {ie}");
            app.state_mut().ie_value_update = None;
        }

        // Handle page changes
        if let Some(action) = app.state().page_action {
            let current_page = app.state().page;
            let new_page = match (action, current_page) {
                (PageAction::Next, Page::IeTable { .. }) if current_page == PAGE_2 => SETTINGS,
                (PageAction::Next, Page::IeTable { .. }) => PAGE_2,
                (PageAction::Next, Page::Settings { .. }) => PAGE_1,

                (PageAction::Prev, Page::Settings { .. }) => PAGE_2,
                (PageAction::Prev, Page::IeTable { .. }) if current_page == PAGE_1 => SETTINGS,
                (PageAction::Prev, Page::IeTable { .. }) => PAGE_1,
            };
            let mut state = app.state_mut();
            state.page = new_page;
            state.page_action = None;
        }

        // Only render if active animation was reported or redraw needed
        if app.should_redraw() || target.clear_animation_status() {
            // Render animated transition between source and target trees
            app.render_animated(&mut target, &PALETTE.white());

            // Draw focus overlay
            if std::env::var("DEBUG_FOCUS").is_ok() {
                app.draw_focus_overlay(&mut target, PALETTE.yellow(), 2);
            }

            // Send to the display
            window.update(target.display());
            // Clear for the next frame
            target.clear(PALETTE.black());
        } else {
            // limit polling for updates to ~30 fps when idle
            std::thread::sleep(Duration::from_millis(33));
        }
    }
}
