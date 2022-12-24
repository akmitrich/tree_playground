use std::{cmp::Ordering, mem::replace};

type BSTree = Option<Box<BstNode>>;

#[derive(Debug, Clone, PartialEq)]
struct BstNode {
    value: i64,
    left: BSTree,
    right: BSTree,
}

impl BstNode {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BstSet {
    root: BSTree,
}

impl BstSet {
    pub fn insert(&mut self, value: i64) -> bool {
        let mut current = &mut self.root;
        while let Some(node) = current {
            match node.value.cmp(&value) {
                Ordering::Less => current = &mut node.right,
                Ordering::Equal => return false,
                Ordering::Greater => current = &mut node.left,
            }
        }
        *current = Some(Box::new(BstNode::new(value)));
        true
    }

    pub fn search(&self, value: i64) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match node.value.cmp(&value) {
                Ordering::Less => current = &node.right,
                Ordering::Equal => return true,
                Ordering::Greater => current = &node.left,
            }
        }
        false
    }

    pub fn remove(&mut self, value: i64) -> Option<i64> {
        fn find_target<'a>(
            value: i64,
            start: &'a mut BSTree,
            prev_ptrs: &mut Vec<*mut BstNode>,
        ) -> Option<&'a mut BstNode> {
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

        fn zero_or_one_child(target: &BstNode) -> bool {
            target.left.is_none() || target.right.is_none()
        }

        fn take_value_when_zero_or_one_child(
            target: &mut BstNode,
            prev: Option<*mut BstNode>,
        ) -> Option<i64> {
            fn take_value_from_parent(target_value: i64, parent: &mut BstNode) -> i64 {
                match parent.left {
                    None => parent.right.take().unwrap().value,
                    Some(ref left) => {
                        if left.value == target_value {
                            parent.left.take().unwrap().value
                        } else {
                            parent.right.take().unwrap().value
                        }
                    }
                }
            }

            let value = if let Some(left) = target.left.take() {
                replace(target, *left).value
            } else if let Some(right) = target.right.take() {
                replace(target, *right).value
            } else {
                match prev {
                    None => return None, // we are going to remove root of the BstSet
                    Some(parent_ptr) => {
                        let parent = unsafe { &mut *parent_ptr };
                        take_value_from_parent(target.value, parent)
                    }
                }
            };
            Some(value)
        }

        fn take_value_when_two_children(target: &mut BstNode) -> i64 {
            fn right_node_has_no_left_child(right_tree: &BSTree) -> bool {
                right_tree.as_ref().unwrap().left.is_none()
            }

            fn move_right_tree_up(target: &mut BstNode) -> i64 {
                let right_tree = &mut target.right;
                let right = right_tree.take().unwrap();
                target.right = right.right;
                replace(&mut target.value, right.value)
            }

            fn swap_with_leftmost_and_take(target: &mut BstNode) -> i64 {
                let mut next = &mut target.right;
                let mut inner_ptrs = Vec::<*mut BstNode>::new();
                while let Some(next_left) = next {
                    if next_left.left.is_some() {
                        inner_ptrs.push(&mut **next_left);
                    }
                    next = &mut next_left.left;
                }
                let parent_left = unsafe { &mut *inner_ptrs.pop().unwrap() };
                let leftmost = parent_left.left.take().unwrap();
                parent_left.left = leftmost.right;
                replace(&mut target.value, leftmost.value)
            }

            if right_node_has_no_left_child(&target.right) {
                move_right_tree_up(target)
            } else {
                swap_with_leftmost_and_take(target)
            }
        }

        let mut prev_ptrs = Vec::<*mut BstNode>::new();
        let target = find_target(value, &mut self.root, &mut prev_ptrs)?;
        let taken_value = if zero_or_one_child(target) {
            match take_value_when_zero_or_one_child(target, prev_ptrs.pop()) {
                Some(value) => value,
                None => self.root.take().unwrap().value, // do remove root of the BstSet
            }
        } else {
            take_value_when_two_children(target)
        };
        Some(taken_value)
    }
}

impl<'a> BstSet {
    pub fn iter(&'a self) -> BstIter<'a> {
        BstIter {
            prev_nodes: Vec::new(),
            current: &self.root,
        }
    }
}

#[derive(Debug)]
pub struct BstIter<'a> {
    prev_nodes: Vec<&'a BstNode>,
    current: &'a BSTree,
}

impl<'a> Iterator for BstIter<'a> {
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
    use super::*;

    #[test]
    fn test_insert() {
        let mut tree = BstSet::default();
        tree.insert(42);
        tree.insert(15);
        tree.insert(23);
        tree.insert(55);
        let sorted = tree.iter().copied().collect::<Vec<i64>>();
        assert_eq!(vec![15, 23, 42, 55], sorted);
    }

    #[test]
    fn test_search() {
        let tree = prepare_minimal_tree();
        assert!(tree.search(42));
        assert!(tree.search(23));
        assert!(!tree.search(111));
        assert!(!tree.search(-1));
    }

    #[test]
    fn test_delete() {
        let mut tree = prepare_tree();
        assert_eq!(Some(96), tree.remove(96));
        tree = prepare_tree();
        tree.remove(15).unwrap();
        assert!(tree.remove(144).is_none());
        println!("{tree:#?}");
    }

    fn prepare_minimal_tree() -> BstSet {
        let mut tree = BstSet::default();
        tree.insert(42);
        tree.insert(15);
        tree.insert(23);
        tree.insert(55);
        tree
    }

    fn prepare_tree() -> BstSet {
        let mut tree = BstSet::default();
        tree.insert(42);
        tree.insert(15);
        tree.insert(21);
        tree.insert(55);
        tree.insert(48);
        tree.insert(25);
        tree.insert(18);
        tree.insert(12);
        tree.insert(8);
        tree.insert(14);
        tree.insert(96);
        tree
    }
}
