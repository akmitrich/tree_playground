use std::{
    cmp::{max, Ordering},
    mem::{replace, swap},
};

type AvlTree = Option<Box<AvlNode>>;

#[derive(Debug, Clone, PartialEq)]
struct AvlNode {
    value: i64,
    height: usize,
    left: AvlTree,
    right: AvlTree,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AvlSet {
    root: AvlTree,
}

impl AvlNode {
    pub fn boxed(value: i64) -> Box<Self> {
        Box::new(Self {
            value,
            left: None,
            right: None,
            height: 0,
        })
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    pub fn rebalance(&mut self) {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();
                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }
                self.rotate_left()
            }
            2 => {
                let left_node = self.left.as_mut().unwrap();
                if left_node.balance_factor() == -1 {
                    left_node.rotate_left()
                }
                self.rotate_right();
            }
            _ => (),
        }
    }

    fn balance_factor(&self) -> isize {
        self.left_height() as isize - self.right_height() as isize
    }

    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |node| node.height)
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |node| node.height)
    }

    fn rotate_left(&mut self) {
        if self.right.is_some() {
            let right_node = self.right.as_mut().unwrap();
            let right_left_tree = right_node.left.take();
            let right_right_tree = right_node.right.take();

            let mut new_left_tree = replace(&mut self.right, right_right_tree);
            swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
            let left_tree = self.left.take();

            let new_left_node = new_left_tree.as_mut().unwrap();
            new_left_node.right = right_left_tree;
            new_left_node.left = left_tree;
            self.left = new_left_tree;

            if let Some(node) = self.left.as_mut() {
                node.update_height();
            }

            self.update_height();
        }
    }

    fn rotate_right(&mut self) {
        if self.left.is_some() {
            let left_node = self.left.as_mut().unwrap();
            let left_right_tree = left_node.right.take();
            let left_left_tree = left_node.left.take();

            let mut new_right_tree = replace(&mut self.left, left_left_tree);
            swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);
            let right_tree = self.right.take();

            let new_right_node = new_right_tree.as_mut().unwrap();
            new_right_node.left = left_right_tree;
            new_right_node.right = right_tree;
            self.right = new_right_tree;

            if let Some(node) = self.right.as_mut() {
                node.update_height();
            }

            self.update_height();
        }
    }
}

impl AvlSet {
    pub fn insert(&mut self, value: i64) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode>::new();
        let mut current = &mut self.root;
        while let Some(node) = current {
            prev_ptrs.push(&mut **node);
            current = match node.value.cmp(&value) {
                Ordering::Less => &mut node.right,
                Ordering::Equal => return false,
                Ordering::Greater => &mut node.left,
            }
        }
        *current = Some(AvlNode::boxed(value));
        current.as_mut().unwrap().update_height();
        for ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *ptr };
            node.update_height();
            node.rebalance();
        }
        true
    }
}

impl<'a> AvlSet {
    pub fn iter(&'a self) -> AvlIter<'a> {
        AvlIter {
            prev_nodes: Vec::new(),
            current: &self.root,
        }
    }
}

pub struct AvlIter<'a> {
    prev_nodes: Vec<&'a AvlNode>,
    current: &'a AvlTree,
}

impl<'a> Iterator for AvlIter<'a> {
    type Item = &'a i64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current {
                None => match self.prev_nodes.pop() {
                    Some(prev_node) => {
                        self.current = &prev_node.right;
                        return Some(&prev_node.value);
                    }
                    None => return None,
                },
                Some(ref node) => {
                    if node.left.is_some() {
                        self.prev_nodes.push(node);
                        self.current = &node.left;
                        continue;
                    }
                    if node.right.is_some() {
                        self.current = &node.right;
                        return Some(&node.value);
                    }
                    self.current = &None;
                    return Some(&node.value);
                }
            }
        }
    }
}
