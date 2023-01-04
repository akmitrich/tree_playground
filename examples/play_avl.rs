use std::{path::Path, time::Instant};

use tree_playground::{
    avl::AvlSet, play_random_numbers, play_sorted_numbers, sorted_percent, Tree,
};

fn main() {
    println!("{}", "=".repeat(80));
    println!("START PLAY WITH AVL TREE");
    for n in [1000_usize, 10000, 100000, 1000000, 10000000] {
        let mut random = AvlSet::default();
        play_random_numbers(&mut random, n);
        println!(
            "Tree is {}% sorted.",
            sorted_percent(random.iter().copied())
        );
        println!();
        let mut sorted = AvlSet::default();
        play_sorted_numbers(&mut sorted, n);
        println!(
            "Tree is {}% sorted.",
            sorted_percent(sorted.iter().copied())
        );

        println!("{}", "-".repeat(80));
    }
}

fn _perform_sort_test(path: impl AsRef<Path>) {
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
