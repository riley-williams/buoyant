#[derive(Debug, Clone, PartialEq)]
pub struct BatteryStatus {
    pub ports: PortState,
    pub cell_state: CellState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PortState {
    /// Main USB-C port power, in watts
    ///
    /// Positive values indicate power out, negative values indicate power in
    pub usbc1_power: f32,
    /// Secondary USB-C port power, in watts
    pub usbc2_power: f32,
    /// USB-A port power, in watts
    pub usba_power: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellState {
    /// Remaining capacity of the battery, in watt-hours
    pub charge: f32,
    /// Estimated maximum usable capacity of the battery, in watt-hours
    pub usable_capacity: f32,
    /// Estimated maximum capacity of the battery, in watt-hours
    pub original_capacity: f32,
    /// Count of charge cycles
    pub cycle_count: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChargeEstimate {
    /// Battery is charging, with the given time remaining (s)
    Charging(f32),
    /// Battery is discharging, with the given time remaining (s)
    Discharging(f32),
    Idle,
}

impl BatteryStatus {
    #[must_use]
    pub fn state_of_charge(&self) -> f32 {
        self.cell_state.charge / self.cell_state.usable_capacity * 100.0
    }

    #[must_use]
    pub fn charge_estimate(&self) -> ChargeEstimate {
        match self.net_power() {
            p if p > 0.0 => ChargeEstimate::Discharging(self.cell_state.charge / (p) * 3600.0),
            p if p < 0.0 => ChargeEstimate::Charging(
                (self.cell_state.usable_capacity - self.cell_state.charge) / (-p) * 3600.0,
            ),
            _ => ChargeEstimate::Idle,
        }
    }

    #[must_use]
    pub fn net_power(&self) -> f32 {
        self.ports.usbc1_power + self.ports.usbc2_power + self.ports.usba_power
    }
}
