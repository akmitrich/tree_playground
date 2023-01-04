use std::cmp::Ordering;

use rand::Rng;
pub type Treap = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    value: i64,
    priority: usize,
    size: usize,
    left: Treap,
    right: Treap,
}

impl Node {
    pub fn new(value: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            value,
            priority: rng.gen(),
            size: 1,
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

pub fn insert(value: i64, t: Treap) -> Treap {
    let node = Some(Box::new(Node::new(value)));
    let (left, right) = split(t, value);
    let add_node_to_left = merge(left, node);
    merge(add_node_to_left, right)
}

pub fn remove(value: i64, t: Treap) -> Treap {
    let mut root = t?;
    match root.value.cmp(&value) {
        Ordering::Less => root.right = remove(value, root.right),
        Ordering::Equal => return merge(root.left, root.right),
        Ordering::Greater => root.left = remove(value, root.left),
    }
    Some(root)
}
