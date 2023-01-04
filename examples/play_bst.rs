use std::{path::Path, time::Instant};

use tree_playground::{bst::BstSet, play_random_numbers, play_sorted_numbers, Tree};

fn main() {
    println!("{}", "=".repeat(80));
    println!("START PLAY WITH BINARY SEARCH TREE");
    for n in [1000_usize, 10000, 100000, 1000000, 10000000] {
        let mut random = BstSet::default();
        play_random_numbers(&mut random, n);
        if n < 100001 {
            println!();
            let mut sorted = BstSet::default();
            play_sorted_numbers(&mut sorted, n);
        }
        println!("{}", "-".repeat(80));
    }
}

fn _perform_sort_test(path: impl AsRef<Path>) {
    tree_playground::tester::run_test(path, |data| {
        let mut tree = BstSet::default();
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
    });
}
