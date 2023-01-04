use std::cmp::Ordering;

use rand::Rng;

use crate::Tree;
type Treap = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    value: i64,
    priority: usize,
    left: Treap,
    right: Treap,
}

impl Node {
    pub fn new(value: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            value,
            priority: rng.gen(),
            left: None,
            right: None,
        }
    }
}

fn merge(t1: Treap, t2: Treap) -> Treap {
    match (t1, t2) {
        (None, t) | (t, None) => t,
        (Some(mut left), Some(mut right)) => {
            if left.priority > right.priority {
                left.right = merge(left.right, Some(right));
                Some(left)
            } else {
                right.left = merge(Some(left), right.left);
                Some(right)
            }
        }
    }
}

fn split(t: Treap, value: i64) -> (Treap, Treap) {
    match t {
        Some(mut root) => {
            if root.value <= value {
                let (left, right) = split(root.right, value);
                root.right = left;
                (Some(root), right)
            } else {
                let (left, right) = split(root.left, value);
                root.left = right;
                (left, Some(root))
            }
        }
        None => (None, None),
    }
}

fn insert(value: i64, t: Treap) -> Treap {
    let node = Some(Box::new(Node::new(value)));
    let (left, right) = split(t, value);
    merge(merge(left, node), right)
}

fn remove(value: i64, t: Treap) -> Treap {
    let mut root = t?;
    match root.value.cmp(&value) {
        Ordering::Less => root.right = remove(value, root.right),
        Ordering::Equal => return merge(root.left, root.right),
        Ordering::Greater => root.left = remove(value, root.left),
    }
    Some(root)
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TreapSet {
    root: Treap,
}

impl Tree for TreapSet {
    fn insert(&mut self, value: i64) -> bool {
        self.root = insert(value, self.root.take());
        true
    }

    fn remove(&mut self, value: i64) -> Option<i64> {
        self.root = remove(value, self.root.take());
        Some(value)
    }

    fn search(&self, value: i64) -> bool {
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
}

impl TreapSet {
    pub fn iter(&self) -> TreapIter {
        TreapIter {
            prev_nodes: vec![],
            current: &self.root,
        }
    }
}

#[derive(Debug)]
pub struct TreapIter<'a> {
    prev_nodes: Vec<&'a Node>,
    current: &'a Treap,
}

impl<'a> Iterator for TreapIter<'a> {
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
