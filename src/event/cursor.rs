use core::cell::Cell;

use crate::event::{EventResult, keyboard::KeyboardEventKind as Kind};

#[derive(Debug, PartialEq, Eq)]
pub struct ComponentPath {
    path: [Cell<u8>; 128],
    focused: Cell<bool>,
    len: Cell<u8>,
    offset: Cell<u8>,
}

impl ComponentPath {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            path: [const { Cell::new(0) }; 128],
            focused: Cell::new(false),
            len: Cell::new(0),
            offset: Cell::new(0),
        }
    }

    #[inline]
    fn current(&self) -> u8 {
        self.path[self.offset.get() as usize].get()
    }
    #[inline]
    fn set_current(&self, value: u8) {
        self.path[self.offset.get() as usize].set(value);
    }

    #[cfg(test)]
    #[allow(clippy::needless_range_loop)]
    fn path_chunk<const N: usize>(&self) -> [u8; N] {
        let mut arr = [0u8; N];
        for i in 0..N {
            arr[i] = self.path[i].get();
        }
        arr
    }

    pub fn is_focused(&self) -> bool {
        self.focused.get()
    }
    pub fn focus(&self) {
        self.focused.set(true);
    }
    pub fn blur(&self) {
        self.focused.set(false);
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len.get() == 0
    }

    pub fn reset(&self) {
        self.blur();
        self.len.set(0);
    }

    pub fn traverse(
        &self,
        event: Kind,
        max: usize,
        mut f: impl FnMut(usize) -> EventResult,
    ) -> EventResult {
        let is_forward = matches!(event, Kind::Down | Kind::Right);
        let offset = self.offset.get();
        let max = max as u8;

        let mut result = EventResult::default();
        let mut allowed_retry = offset == 0;

        if self.len.get() == offset {
            if event.is_movement() {
                self.set_current(if is_forward { 0 } else { max });
                self.len.set(offset + 1);
            } else {
                return result;
            }
        }

        debug_assert!(offset < self.len.get());

        loop {
            let current = self.current();

            self.offset.set(offset + 1);

            result.merge(f(current as usize));

            self.offset.set(offset);

            if result.handled || !event.is_movement() {
                return result;
            }

            // Remove the path to unsuccessful child
            self.len.set(offset + 1);

            let overflow = if is_forward {
                self.delta(1, max)
            } else {
                self.delta(-1, 0)
            };

            if overflow {
                if allowed_retry {
                    allowed_retry = false;
                    self.set_current(if is_forward { 0 } else { max });
                    continue;
                }
                self.len.set(offset);
                return result;
            }
        }
    }

    #[inline]
    pub fn delta(&self, delta: i8, bound: u8) -> bool {
        let current = self.current();
        if current == bound {
            true
        } else {
            self.set_current((current as i8 + delta) as u8);
            false
        }
    }
}

impl Default for ComponentPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::event::keyboard::KeyboardEventKind;

    use super::*;
    use std::{vec, vec::Vec};

    #[derive(Default, Debug)]
    struct Meta {
        order: Vec<usize>,
        focused_id: Option<usize>,
    }

    #[derive(Debug)]
    enum Node {
        Node(Vec<Self>),
        Leaf {
            id: usize,
            focusable: bool,
            focused: bool,
        },
    }

    impl Node {
        fn handle(
            &mut self,
            meta: &mut Meta,
            event: KeyboardEventKind,
            path: &ComponentPath,
        ) -> EventResult {
            match self {
                Self::Node(children) => {
                    let max = children.len() - 1;

                    path.traverse(event, max, |i| children[i].handle(meta, event, path))
                }
                Self::Leaf {
                    id,
                    focusable,
                    focused,
                } => {
                    let mut result = EventResult::default();
                    meta.order.push(*id);
                    if !*focusable {
                        return result;
                    }
                    if *focused {
                        assert!(path.is_focused(), "Inconsistent focus state ({id})");
                        *focused = false;
                        meta.focused_id = None;
                        path.blur();
                    } else {
                        assert!(!path.is_focused(), "Inconsistent focus state ({id})");
                        *focused = true;
                        meta.focused_id = Some(*id);
                        path.focus();
                        result.handled = true;
                    }

                    result
                }
            }
        }
    }

    #[test]
    fn test_find_focus_forward() {
        let mut tree = Node::Node(vec![
            Node::Leaf {
                id: 1,
                focusable: false,
                focused: false,
            },
            Node::Node(vec![
                Node::Leaf {
                    id: 2,
                    focusable: true,
                    focused: false,
                },
                Node::Node(vec![
                    Node::Leaf {
                        id: 3,
                        focusable: false,
                        focused: false,
                    },
                    Node::Leaf {
                        id: 4,
                        focusable: true,
                        focused: false,
                    },
                ]),
                Node::Leaf {
                    id: 5,
                    focusable: false,
                    focused: false,
                },
                Node::Leaf {
                    id: 6,
                    focusable: true,
                    focused: false,
                },
            ]),
            Node::Leaf {
                id: 7,
                focusable: true,
                focused: false,
            },
        ]);

        let path = ComponentPath::new();

        assert!(path.is_empty());

        for i in 0..5 {
            std::println!("--- Next {i}.0 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(2));
            if i == 0 {
                assert_eq!(meta.order, vec![1, 2]);
            } else {
                assert_eq!(meta.order, vec![7, 1, 2]);
            }
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 0]);

            std::println!("--- Next {i}.1 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![2, 3, 4]);
            assert_eq!(path.len.get(), 3);
            assert_eq!(path.path_chunk(), [1, 1, 1]);

            std::println!("--- Next {i}.2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![4, 5, 6]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 3]);

            std::println!("--- Next {i}.3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(7));
            assert_eq!(meta.order, vec![6, 7]);
            assert_eq!(path.len.get(), 1);
            assert_eq!(path.path_chunk(), [2]);
        }
    }

    #[test]
    fn test_find_focus_backward() {
        let mut tree = Node::Node(vec![
            Node::Leaf {
                id: 1,
                focusable: false,
                focused: false,
            },
            Node::Node(vec![
                Node::Leaf {
                    id: 2,
                    focusable: true,
                    focused: false,
                },
                Node::Node(vec![
                    Node::Leaf {
                        id: 3,
                        focusable: false,
                        focused: false,
                    },
                    Node::Leaf {
                        id: 4,
                        focusable: true,
                        focused: false,
                    },
                ]),
                Node::Leaf {
                    id: 5,
                    focusable: false,
                    focused: false,
                },
                Node::Leaf {
                    id: 6,
                    focusable: true,
                    focused: false,
                },
            ]),
            Node::Leaf {
                id: 7,
                focusable: true,
                focused: false,
            },
        ]);

        let path = ComponentPath::new();

        assert!(path.is_empty());

        let mut meta = Meta::default();
        tree.handle(&mut meta, KeyboardEventKind::Down, &path);
        tree.handle(&mut meta, KeyboardEventKind::Down, &path);
        tree.handle(&mut meta, KeyboardEventKind::Down, &path);
        tree.handle(&mut meta, KeyboardEventKind::Down, &path);

        assert_eq!(
            meta.focused_id,
            Some(7),
            "Assumption that 'forward' test passes."
        );

        for i in 0..5 {
            std::println!("--- Next {i}.0  ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![7, 6]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 3]);

            std::println!("--- Next {i}.1 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![6, 5, 4]);
            assert_eq!(path.len.get(), 3);
            assert_eq!(path.path_chunk(), [1, 1, 1]);

            std::println!("--- Next {i}.2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(2));
            assert_eq!(meta.order, vec![4, 3, 2]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 0]);

            std::println!("--- Next {i}.3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(7));
            assert_eq!(meta.order, vec![2, 1, 7]);
            assert_eq!(path.len.get(), 1);
            assert_eq!(path.path_chunk(), [2]);
        }
    }

    #[test]
    fn test_find_focus_backward_start() {
        let mut tree = Node::Node(vec![Node::Node(vec![
            Node::Leaf {
                id: 1,
                focusable: false,
                focused: false,
            },
            Node::Node(vec![
                Node::Leaf {
                    id: 2,
                    focusable: true,
                    focused: false,
                },
                Node::Node(vec![
                    Node::Leaf {
                        id: 3,
                        focusable: false,
                        focused: false,
                    },
                    Node::Leaf {
                        id: 4,
                        focusable: true,
                        focused: false,
                    },
                ]),
                Node::Leaf {
                    id: 5,
                    focusable: false,
                    focused: false,
                },
                Node::Leaf {
                    id: 6,
                    focusable: true,
                    focused: false,
                },
            ]),
            Node::Leaf {
                id: 7,
                focusable: true,
                focused: false,
            },
        ])]);

        let path = ComponentPath::new();

        assert!(path.is_empty());

        for i in 0..5 {
            std::println!("--- Next {i}.0 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(7));
            if i == 0 {
                assert_eq!(meta.order, vec![7]);
            } else {
                assert_eq!(meta.order, vec![2, 1, 7]);
            }
            // assert_eq!(path.len.get(), 1);
            // assert_eq!(path.path_chunk(), [2]);

            std::println!("--- Next {i}.1  ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![7, 6]);
            // assert_eq!(path.len.get(), 2);
            // assert_eq!(path.path_chunk(), [1, 3]);

            std::println!("--- Next {i}.2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![6, 5, 4]);
            // assert_eq!(path.len.get(), 3);
            // assert_eq!(path.path_chunk(), [1, 1, 1]);

            std::println!("--- Next {i}.3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(2));
            assert_eq!(meta.order, vec![4, 3, 2]);
            // assert_eq!(path.len.get(), 2);
            // assert_eq!(path.path_chunk(), [1, 0]);
        }
    }
}
