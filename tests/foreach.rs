use buoyant::{
    environment::DefaultEnvironment,
    font::BufferCharacterFont,
    layout::{Layout as _, VerticalAlignment},
    primitives::Point,
    render::CharacterRender,
    render_target::{CharacterRenderTarget, FixedTextBuffer},
    view::{Divider, ForEach, HStack, Spacer, Text},
};

static FONT: BufferCharacterFont = BufferCharacterFont {};

#[derive(Debug)]
struct User {
    name: String,
    age: String,
}

#[test]
fn foreach_with_inner_wrapping_hstack() {
    let mut users = heapless::Vec::<User, 10>::new();

    users
        .push(User {
            name: "Alice".to_string(),
            age: "99".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Bob".to_string(),
            age: "2".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Person Name".to_string(),
            age: "77".to_string(),
        })
        .unwrap();

    let view = ForEach::<10, _, _, _>::new(&users, |user| {
        HStack::three(
            Text::char(&user.name, &FONT),
            Spacer::default(),
            Text::char(&user.age, &FONT),
        )
        .alignment(VerticalAlignment::Bottom)
    });
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let layout = view.layout(buffer.size(), &env);
    view.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Alice   99");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Bob      2");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name    77");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}

mod weather {
    use core::str::FromStr;

    pub static DAYS: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    pub struct Weather {
        pub temperature: heapless::String<6>,
        pub day: &'static str,
        pub rain_chance: u8,
    }

    pub fn weekly_outlook() -> [Weather; 7] {
        [
            Weather {
                temperature: heapless::String::<6>::from_str("75°F").unwrap(),
                day: DAYS[0],
                rain_chance: 0,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("74°F").unwrap(),
                day: DAYS[1],
                rain_chance: 0,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("78°F").unwrap(),
                day: DAYS[2],
                rain_chance: 30,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("78°F").unwrap(),
                day: DAYS[3],
                rain_chance: 40,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("82°F").unwrap(),
                day: DAYS[4],
                rain_chance: 10,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("79°F").unwrap(),
                day: DAYS[5],
                rain_chance: 100,
            },
            Weather {
                temperature: heapless::String::<6>::from_str("75°F").unwrap(),
                day: DAYS[6],
                rain_chance: 10,
            },
        ]
    }
}

#[test]
fn test_weather() {
    let mut buffer = FixedTextBuffer::<50, 100>::default();
    draw(&mut buffer);
    println!("{}", buffer);
}

fn draw<T: CharacterRenderTarget<Color = ()>>(display: &mut T) {
    let display_area = display.size();

    let outlook = weather::weekly_outlook();

    let view = view(&outlook);
    let env = DefaultEnvironment::new(());
    let layout = view.layout(display_area, &env);

    view.render(display, &layout, buoyant::primitives::Point::zero(), &env);
}

fn view(outlook: &[weather::Weather]) -> impl CharacterRender<()> + use<'_> {
    HStack::three(weather(outlook), Divider::default(), clock())
}

fn weather(outlook: &[weather::Weather]) -> impl CharacterRender<()> + use<'_> {
    ForEach::<7, _, _, _>::new(outlook, |weather| {
        HStack::three(
            Text::char(weather.day, &FONT),
            Divider::default(),
            Text::char(&weather.temperature, &FONT),
        )
    })
}

fn clock() -> impl CharacterRender<()> {
    Text::char("This is a test of the display and text wrapping behavior. If you're reading this, it worked! This text should be in the middle and wrap several times.", &FONT)
}
