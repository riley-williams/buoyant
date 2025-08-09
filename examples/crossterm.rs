use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::primitives::Point;
use buoyant::render::Render;
use buoyant::{render_target::CrosstermRenderTarget, view::prelude::*};
use crossterm::event::{read, Event};
use crossterm::style::Colors;

const FONT: CharacterBufferFont = CharacterBufferFont;

fn view() -> impl View<Colors, ()> {
    VStack::new((
        HStack::new((
            Text::new(
                "This red text is aligned to the leading edge of its space\nThe stack however, has bottom alignment.",
                &FONT,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Leading)
                .foreground_color(
                    Colors {
                        foreground: Some(crossterm::style::Color::Red),
                        background: None
                    },
                ),
            Spacer::default(),
            Text::new(
                "This text is aligned to the right, with trailing multi-line text alignment",
                &FONT,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .flex_frame().with_min_width(10).with_max_width(35),
        ))
            .with_spacing(1)
            .with_alignment(VerticalAlignment::Bottom),
        Divider::default(),
        VStack::new((
            ZStack::new((
                Rectangle
                    .foreground_color(
                        Colors {
                            foreground: Some(crossterm::style::Color::Rgb { r: 0, g: 255, b: 0 }),
                            background: Some(crossterm::style::Color::Rgb { r: 127, g: 0, b: 0 })
                        }
                    ),
                Text::new(
                    "This is in a fixed size box",
                    &FONT,
                )
                    .frame_sized(10, 10),
            )),
            Text::new(
                "This is several lines of text.\nEach line is centered in the available space.\n The rectangle fills all the remaining vertical space and aligns the content within it.\n2 points of padding are around this text",
                &FONT,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Center)
                .foreground_color(
                    Colors {
                        foreground: Some(crossterm::style::Color::Rgb { r: 255, g: 0, b: 255 }),
                        background: None
                    }
                )
                .padding(Edges::All, 2),
            Divider::default()
                .foreground_color(Colors {
                    foreground: Some(crossterm::style::Color::DarkYellow),
                    background: None
                })
        )),
    ))
}

fn main() {
    let mut target = CrosstermRenderTarget::default();

    target.enter_fullscreen();
    target.clear();
    let size = target.size();
    println!("Size {size:?}");

    let view = view();

    println!("View size {}", std::mem::size_of_val(&view));

    render_view(&mut target, &view);

    loop {
        // `read()` blocks until an `Event` is available
        #[allow(clippy::match_same_arms)]
        match read().unwrap() {
            Event::FocusGained => (),
            Event::FocusLost => (),
            Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Mouse(_) => (),
            Event::Resize(_, _) => {
                render_view(&mut target, &view);
            }
            Event::Paste(_) => (),
        }
    }
}

fn render_view(target: &mut CrosstermRenderTarget, view: &impl View<Colors, ()>) {
    target.clear();
    let size = target.size();
    let env = DefaultEnvironment::default();
    let mut state = view.build_state(&mut ());
    let layout = view.layout(&size.into(), &env, &mut (), &mut state);
    let tree = view.render_tree(&layout, Point::zero(), &env, &mut (), &mut state);
    tree.render(
        target,
        &Colors {
            foreground: Some(crossterm::style::Color::Rgb {
                r: 255,
                g: 0,
                b: 255,
            }),
            background: None,
        },
        Point::zero(),
    );
    target.flush();
}
