use crate::state::{BatteryStatus, CellState, PortState};

#[derive(Debug, Clone, PartialEq)]
pub struct ChargeSim {
    pub battery: BatteryStatus,
    last_update: std::time::Instant,
}

impl ChargeSim {
    #[must_use]
    pub fn new(capacity: f32) -> Self {
        let battery = BatteryStatus {
            ports: PortState {
                usbc1_power: 0.0,
                usbc2_power: 0.0,
                usba_power: 0.0,
            },
            cell_state: CellState {
                charge: 0.0,
                usable_capacity: capacity,
                original_capacity: capacity,
                cycle_count: 0.0,
            },
        };
        Self {
            battery,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_update;
        self.last_update = now;
        let net_power = self.battery.net_power();
        let hours_elapsed = elapsed.as_secs_f32() / 3600.0;
        self.battery.cell_state.cycle_count +=
            net_power.abs() * hours_elapsed / self.battery.cell_state.original_capacity;
        let charge = &mut self.battery.cell_state.charge;
        *charge -= net_power * hours_elapsed;
        if *charge >= self.battery.cell_state.usable_capacity
            && self.battery.ports.usbc1_power < 0.0
        {
            let out = self.battery.ports.usbc2_power + self.battery.ports.usba_power;
            self.battery.ports.usbc1_power = -out;
        }
        if *charge <= 0.0 {
            if self.battery.ports.usbc1_power > 0.0 {
                self.battery.ports.usbc1_power = 0.0;
            }
            self.battery.ports.usbc2_power = 0.0;
            self.battery.ports.usba_power = 0.0;
        }
        *charge = charge.clamp(0.0, self.battery.cell_state.usable_capacity);
    }
}
