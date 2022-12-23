fn main() {
    let mut tree = tree_playground::avl::AvlSet::default();
    println!("Empty: {tree:?}");
    tree.insert(42);
    tree.insert(15);
    tree.insert(23);
    tree.insert(55);
    println!("Final: {tree:#?}");
}