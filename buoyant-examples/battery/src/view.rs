use buoyant::{match_view, view::prelude::*};

use crate::{color::ColorFormat, state::BatteryStatus};

mod charge;
pub mod screen_setting;
pub mod stats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Charge,
    Stats,
    Settings,
}

impl Screen {
    pub fn increment(&mut self) {
        *self = match self {
            Self::Charge => Self::Stats,
            Self::Stats => Self::Settings,
            Self::Settings => Self::Charge,
        };
    }
}

#[must_use]
pub fn root_view(
    screen: Screen,
    battery: &BatteryStatus,
    auto_off: bool,
) -> impl View<ColorFormat, ()> + use<> {
    match_view!(screen, {
        Screen::Charge => charge::view(battery),
        Screen::Settings => screen_setting::view(auto_off),
        Screen::Stats => stats::view(battery),
    })
    .padding(Edges::All, 5)
}
