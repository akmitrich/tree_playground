use std::{
    cmp::{max, Ordering},
    mem::{replace, swap},
};

use crate::Tree;

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

impl Tree for AvlSet {
    fn insert(&mut self, value: i64) -> bool {
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
        update_height_rebalance(prev_ptrs);
        true
    }

    fn search(&self, value: i64) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match node.value.cmp(&value) {
                Ordering::Less => &node.right,
                Ordering::Equal => return true,
                Ordering::Greater => &node.left,
            }
        }
        false
    }

    fn remove(&mut self, value: i64) -> Option<i64> {
        fn find_target<'a>(
            value: i64,
            start: &'a mut AvlTree,
            prev_ptrs: &mut Vec<*mut AvlNode>,
        ) -> Option<&'a mut AvlNode> {
            let mut target = None;
            let mut current = start;
            while let Some(node) = current {
                match node.value.cmp(&value) {
                    Ordering::Less => {
                        prev_ptrs.push(&mut **node);
                        current = &mut node.right;
                    }
                    Ordering::Equal => {
                        target = Some(&mut **node);
                        break;
                    }
                    Ordering::Greater => {
                        prev_ptrs.push(&mut **node);
                        current = &mut node.left;
                    }
                }
            }
            target
        }

        fn has_zero_or_one_child(target: &AvlNode) -> bool {
            target.left.is_none() || target.right.is_none()
        }

        fn try_take_value_when_zero_or_one_child(
            target: &mut AvlNode,
            prev_ptr: Option<*mut AvlNode>,
        ) -> Option<i64> {
            let value = if let Some(left_node) = target.left.take() {
                replace(target, *left_node).value
            } else if let Some(right_node) = target.right.take() {
                replace(target, *right_node).value
            } else {
                let prev = prev_ptr?;
                let prev_node = unsafe { &mut *prev };
                let inner = match prev_node.left {
                    Some(ref left_node) if left_node.value == target.value => {
                        prev_node.left.take().unwrap().value
                    }
                    _ => prev_node.right.take().unwrap().value,
                };
                prev_node.update_height();
                prev_node.rebalance();
                inner
            };
            Some(value)
        }

        fn take_value_when_two_children(target: &mut AvlNode) -> i64 {
            fn tree_has_no_left_children(tree: &AvlTree) -> bool {
                tree.as_ref().unwrap().left.is_none()
            }

            fn raise_right_tree(target: &mut AvlNode) -> i64 {
                let right_node = &mut target.right.take().unwrap();
                let value = replace(&mut target.value, right_node.value);
                target.right = right_node.right.take();
                target.update_height();
                target.rebalance();
                value
            }

            fn swap_with_leftmost_node_from_right_tree(target: &mut AvlNode) -> i64 {
                fn traverse_to_leftmost(tree: &mut AvlTree) -> Vec<*mut AvlNode> {
                    let mut inner_ptrs = Vec::<*mut AvlNode>::new();
                    let mut current = tree;
                    while let Some(node) = current {
                        if node.left.is_some() {
                            inner_ptrs.push(&mut **node);
                        }
                        current = &mut node.left
                    }
                    inner_ptrs
                }

                fn safely_replace_value(value: &mut i64, leftmost_parent: &mut AvlNode) -> i64 {
                    let mut leftmost_node = leftmost_parent.left.take().unwrap();
                    let value = replace(value, leftmost_node.value);
                    leftmost_parent.left = leftmost_node.right.take();
                    leftmost_parent.update_height();
                    leftmost_parent.rebalance();
                    value
                }

                let mut inner_ptrs = traverse_to_leftmost(&mut target.right);
                let leftmost_parent = unsafe { &mut *inner_ptrs.pop().unwrap() };
                let value = safely_replace_value(&mut target.value, leftmost_parent);
                update_height_rebalance(inner_ptrs);
                target.update_height();
                target.rebalance();
                value
            }

            if tree_has_no_left_children(&target.right) {
                raise_right_tree(target)
            } else {
                swap_with_leftmost_node_from_right_tree(target)
            }
        }

        let mut prev_ptrs = Vec::<*mut AvlNode>::new();
        let target = find_target(value, &mut self.root, &mut prev_ptrs)?;
        let taken_value = if has_zero_or_one_child(target) {
            try_take_value_when_zero_or_one_child(target, prev_ptrs.pop())
                .unwrap_or_else(|| self.root.take().unwrap().value) // None means to remove root of the tree
        } else {
            take_value_when_two_children(target)
        };
        update_height_rebalance(prev_ptrs);
        Some(taken_value)
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
fn update_height_rebalance(prev_ptrs: Vec<*mut AvlNode>) {
    for ptr in prev_ptrs.into_iter().rev() {
        let node = unsafe { &mut *ptr };
        node.update_height();
        node.rebalance();
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use std::{path::Path, time::Instant};

    use crate::tester::run_test;

    use super::*;

    #[test]
    fn avl_sort_random() {
        let base = Path::new("..");
        perform_sort_test(base.join("sorting-tests").join("0.random"));
    }

    fn perform_sort_test(path: impl AsRef<Path>) {
        run_test(path, |data| {
            let mut tree = AvlSet::default();
            let start = Instant::now();
            let mut n = 0;
            for number in data[1].split(' ').filter_map(|x| x.parse::<i64>().ok()) {
                tree.insert(number);
                n += 1;
            }
            let elapsed = Instant::now().duration_since(start);
            println!("Inserted {n} numbers in {elapsed:?}");
            tree.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
    }
}
