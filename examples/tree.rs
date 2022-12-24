use std::path::Path;

use tree_playground::bst::BstSet;

fn main() {
    let path = Path::new("..");
    tree_playground::tester::run_test(path.join("sorting-tests").join("0.random"), |data| {
        let mut tree = BstSet::default();
        for number in data[1].split(' ').filter_map(|x| x.parse::<i64>().ok()) {
            tree.insert(number);
        }
        tree.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    })
}
