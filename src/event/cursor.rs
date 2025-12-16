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
    pub const fn new() -> Self {
        Self {
            path: [const { Cell::new(0) }; 128],
            focused: Cell::new(false),
            len: Cell::new(0),
            offset: Cell::new(0),
        }
    }

    #[inline]
    fn current(&self) -> usize {
        self.path[self.offset.get() as usize].get() as usize
    }
    #[inline]
    fn set_current(&self, value: u8) {
        self.path[self.offset.get() as usize].set(value);
    }

    #[cfg(test)]
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
        let mut result = EventResult::default();
        let offset = self.offset.get();

        debug_assert!(matches!(
            event,
            Kind::Up | Kind::Down | Kind::Left | Kind::Right
        ));

        if self.len.get() == offset {
            match event {
                Kind::Down | Kind::Right => self.set_current(0),
                Kind::Up | Kind::Left => self.set_current(max as u8),
                _ => (),
            }
            self.len.set(offset + 1);
        }

        debug_assert!(offset < self.len.get());

        loop {
            let current = self.current();

            self.offset.set(offset + 1);

            result.merge(f(current));

            self.offset.set(offset);

            if result.handled {
                return result;
            }

            let overflow = match event {
                Kind::Down | Kind::Right => self.delta(1, max),
                Kind::Up | Kind::Left => self.delta(-1, 0),
                _ => false,
            };

            if overflow {
                self.len.set(offset);
                return result;
            } else {
                self.len.set(offset + 1);
            }
        }
    }

    #[inline]
    pub fn delta(&self, delta: i8, bound: usize) -> bool {
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
    use std::{cell::Cell, vec, vec::Vec};

    #[derive(Default, Debug)]
    struct Meta {
        order: Vec<usize>,
        focused_id: Option<usize>,
    }

    #[derive(Debug)]
    enum Node {
        Node(Vec<Node>),
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
                Node::Node(children) => {
                    let max = children.len() - 1;

                    path.traverse(event, max, |i| children[i].handle(meta, event, path))
                }
                Node::Leaf {
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
                        if !path.is_focused() {
                            panic!("Inconsistent focus state");
                        }
                        *focused = false;
                        meta.focused_id = None;
                        path.blur();
                    } else {
                        if path.is_focused() {
                            panic!("Inconsistent focus state");
                        }
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

        for _ in 0..5 {
            assert!(path.is_empty());

            std::println!("--- Next 0 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(2));
            assert_eq!(meta.order, vec![1, 2]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 0]);

            std::println!("--- Next 1 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![2, 3, 4]);
            assert_eq!(path.len.get(), 3);
            assert_eq!(path.path_chunk(), [1, 1, 1]);

            std::println!("--- Next 2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![4, 5, 6]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 3]);

            std::println!("--- Next 3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(7));
            assert_eq!(meta.order, vec![6, 7]);
            assert_eq!(path.len.get(), 1);
            assert_eq!(path.path_chunk(), [2]);

            std::println!("--- Next 4 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(!result.handled);
            assert_eq!(meta.focused_id, None);
            assert_eq!(meta.order, vec![7]);
            assert_eq!(path.len.get(), 0);
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

        for _ in 0..5 {
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

            std::println!("--- Next 0 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![7, 6]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 3]);

            std::println!("--- Next 1 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![6, 5, 4]);
            assert_eq!(path.len.get(), 3);
            assert_eq!(path.path_chunk(), [1, 1, 1]);

            std::println!("--- Next 2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(2));
            assert_eq!(meta.order, vec![4, 3, 2]);
            assert_eq!(path.len.get(), 2);
            assert_eq!(path.path_chunk(), [1, 0]);

            std::println!("--- Next 3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Up, &path);
            assert!(!result.handled);
            assert_eq!(meta.focused_id, None);
            assert_eq!(meta.order, vec![2, 1]);
            assert_eq!(path.len.get(), 0);
        }
    }
}
