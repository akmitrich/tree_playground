use tree_playground::{play_random_numbers, play_sorted_numbers, sorted_percent, treap::TreapSet};

fn main() {
    println!("{}", "=".repeat(80));
    println!("START PLAY WITH TREAP");
    for n in [1000_usize, 10000, 100000, 1000000, 10000000] {
        let mut random = TreapSet::default();
        play_random_numbers(&mut random, n);
        println!(
            "Tree is {}% sorted.",
            sorted_percent(random.iter().copied())
        );
        println!();
        let mut sorted = TreapSet::default();
        play_sorted_numbers(&mut sorted, n);
        println!(
            "Tree is {}% sorted.",
            sorted_percent(sorted.iter().copied())
        );

        println!("{}", "-".repeat(80));
    }
}
