use tree_playground::treap::insert;

fn main() {
    let mut tree = None;
    for i in -5..6 {
        tree = insert(i * 2, tree);
    }
    println!("{:#?}", tree);
}
