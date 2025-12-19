#![allow(clippy::match_same_arms)]

mod definitions;
mod hardware_input_input_line;
mod mock_data;
mod settings;
mod table;

use std::time::Instant;

use buoyant::{
    environment::DefaultEnvironment,
    event::{
        input::{self, Groups, Input},
        keyboard::KeyboardInput,
        // simulator::MouseTracker,
    },
    render::Render,
    render_target::EmbeddedGraphicsRenderTarget,
    view::{Pagination, PaginationAction, prelude::*},
};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::{
    definitions::{GoodPixelColor, Page, PageAction, RenderData, State},
    mock_data::{PAGE_1, PAGE_2, SETTINGS},
};

const FONT: u8g2_fonts::FontRenderer =
    u8g2_fonts::FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_t0_13_tf>();

const G0: Groups = Groups::from_mask(0b01);
const G1: Groups = Groups::from_mask(0b10);

pub fn view<'a, 'b, C: GoodPixelColor, F: Fn(&State) + 'a + Copy>(
    data: RenderData<'a, C>,
    state: &'b State,
    save_settings: F,
) -> impl View<C, State> + use<'a, C, F> {
    let paginate = move |a, s: &mut State| {
        s.page_action = Some(match a {
            PaginationAction::Next => definitions::PageAction::Next,
            PaginationAction::Previous => definitions::PageAction::Prev,
            PaginationAction::Enter => return,
            PaginationAction::Submit => return,
            PaginationAction::Escape => return,
        });
        _ = s.opened_input.take().map(|(_, d)| {
            data.input.blur(G1);
            d.into_guard(data.input);
        });
    };

    let state = state.clone();

    Pagination::new_horizontal::<_, _, State>(G0, paginate, move |_| {
        let state = state.clone();
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
                table::table(data, (r, c), names, ie, eu),
                hardware_input_input_line::hw_line(footer, data.palette, true),
            )),
            Page::Settings { header, footer } => VStack::new((
                hardware_input_input_line::hw_line(header, data.palette, false),
                settings::settings(data, &state, save_settings),
                hardware_input_input_line::hw_line(footer, data.palette, true),
            )),
        })
        .background(
            Alignment::Center,
            Rectangle.foreground_color(data.palette.dark_blue()),
        )
    })
    .reroute_navigation(true)
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
    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut window = Window::new("Coffeeeee", &output_settings);
    let app_start = Instant::now();
    // let mut touch_tracker = MouseTracker::new();

    let mut group_data_0 = input::GroupData::new();
    let mut group_data_1 = input::GroupData::new();
    let mut group_data_2 = input::GroupData::new();
    let keyboard = KeyboardInput::new();
    let mut input = Input::new();

    display.clear(PALETTE.black()).unwrap();

    let mut events = vec![];
    let mut app_state = definitions::State {
        static_ip: core::net::Ipv4Addr::new(192, 168, 11, 100),
        gateway: core::net::Ipv4Addr::new(192, 168, 11, 1),
        dns: core::net::Ipv4Addr::new(192, 168, 11, 137),
        dhcp: true,
        net_mask: 24,
        ..Default::default()
    };

    let [g0, g1, g2] = [0, 1, 2].map(input::Group::new);
    let g012 = Groups::from_iter([g0, g1, g2]);

    input.add_group(g0, &mut group_data_0).unwrap();
    input.add_group(g1, &mut group_data_1).unwrap();
    input.add_group(g2, &mut group_data_2).unwrap();

    input.add_keyboard(g012, &keyboard).unwrap();

    let mut page = mock_data::SETTINGS;

    let save_settings = |app_state: &definitions::State| {
        println!("Saving settings");
        println!("  IP: {}", app_state.static_ip);
        println!("  Gateway: {}", app_state.gateway);
        println!("  Net Mask: {}", app_state.net_mask);
        println!("  DNS: {}", app_state.dns);
        println!("  DHCP: {}", app_state.dhcp);
    };

    let mut state = {
        let render_data = RenderData {
            palette: &PALETTE,
            page,
            input: input.as_ref(),
        };
        let view = view(render_data, &app_state, save_settings);
        view.build_state(&mut app_state)
    };

    for _ in 0usize.. {
        display.clear(PALETTE.black()).unwrap();

        if let Some(action) = app_state.page_action.take() {
            match (action, page) {
                (PageAction::Next, Page::IeTable { .. }) if page == PAGE_2 => page = SETTINGS,
                (PageAction::Next, Page::IeTable { .. }) => page = PAGE_2,
                (PageAction::Next, Page::Settings { .. }) => page = PAGE_1,

                (PageAction::Prev, Page::Settings { .. }) => page = PAGE_2,
                (PageAction::Prev, Page::IeTable { .. }) if page == PAGE_1 => page = SETTINGS,
                (PageAction::Prev, Page::IeTable { .. }) => page = PAGE_1,
            }
        }

        let render_data = RenderData {
            palette: &PALETTE,
            page,
            input: input.as_ref(),
        };

        let view = view(render_data, &app_state, save_settings);
        let now = app_start.elapsed();
        let env = DefaultEnvironment::new(now).input(&input);
        let layout = view.layout(&display.size().into(), &env, &mut app_state, &mut state);
        let mut target_tree = view.render_tree(
            &layout,
            buoyant::primitives::Point::default(),
            &env,
            &mut app_state,
            &mut state,
        );

        let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
        Render::render(&target_tree, &mut target, &PALETTE.white());
        window.update(&display);

        for event in events.drain(..) {
            let context = buoyant::event::EventContext::new(now).input(&input);
            let _ = view.handle_event(
                &event,
                &context,
                &mut target_tree,
                &mut app_state,
                &mut state,
            );
        }

        std::thread::sleep(std::time::Duration::from_millis(50));

        let now = app_start.elapsed();
        for event in window.events() {
            if event == embedded_graphics_simulator::SimulatorEvent::Quit {
                return;
            }
            // if let Some(e) = touch_tracker.process_event(event) {
            //     events.push(e);
            // }
            if let Some(e) = decode_event(event, now, &input, &keyboard) {
                events.push(e);
            }
        }
    }
}

fn decode_event(
    sim_event: embedded_graphics_simulator::SimulatorEvent,
    now: std::time::Duration,
    input: &input::Input,
    keyboard: &KeyboardInput,
) -> Option<buoyant::event::Event> {
    use buoyant::event::keyboard::{ButtonState, Key};
    use embedded_graphics_simulator::{SimulatorEvent, sdl2};

    let pressed = ButtonState::Pressed;
    let released = ButtonState::Released;
    let key_state = match sim_event {
        SimulatorEvent::KeyDown { keycode, .. } => match keycode {
            sdl2::Keycode::H => Some((Key::Left, pressed)),
            sdl2::Keycode::K => Some((Key::Up, pressed)),
            sdl2::Keycode::J => Some((Key::Down, pressed)),
            sdl2::Keycode::L => Some((Key::Right, pressed)),
            sdl2::Keycode::RETURN => Some((Key::Enter, pressed)),
            sdl2::Keycode::E => Some((Key::Escape, pressed)),
            _ => None,
        },
        SimulatorEvent::KeyUp { keycode, .. } => match keycode {
            sdl2::Keycode::H => Some((Key::Left, released)),
            sdl2::Keycode::K => Some((Key::Up, released)),
            sdl2::Keycode::J => Some((Key::Down, released)),
            sdl2::Keycode::L => Some((Key::Right, released)),
            sdl2::Keycode::RETURN => Some((Key::Enter, released)),
            sdl2::Keycode::E => Some((Key::Escape, released)),
            _ => None,
        },
        _ => None,
    };

    if let Some((key, state)) = key_state
        && let Some(event) = input.keyboard_input(keyboard, key, state, now)
    {
        return Some(event);
    }
    None
}
