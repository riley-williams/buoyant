use crate::definitions::{HwCell, IeName, Page};

pub const HEADER: &[HwCell] = &[
    HwCell::DoubleDigital("T1", true, false),
    HwCell::DoubleDigital("T2", false, true),
    HwCell::Digital("C1", false),
    HwCell::Analog("U1", 75),
    HwCell::Analog("U2", 40),
    HwCell::Analog("A1", 10),
    HwCell::Analog("A2", 10),
    HwCell::Analog("A3", 0),
    HwCell::Analog("A4", 1),
    HwCell::Analog("A5", 10),
    HwCell::Analog("A6", 100),
    HwCell::Analog("A7", 99),
    HwCell::Analog("A8", 10),
    HwCell::DoubleDigital("T3", false, false),
];
pub const FOOTER: &[HwCell] = &[
    HwCell::Digital("Q1", false),
    HwCell::Digital("Q2", false),
    HwCell::Digital("Q3", false),
    HwCell::Digital("Q4", false),
    HwCell::Digital("Y1", true),
    HwCell::Digital("Y2", false),
    HwCell::Digital("Y3", false),
    HwCell::Digital("Y4", false),
    HwCell::Digital("X1", true),
    HwCell::Digital("X2", true),
    HwCell::Digital("X3", true),
    HwCell::Digital("X4", true),
    HwCell::Digital("X5", false),
    HwCell::Digital("X6", true),
    HwCell::Digital("X7", true),
    HwCell::Digital("X8", true),
];

pub const SETTINGS: Page = Page::Settings {
    header: HEADER,
    footer: FOOTER,
};

pub const PAGE_1: Page = Page::IeTable {
    header: HEADER,
    footer: FOOTER,
    table_dimensions: (3, 3),
    names: NAMES_1,
    ie: IE_1,
    eu: EU_1,
};

pub const PAGE_2: Page = Page::IeTable {
    header: HEADER,
    footer: FOOTER,
    table_dimensions: (3, 3),
    names: NAMES_2,
    ie: IE_2,
    eu: EU_2,
};

pub const NAMES_1: &[Option<IeName>] = &[
    Some(IeName::Known("IE2")),
    Some(IeName::Known("IE3")),
    Some(IeName::Known("Long Name 123")),
    Some(IeName::Known("IE4")),
    Some(IeName::Known("Long Name 12345")),
    Some(IeName::Known("IE20")),
    Some(IeName::Known("IE5")),
    Some(IeName::Known("IE1")),
    Some(IeName::Known("Veeery Long Name 12345 123456789abcdEF")),
];

pub const IE_1: &[Option<f32>] = &[
    Some(32.3f32),
    Some(32.3f32),
    Some(0.3f32),
    Some(32.3f32),
    Some(32.3f32),
    Some(0.3f32),
    Some(32.3f32),
    Some(32.3f32),
    Some(32000.3f32),
];

pub const EU_1: &[Option<&str>] = &[
    Some("°C"),
    Some("°C"),
    None,
    Some("A"),
    Some("°C"),
    Some("°C"),
    None,
    Some("A"),
    Some("Foo"),
];

pub const NAMES_2: &[Option<IeName>] = &[
    Some(IeName::Addr((192, 168), (1, 1, 1))),
    Some(IeName::Addr((10, 0), (0, 1, 1))),
    Some(IeName::Addr((172, 16), (0, 1, 1))),
    Some(IeName::Known("U1")),
    Some(IeName::Known("U2")),
    Some(IeName::Known("U3")),
    Some(IeName::Known("A1")),
    Some(IeName::Known("A2")),
    Some(IeName::Known("A3")),
];

pub const IE_2: &[Option<f32>] = &[
    Some(32.0f32),
    Some(16.0f32),
    Some(28.0f32),
    Some(12.34f32),
    Some(56.78f32),
    Some(90.12f32),
    Some(3.3f32),
    Some(5.0f32),
    Some(12.0f32),
];
pub const EU_2: &[Option<&str>] = &[
    None,
    None,
    None,
    Some("V"),
    Some("V"),
    Some("V"),
    Some("A"),
    Some("A"),
    Some("A"),
];
