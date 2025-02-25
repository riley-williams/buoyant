use buoyant::font::CharacterBufferFont;
use buoyant::primitives::Point;
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget as _;
use buoyant::render::Renderable;
use buoyant::view::padding::Edges;
use buoyant::view::{make_render_tree, LayoutExtensions as _, RenderExtensions as _};
use buoyant::{
    layout::VerticalAlignment,
    render_target::CrosstermRenderTarget,
    view::{
        shape::Rectangle, Divider, HStack, HorizontalTextAlignment, Spacer, Text, VStack, ZStack,
    },
};
use crossterm::event::{read, Event};
use crossterm::style::Colors;

const FONT: CharacterBufferFont = CharacterBufferFont;

fn view() -> impl Renderable<Colors, Renderables: CharacterRender<Colors>> {
    VStack::new((
        HStack::new((
            Text::str(
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
            Text::str(
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
                Text::str(
                    "This is in a fixed size box",
                    &FONT,
                )
                    .frame().with_width(10).with_height(10),
            )),
            Text::str(
                "This is several lines of text.\nEach line is centered in the available space.\n The rectangle fills all the remaining verical space and aligns the content within it.\n2 points of padding are around this text",
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

    println!("View size {}", core::mem::size_of_val(&view));

    render_view(&mut target, &view);

    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize(_, _) => {
                render_view(&mut target, &view);
            }
            Event::Mouse(_) | Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
        }
    }
}

fn render_view(
    target: &mut CrosstermRenderTarget,
    view: &impl Renderable<Colors, Renderables: CharacterRender<Colors>>,
) {
    target.clear();
    let size = target.size();
    let tree = make_render_tree(view, size);
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
