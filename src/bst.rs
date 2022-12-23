use std::cmp::Ordering;

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
