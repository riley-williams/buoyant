use std::time::{Duration, Instant};

use buoyant::{
    environment::DefaultEnvironment,
    layout::Layout as _,
    primitives::ProposedDimensions,
    render::{Render, Renderable as _},
};

use crate::{charge_simulator::ChargeSim, color, view::Screen};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Unpressed,
    Pressed(Instant),
    Reset,
}

impl ButtonState {
    #[must_use]
    pub fn is_unpressed(&self) -> bool {
        matches!(self, Self::Unpressed)
    }

    #[must_use]
    pub fn pressed(&self) -> Option<Instant> {
        if let Self::Pressed(i) = self {
            Some(*i)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    state: State,
    is_dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub simulator: ChargeSim,
    pub screen: Screen,
    pub auto_off: bool,
}

impl App {
    #[must_use]
    pub fn new(simulator: ChargeSim) -> Self {
        Self {
            state: State {
                simulator,
                screen: Screen::default(),
                auto_off: false,
            },
            is_dirty: false,
        }
    }

    pub fn state_mut(&mut self) -> &mut State {
        self.is_dirty = true;
        &mut self.state
    }

    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn reset_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }

    #[must_use]
    pub fn tree(
        &self,
        dimensions: ProposedDimensions,
        app_time: Duration,
    ) -> impl Render<color::ColorFormat> {
        let env = DefaultEnvironment::new(app_time);
        let view = crate::view::root_view(
            self.state.screen,
            &self.state.simulator.battery,
            self.state.auto_off,
        );
        let layout = view.layout(&dimensions, &env);
        view.render_tree(&layout, buoyant::primitives::Point::zero(), &env)
    }
}
