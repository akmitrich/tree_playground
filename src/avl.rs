#[derive(Debug, Clone, PartialEq)]
struct AvlNode {
    value: i64,
    left: AvlTree,
    right: AvlTree,
}
type AvlTree = Option<Box<AvlNode>>;

impl AvlNode {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AvlSet {
    root: AvlTree,
}

impl AvlSet {
    pub fn insert(&mut self, value: i64) -> bool {
        let mut current = &mut self.root;
        while let Some(node) = current {
            match node.value.cmp(&value) {
                std::cmp::Ordering::Less => current = &mut node.right,
                std::cmp::Ordering::Equal => return false,
                std::cmp::Ordering::Greater => current = &mut node.left,
            }
        }
        *current = Some(Box::new(AvlNode::new(value)));
        true
    }
}
