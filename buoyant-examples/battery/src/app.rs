use core::time::Duration;

use buoyant::view::prelude::*;

use crate::{charge_simulator::ChargeSim, view::Screen};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Unpressed,
    Pressed(Duration),
    Reset,
}

impl ButtonState {
    #[must_use]
    pub fn is_unpressed(&self) -> bool {
        matches!(self, Self::Unpressed)
    }

    #[must_use]
    pub fn pressed(&self) -> Option<Duration> {
        if let Self::Pressed(i) = self {
            Some(*i)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub simulator: ChargeSim,
    pub screen: Screen,
    pub auto_off: bool,
}

impl State {
    #[must_use]
    pub fn new(simulator: ChargeSim) -> Self {
        Self {
            simulator,
            screen: Screen::default(),
            auto_off: false,
        }
    }
}

/// The root view function used by [`buoyant::app::App`].
#[must_use]
pub fn root_view(state: &State) -> impl View<crate::color::ColorFormat, State> + use<> {
    crate::view::root_view(state.screen, &state.simulator.battery, state.auto_off)
}
