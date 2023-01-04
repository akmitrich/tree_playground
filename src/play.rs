use std::time::Instant;

use rand::Rng;

use crate::Tree;

pub fn play_random_numbers(tree: &mut dyn Tree, n: usize) {
    let mut rng = rand::thread_rng();
    let start_insert_random_numbers = Instant::now();
    let mut inserted = 0;
    for _ in 0..n {
        if tree.insert(rng.gen_range(0..(3 * n)).try_into().unwrap()) {
            inserted += 1;
        }
    }
    println!(
        "Inserted {inserted} out of {n} random numbers in {:?}",
        Instant::now().duration_since(start_insert_random_numbers)
    );
    let start_search = Instant::now();
    let mut found = 0;
    for _ in 0..n / 10 {
        if tree.search(rng.gen_range(0..(3 * n)).try_into().unwrap()) {
            found += 1;
        }
    }
    println!(
        "Searched for {} numbers in {:?}. Found {found}.",
        n / 10,
        Instant::now().duration_since(start_search)
    );
    let start_remove = Instant::now();
    let mut removed = 0;
    for _ in 0..n / 10 {
        if tree
            .remove(rng.gen_range(0..(3 * n)).try_into().unwrap())
            .is_some()
        {
            removed += 1;
        }
    }
    println!(
        "Removed {} numbers in {:?}. Deleted {removed}.",
        n / 10,
        Instant::now().duration_since(start_remove)
    );
}

pub fn play_sorted_numbers(tree: &mut dyn Tree, n: usize) {
    let mut rng = rand::thread_rng();
    let start_insert_sorted_numbers = Instant::now();
    for x in 0..n {
        tree.insert(x as _);
    }
    println!(
        "Inserted {n} sorted numbers in {:?}",
        Instant::now().duration_since(start_insert_sorted_numbers)
    );
    let start_search = Instant::now();
    let mut found = 0;
    for _ in 0..n / 10 {
        if tree.search(rng.gen_range(0..n).try_into().unwrap()) {
            found += 1;
        }
    }
    println!(
        "Searched for {} numbers in {:?}. Found {found}.",
        n / 10,
        Instant::now().duration_since(start_search)
    );
    let start_remove = Instant::now();
    let mut removed = 0;
    for _ in 0..n / 10 {
        let to_delete = rng.gen_range(0..n).try_into().unwrap();
        let result = tree.remove(to_delete);
        if result.is_some() {
            assert_eq!(to_delete, result.unwrap());
            removed += 1;
        }
    }
    println!(
        "Removed {} numbers in {:?}. Deleted {removed}.",
        n / 10,
        Instant::now().duration_since(start_remove)
    );
}

pub fn sorted_percent(mut data: impl Iterator<Item = i64>) -> usize {
    let start = data.next().unwrap();
    let (size, success, _) = data.fold((0_usize, 0_usize, start), |(sum, ok, prev), current| {
        (sum + 1, if prev > current { ok } else { ok + 1 }, current)
    });
    if size > 0 {
        success * 100 / size
    } else {
        100
    }
}
