use tree_playground::bst::BstSet;

fn main() {
    let mut tree = BstSet::default();
    println!("Empty: {tree:?}");
    tree.insert(42);
    tree.insert(15);
    tree.insert(23);
    tree.insert(55);
    println!("Final: {tree:#?}");
    println!("Sorted: {:?}", tree.iter().collect::<Vec<&i64>>());
}
