use core::{cell::Cell, time::Duration};

use crate::{
    focus::{self, FocusAction, FocusGroup, RoleSet},
    primitives::Point,
    render::ContentShape,
};

/// An interaction event that can be handled by a view.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// A touch event.
    Touch(embedded_touch::Touch),
    /// A scroll event with the given offset.
    Scroll(Point),
    /// A request to move focus, often driven by navigational buttons
    /// or an encoder.
    Focus {
        /// The focus action to perform.
        action: FocusAction,
        /// The focus group this event targets.
        group: FocusGroup,
    },
    /// A key was pressed.
    KeyDown {
        /// The key that was pressed.
        key: Key,
        /// The focus group this event targets.
        group: FocusGroup,
    },
    /// A key was released.
    KeyUp {
        /// The key that was released.
        key: Key,
        /// The focus group this event targets.
        group: FocusGroup,
    },
}

/// A key press event.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// A character key.
    Character(char),
    /// The up arrow key.
    UpArrow,
    /// The down arrow key.
    DownArrow,
    /// The left arrow key.
    LeftArrow,
    /// The right arrow key.
    RightArrow,
    /// The escape key.
    Escape,
    /// The backspace key.
    Backspace,
    /// The delete key.
    Delete,
}

impl From<FocusAction> for Event {
    fn from(action: FocusAction) -> Self {
        Self::Focus {
            action,
            group: focus::GROUP_0,
        }
    }
}

impl Event {
    /// Returns a new event with the specified offset applied to any point-based data.
    #[must_use]
    pub fn offset(&self, offset: Point) -> Self {
        let mut event = self.clone();
        match &mut event {
            Self::Touch(touch) => {
                touch.location += offset.into();
            }
            Self::Scroll(_) | Self::Focus { .. } | Self::KeyDown { .. } | Self::KeyUp { .. } => {}
        }
        event
    }

    /// Returns a new event with the specified focus group set on Focus events.
    ///
    /// Non-focus events are returned unchanged.
    #[must_use]
    pub fn with_focus_group(mut self, group: FocusGroup) -> Self {
        if let Self::Focus {
            group: ref mut g, ..
        } = self
        {
            *g = group;
        }
        self
    }
}

/// The result of handling an event.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EventResult {
    /// Focus successfully moved to a new element
    Handled {
        /// The content shape of the focused element
        shape: ContentShape,
        /// The element which handled the event has focus
        request_focus: bool,
        /// The group of the focused element
        group: FocusGroup,
    },
    /// The event was not handled, or focus not obtained
    #[default]
    Deferred,
}

impl EventResult {
    /// Creates a new `EventResult` indicating the event was handled but focus was not obtained.
    #[must_use]
    pub const fn handled_unfocused() -> Self {
        Self::Handled {
            shape: ContentShape::Empty,
            request_focus: false,
            group: focus::GROUP_0,
        }
    }

    /// Creates a new `EventResult` indicating the event was handled with focus.
    #[must_use]
    pub const fn handled_focused(shape: ContentShape) -> Self {
        Self::Handled {
            shape,
            request_focus: true,
            group: focus::GROUP_0,
        }
    }

    /// Returns a new [`EventResult`] with the specified focus group.
    #[must_use]
    pub const fn with_group(mut self, group: FocusGroup) -> Self {
        if let Self::Handled {
            group: event_group, ..
        } = &mut self
        {
            *event_group = group;
        }
        self
    }

    /// Returns true if the event was handled (not deferred).
    #[must_use]
    pub const fn is_handled(&self) -> bool {
        matches!(self, Self::Handled { .. })
    }

    /// Returns true if this result is from an element requesting focus.
    #[must_use]
    pub const fn requested_focus(&self) -> bool {
        matches!(
            self,
            Self::Handled {
                request_focus: true,
                ..
            }
        )
    }

    /// Returns the content shape if this result has one.
    #[must_use]
    pub fn shape(&self) -> Option<&ContentShape> {
        match self {
            Self::Handled { shape, .. } => Some(shape),
            Self::Deferred => None,
        }
    }

    /// Returns the result offset by the given amount if it has a shape.
    #[must_use]
    pub fn with_offset(mut self, offset: Point) -> Self {
        match &mut self {
            Self::Handled { shape, .. } => {
                shape.offset(offset);
            }
            Self::Deferred => (),
        }
        self
    }
}

/// Context provided to views when handling events.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventContext {
    /// The time since the application started.
    pub app_time: Duration,
    /// Whether a redraw has been requested.
    pub redraw_requested: Cell<bool>,
    /// Whether a view rebuild has been requested.
    pub view_rebuild_requested: Cell<bool>,
    /// Which roles are currently active.
    pub roles: RoleSet,
}

impl EventContext {
    /// Creates a new `EventContext` with the given application time.
    #[must_use]
    pub const fn new(app_time: Duration) -> Self {
        Self {
            app_time,
            redraw_requested: Cell::new(false),
            view_rebuild_requested: Cell::new(false),
            roles: RoleSet::any(),
        }
    }

    #[must_use]
    pub fn with_roles(mut self, roles: impl Into<RoleSet>) -> Self {
        self.roles = roles.into();
        self
    }

    /// Indicates to the render loop that the view needs to be rebuilt.
    pub fn request_view_rebuild(&self) {
        self.view_rebuild_requested.set(true);
    }

    /// This flag indicates the view should be redrawn even if no animations were reported as
    /// active.
    ///
    /// This should be set when a view directly modifies the render tree state
    /// without requesting a view recompute, e.g. scrollview dragging.
    pub fn request_redraw(&self) {
        self.redraw_requested.set(true);
    }
}

#[cfg(feature = "embedded-graphics-simulator")]
pub mod simulator {
    use crate::{event::Key, primitives::Point};

    use super::Event;
    use embedded_graphics_simulator::{SimulatorEvent, sdl2::Keycode};
    use embedded_touch::{Phase, PointerButton, Tool, Touch, TouchPoint};

    /// Tracks mouse state and converts simulator events to touch events.
    #[derive(Debug, Default)]
    pub struct MouseTracker {
        touch: Option<Touch>,
    }

    impl MouseTracker {
        /// Creates a new mouse tracker.
        #[must_use]
        pub fn new() -> Self {
            Self { touch: None }
        }

        /// Processes a simulator event and returns the corresponding event type.
        pub fn process_event(&mut self, event: SimulatorEvent) -> Option<Event> {
            match event {
                SimulatorEvent::MouseButtonDown { point, mouse_btn } => {
                    let touch = Touch {
                        id: 0,
                        location: TouchPoint::new(point.x, point.y),
                        phase: Phase::Started,
                        tool: Tool::Pointer {
                            button: map_button(mouse_btn),
                        },
                    };
                    self.touch = Some(touch.clone());
                    Some(Event::Touch(touch))
                }
                SimulatorEvent::MouseButtonUp { point, mouse_btn } => {
                    let touch = Touch {
                        id: 0,
                        location: TouchPoint::new(point.x, point.y),
                        phase: Phase::Ended,
                        tool: Tool::Pointer {
                            button: map_button(mouse_btn),
                        },
                    };

                    self.touch = None;
                    Some(Event::Touch(touch))
                }
                SimulatorEvent::MouseMove { point } => {
                    if let Some(touch) = &mut self.touch {
                        touch.location = TouchPoint::new(point.x, point.y);
                        touch.phase = Phase::Moved;
                        Some(Event::Touch(touch.clone()))
                    } else {
                        let touch = Touch {
                            id: 0,
                            location: TouchPoint::new(point.x, point.y),
                            phase: Phase::Hovering(None),
                            tool: Tool::Pointer {
                                button: PointerButton::None,
                            },
                        };

                        Some(Event::Touch(touch))
                    }
                }
                SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction,
                } => {
                    if direction == embedded_graphics_simulator::sdl2::MouseWheelDirection::Flipped
                    {
                        Some(Event::Scroll(Point::new(
                            scroll_delta.x * 4,
                            scroll_delta.y * 4,
                        )))
                    } else {
                        Some(Event::Scroll(Point::new(
                            -scroll_delta.x * 4,
                            -scroll_delta.y * 4,
                        )))
                    }
                }
                SimulatorEvent::Quit => None,
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => keycode.try_into().ok().map(|k| Event::KeyDown {
                    key: k,
                    group: crate::focus::GROUP_0,
                }),
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => keycode.try_into().ok().map(|k| Event::KeyUp {
                    key: k,
                    group: crate::focus::GROUP_0,
                }),
            }
        }
    }

    fn map_button(mouse_btn: embedded_graphics_simulator::sdl2::MouseButton) -> PointerButton {
        match mouse_btn {
            embedded_graphics_simulator::sdl2::MouseButton::Left => PointerButton::Primary,
            embedded_graphics_simulator::sdl2::MouseButton::Right => PointerButton::Secondary,
            embedded_graphics_simulator::sdl2::MouseButton::Middle => PointerButton::Tertiary,
            _ => PointerButton::None,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UnknownKeycode;

    impl TryFrom<embedded_graphics_simulator::sdl2::Keycode> for Key {
        type Error = UnknownKeycode;

        #[allow(clippy::too_many_lines)]
        fn try_from(
            value: embedded_graphics_simulator::sdl2::Keycode,
        ) -> Result<Self, Self::Error> {
            // FIXME: this is incomplete, sometimes wrong
            match value {
                // Arrow keys
                Keycode::RIGHT => Ok(Self::RightArrow),
                Keycode::LEFT => Ok(Self::LeftArrow),
                Keycode::DOWN => Ok(Self::DownArrow),
                Keycode::UP => Ok(Self::UpArrow),

                // Special keys
                Keycode::ESCAPE => Ok(Self::Escape),
                Keycode::DELETE => Ok(Self::Delete),
                Keycode::BACKSPACE | Keycode::KP_BACKSPACE => Ok(Self::Backspace),

                // Whitespace and control
                Keycode::TAB | Keycode::KP_TAB => Ok(Self::Character('\t')),
                Keycode::RETURN | Keycode::KP_ENTER => Ok(Self::Character('\n')),
                Keycode::SPACE | Keycode::KP_SPACE => Ok(Self::Character(' ')),

                // Punctuation (combined regular and keypad)
                Keycode::EXCLAIM | Keycode::KP_EXCLAM => Ok(Self::Character('!')),
                Keycode::QUOTEDBL => Ok(Self::Character('"')),
                Keycode::HASH | Keycode::KP_HASH => Ok(Self::Character('#')),
                Keycode::DOLLAR => Ok(Self::Character('$')),
                Keycode::PERCENT | Keycode::KP_PERCENT => Ok(Self::Character('%')),
                Keycode::AMPERSAND | Keycode::KP_AMPERSAND => Ok(Self::Character('&')),
                Keycode::QUOTE => Ok(Self::Character('\'')),
                Keycode::LEFTPAREN | Keycode::KP_LEFTPAREN => Ok(Self::Character('(')),
                Keycode::RIGHTPAREN | Keycode::KP_RIGHTPAREN => Ok(Self::Character(')')),
                Keycode::ASTERISK | Keycode::KP_MULTIPLY => Ok(Self::Character('*')),
                Keycode::PLUS | Keycode::KP_PLUS => Ok(Self::Character('+')),
                Keycode::COMMA | Keycode::KP_COMMA => Ok(Self::Character(',')),
                Keycode::MINUS | Keycode::KP_MINUS => Ok(Self::Character('-')),
                Keycode::PERIOD | Keycode::KP_PERIOD => Ok(Self::Character('.')),
                Keycode::SLASH | Keycode::KP_DIVIDE => Ok(Self::Character('/')),
                Keycode::COLON | Keycode::KP_COLON => Ok(Self::Character(':')),
                Keycode::SEMICOLON => Ok(Self::Character(';')),
                Keycode::LESS | Keycode::KP_LESS => Ok(Self::Character('<')),
                Keycode::EQUALS | Keycode::KP_EQUALS => Ok(Self::Character('=')),
                Keycode::GREATER | Keycode::KP_GREATER => Ok(Self::Character('>')),
                Keycode::QUESTION => Ok(Self::Character('?')),
                Keycode::AT | Keycode::KP_AT => Ok(Self::Character('@')),
                Keycode::LEFTBRACKET => Ok(Self::Character('[')),
                Keycode::BACKSLASH => Ok(Self::Character('\\')),
                Keycode::RIGHTBRACKET => Ok(Self::Character(']')),
                Keycode::CARET | Keycode::KP_XOR => Ok(Self::Character('^')),
                Keycode::UNDERSCORE => Ok(Self::Character('_')),
                Keycode::BACKQUOTE => Ok(Self::Character('`')),
                Keycode::KP_LEFTBRACE => Ok(Self::Character('{')),
                Keycode::KP_RIGHTBRACE => Ok(Self::Character('}')),
                Keycode::KP_VERTICALBAR => Ok(Self::Character('|')),

                // Numbers (combined regular and keypad)
                Keycode::NUM_0 | Keycode::KP_0 => Ok(Self::Character('0')),
                Keycode::NUM_1 | Keycode::KP_1 => Ok(Self::Character('1')),
                Keycode::NUM_2 | Keycode::KP_2 => Ok(Self::Character('2')),
                Keycode::NUM_3 | Keycode::KP_3 => Ok(Self::Character('3')),
                Keycode::NUM_4 | Keycode::KP_4 => Ok(Self::Character('4')),
                Keycode::NUM_5 | Keycode::KP_5 => Ok(Self::Character('5')),
                Keycode::NUM_6 | Keycode::KP_6 => Ok(Self::Character('6')),
                Keycode::NUM_7 | Keycode::KP_7 => Ok(Self::Character('7')),
                Keycode::NUM_8 | Keycode::KP_8 => Ok(Self::Character('8')),
                Keycode::NUM_9 | Keycode::KP_9 => Ok(Self::Character('9')),

                // Letters (combined regular and keypad)
                Keycode::A | Keycode::KP_A => Ok(Self::Character('a')),
                Keycode::B | Keycode::KP_B => Ok(Self::Character('b')),
                Keycode::C | Keycode::KP_C => Ok(Self::Character('c')),
                Keycode::D | Keycode::KP_D => Ok(Self::Character('d')),
                Keycode::E | Keycode::KP_E => Ok(Self::Character('e')),
                Keycode::F | Keycode::KP_F => Ok(Self::Character('f')),
                Keycode::G => Ok(Self::Character('g')),
                Keycode::H => Ok(Self::Character('h')),
                Keycode::I => Ok(Self::Character('i')),
                Keycode::J => Ok(Self::Character('j')),
                Keycode::K => Ok(Self::Character('k')),
                Keycode::L => Ok(Self::Character('l')),
                Keycode::M => Ok(Self::Character('m')),
                Keycode::N => Ok(Self::Character('n')),
                Keycode::O => Ok(Self::Character('o')),
                Keycode::P => Ok(Self::Character('p')),
                Keycode::Q => Ok(Self::Character('q')),
                Keycode::R => Ok(Self::Character('r')),
                Keycode::S => Ok(Self::Character('s')),
                Keycode::T => Ok(Self::Character('t')),
                Keycode::U => Ok(Self::Character('u')),
                Keycode::V => Ok(Self::Character('v')),
                Keycode::W => Ok(Self::Character('w')),
                Keycode::X => Ok(Self::Character('x')),
                Keycode::Y => Ok(Self::Character('y')),
                Keycode::Z => Ok(Self::Character('z')),

                _ => Err(UnknownKeycode),
            }
        }
    }
}
