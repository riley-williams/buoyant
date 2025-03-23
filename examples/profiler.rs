use buoyant::primitives::Point;
use buoyant::render::{CharacterRender as _, CharacterRenderTarget};
use buoyant::view::padding::Edges;
use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::{Layout, VerticalAlignment},
    primitives::Size,
    render_target::FixedTextBuffer,
    view::{
        make_render_tree, Divider, HStack, HorizontalTextAlignment, Spacer, Text, VStack, ViewExt,
    },
};

fn main() {
    let mut target = FixedTextBuffer::<100, 100>::default();

    target.clear();
    let mut size = target.size();
    println!("Size {size:?}");

    let env = DefaultEnvironment::non_animated();

    let font = CharacterBufferFont {};
    let stack = VStack::new((
     HStack::new((
         Text::new(
             "This text is centered horizontally in the middle of its space\nThe stack however, has bottom alignment.",
             &font,
                 )
                 .multiline_text_alignment(HorizontalTextAlignment::Center),
         Divider::default().foreground_color('|'),
         Text::new(
             "This text is aligned to the right, with trailing multi-line text alignment",
             &font,
                 )
                 .multiline_text_alignment(HorizontalTextAlignment::Trailing),
         ))
         .with_spacing(1)
         .with_alignment(VerticalAlignment::Bottom),
     Divider::default().foreground_color('-'),
     VStack::new((
         Spacer::default(),
         Text::new(
             "This is several lines of text.\nEach line is centered in the available space.\n Spacers are used to fill all the remaining verical space and align the content within it.\n2 points of padding are around this text",
             &font,
         )
             .multiline_text_alignment(HorizontalTextAlignment::Center)
             .padding(Edges::All, 2),
         Divider::default().foreground_color('-'),
         )),
     ));

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));
    let sample_layout = stack.layout(&size.into(), &env);
    println!("Layout size {}", std::mem::size_of_val(&sample_layout));

    target.clear();
    for width in 1..100 {
        for height in 1..100 {
            size = Size::new(width, height);
            let tree = make_render_tree(&stack, size);
            tree.render(&mut target, &' ', Point::zero());
        }
    }
}
