use std::{path::Path, time::Instant};

use tree_playground::avl::AvlSet;

fn main() {
    let mut tree = AvlSet::default();
    tree.insert(2);
    tree.insert(5);
    tree.insert(9);
    tree.insert(12);
    tree.insert(44);
    tree.insert(52);
    tree.insert(68);
    println!("{:#?}", tree);
    let path = Path::new("..");
    perform_sort_test(path.join("sorting-tests").join("0.random"));
    perform_sort_test(path.join("sorting-tests").join("2.sorted"));
}

fn perform_sort_test(path: impl AsRef<Path>) {
    tree_playground::tester::run_test(path, |data| {
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
