use core::cell::Cell;

use crate::event::{EventResult, keyboard::KeyboardEventKind as Kind};

#[derive(Debug, PartialEq, Eq)]
pub struct ComponentPath {
    path: [Cell<u8>; 128],
    focused: Cell<bool>,
    init_offset: Cell<u8>,
    offset: Cell<u8>,
}

impl ComponentPath {
    pub const fn new() -> Self {
        Self {
            path: [const { Cell::new(0) }; 128],
            focused: Cell::new(false),
            init_offset: Cell::new(0),
            offset: Cell::new(0),
        }
    }

    #[inline]
    fn current(&self) -> usize {
        self.path[self.offset.get() as usize].get() as usize
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
        self.init_offset.get() == 0
    }

    pub fn reset(&self) {
        self.blur();
        self.path[0].set(0);
        self.init_offset.set(0);
    }

    pub fn traverse(
        &self,
        event: Kind,
        max: usize,
        mut f: impl FnMut(usize) -> EventResult,
    ) -> EventResult {
        let mut result = EventResult::default();
        let offset = self.offset.get();

        loop {
            let current = self.current();

            // std::println!(
            //     "Traverse event: offset={}, init_offset={}, current={}",
            //     self.offset.get(),
            //     self.init_offset.get(),
            //     current
            // );
            if self.init_offset.get() == offset {
                self.init_offset.set(offset + 1);
                self.path[offset as usize + 1].set(0);
            }
            self.offset.set(offset + 1);

            result.merge(f(current));

            self.offset.set(offset);

            if result.handled {
                return result;
            }

            self.init_offset.set(offset);

            if !match event {
                Kind::Down | Kind::Right => self.delta(1, max),
                Kind::Up | Kind::Left => self.delta(-1, 0),
                _ => false,
            } {
                self.path[offset as usize].set(0);
                return result;
            }
        }
    }

    #[inline]
    pub fn delta(&self, delta: i8, bound: usize) -> bool {
        let current = self.current();
        if current == bound {
            false
        } else {
            let next = (current as i8 + delta) as u8;
            self.path[self.offset.get() as usize].set(next);
            true
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
    fn test_find_focus() {
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
            assert_eq!(path.init_offset.get(), 2);
            assert_eq!(path.path[..2], vec![Cell::new(1), Cell::new(0)]);

            std::println!("--- Next 1 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(4));
            assert_eq!(meta.order, vec![2, 3, 4]);
            assert_eq!(path.init_offset.get(), 3);
            assert_eq!(
                path.path[..3],
                vec![Cell::new(1), Cell::new(1), Cell::new(1)]
            );
            std::println!(
                "Path after Next 1: {:?}",
                &path.path[..path.init_offset.get() as usize]
            );

            std::println!("--- Next 2 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(6));
            assert_eq!(meta.order, vec![4, 5, 6]);
            assert_eq!(path.init_offset.get(), 2);
            assert_eq!(path.path[..2], vec![Cell::new(1), Cell::new(3)]);

            std::println!("--- Next 3 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(result.handled);
            assert_eq!(meta.focused_id, Some(7));
            assert_eq!(meta.order, vec![6, 7]);
            assert_eq!(path.init_offset.get(), 1);
            assert_eq!(path.path[..1], vec![Cell::new(2)]);

            std::println!("--- Next 4 ---");

            let mut meta = Meta::default();
            let result = tree.handle(&mut meta, KeyboardEventKind::Down, &path);
            assert!(!result.handled);
            assert_eq!(meta.focused_id, None);
            assert_eq!(meta.order, vec![7]);
            assert_eq!(path.init_offset.get(), 0);
        }
    }
}
