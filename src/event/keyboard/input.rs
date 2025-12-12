use core::{cell::Cell, time::Duration};

use crate::event::{input::Groups, keyboard::KeyboardEventKind};

use super::{ButtonState, Key, KeyboardEvent};

#[derive(Debug, Clone, Copy)]
struct FullButtonState {
    key: Key,
    button_state: ButtonState,
    timestamp: Duration,
    issued_long_at: Option<Duration>,
}

#[derive(Debug)]
pub struct KeyboardInput {
    prev_button: Cell<Option<FullButtonState>>,
    long_press_threshold: Duration,
    long_press_duration_per_repeat: Duration,
}

impl KeyboardInput {
    pub const fn new() -> Self {
        Self {
            prev_button: Cell::new(None),
            long_press_threshold: Duration::new(1, 0),
            long_press_duration_per_repeat: Duration::new(0, 300_000_000),
        }
    }
    pub const fn with_long_press(
        mut self,
        threshold: Duration,
        duration_per_repeat: Duration,
    ) -> Self {
        self.long_press_threshold = threshold;
        self.long_press_duration_per_repeat = duration_per_repeat;
        self
    }

    pub fn input(
        &self,
        key: Key,
        button_state: ButtonState,
        timestamp: Duration,
    ) -> Option<KeyboardEvent> {
        let mut prev_button = self.prev_button.take();

        if let Some(prev) = prev_button.as_ref()
            && timestamp < prev.timestamp
        {
            debug_assert!(
                prev.timestamp <= timestamp,
                "Timestamps must be non-decreasing"
            );
            prev_button = None;
        }

        let event = match key {
            Key::Up => Some(KeyboardEventKind::Up),
            Key::Down => Some(KeyboardEventKind::Down),
            Key::Left => Some(KeyboardEventKind::Left),
            Key::Right => Some(KeyboardEventKind::Right),
            Key::Enter => Some(KeyboardEventKind::Click),
            Key::Escape => Some(KeyboardEventKind::Cancel),
            Key::Char(c) => Some(KeyboardEventKind::Char(c)),
        };

        let event = match (prev_button.as_mut(), button_state) {
            (Some(prev), ButtonState::Pressed)
                if prev.button_state == ButtonState::Pressed && prev.key == key =>
            {
                let long = if key == Key::Enter {
                    Some(KeyboardEventKind::LongPress)
                } else {
                    event
                };
                let long = if prev.issued_long_at.is_none()
                    && prev.timestamp + self.long_press_threshold <= timestamp
                {
                    long
                } else if let Some(prev_timestamp) = prev.issued_long_at
                    && prev_timestamp + self.long_press_duration_per_repeat <= timestamp
                {
                    long
                } else {
                    None
                };

                if long.is_some() {
                    prev_button = Some(FullButtonState {
                        key,
                        button_state,
                        timestamp,
                        issued_long_at: Some(timestamp),
                    });
                }

                long
            }
            (Some(prev), ButtonState::Released)
                if prev.button_state == ButtonState::Pressed && prev.key == key =>
            {
                if prev.issued_long_at.is_none() {
                    prev_button = None;
                    event
                } else {
                    prev_button = None;
                    None
                }
            }
            _ => {
                prev_button = Some(FullButtonState {
                    key,
                    button_state,
                    timestamp,
                    issued_long_at: None,
                });

                None
            }
        };

        self.prev_button.set(prev_button);

        event.map(|kind| KeyboardEvent {
            kind,
            groups: Groups::ZERO,
        })
    }
}

impl Eq for KeyboardInput {}
impl PartialEq for KeyboardInput {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ButtonState::{Pressed as P, Released as R};
    use KeyboardEventKind::*;

    fn ms(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    fn ki() -> KeyboardInput {
        KeyboardInput {
            prev_button: None.into(),
            long_press_threshold: ms(500),
            long_press_duration_per_repeat: ms(200),
        }
    }

    #[test]
    fn single_press() {
        let ki = ki();
        assert_eq!(ki.input(Key::Enter, P, ms(0)), None);
        assert_eq!(ki.input(Key::Enter, R, ms(100)).unwrap().kind, Click);
    }

    #[test]
    fn single_press_twice() {
        let ki = ki();
        assert_eq!(ki.input(Key::Enter, P, ms(0)), None);
        assert_eq!(ki.input(Key::Enter, R, ms(100)).unwrap().kind, Click);
        assert_eq!(ki.input(Key::Enter, P, ms(200)), None);
        assert_eq!(ki.input(Key::Enter, R, ms(300)).unwrap().kind, Click);
    }

    #[test]
    fn press_up_down() {
        let ki = ki();
        assert_eq!(ki.input(Key::Up, P, ms(0)), None);
        assert_eq!(ki.input(Key::Up, R, ms(100)).unwrap().kind, Up);
        assert_eq!(ki.input(Key::Down, P, ms(200)), None);
        assert_eq!(ki.input(Key::Down, R, ms(300)).unwrap().kind, Down);
    }

    #[test]
    fn press_up_down_interrupt() {
        let ki = ki();
        assert_eq!(ki.input(Key::Up, P, ms(0)), None);
        assert_eq!(ki.input(Key::Down, P, ms(200)), None);
        assert_eq!(ki.input(Key::Down, R, ms(300)).unwrap().kind, Down);
    }

    #[test]
    fn press_long_up_then_down_interrupt() {
        let ki = ki();
        assert_eq!(ki.input(Key::Up, P, ms(0)), None);
        assert_eq!(ki.input(Key::Up, P, ms(500)).unwrap().kind, Up);
        assert_eq!(ki.input(Key::Up, P, ms(700)).unwrap().kind, Up);
        assert_eq!(ki.input(Key::Down, P, ms(800)), None);
        assert_eq!(ki.input(Key::Down, R, ms(900)).unwrap().kind, Down);
    }

    #[test]
    fn long_press_and_press() {
        let ki = ki();
        assert_eq!(ki.input(Key::Enter, P, ms(0)), None);
        assert_eq!(ki.input(Key::Enter, P, ms(100)), None);
        assert_eq!(ki.input(Key::Enter, P, ms(300)), None);
        assert_eq!(ki.input(Key::Enter, P, ms(500)).unwrap().kind, LongPress);
        assert_eq!(ki.input(Key::Enter, P, ms(600)), None);
        assert_eq!(ki.input(Key::Enter, P, ms(700)).unwrap().kind, LongPress);
        assert_eq!(ki.input(Key::Enter, P, ms(900)).unwrap().kind, LongPress);
        assert_eq!(ki.input(Key::Enter, P, ms(1000)), None);
        assert_eq!(ki.input(Key::Enter, R, ms(1100)), None);
        assert_eq!(ki.input(Key::Enter, P, ms(1200)), None);
        assert_eq!(ki.input(Key::Enter, R, ms(1600)).unwrap().kind, Click);
    }
}
