pub mod avl;
pub mod bst;
mod play;
pub mod tester;
pub mod treap;

pub use play::play_random_numbers;
pub use play::play_sorted_numbers;
pub use play::sorted_percent;

pub trait Tree {
    fn insert(&mut self, value: i64) -> bool;
    fn remove(&mut self, value: i64) -> Option<i64>;
    fn search(&self, value: i64) -> bool;
}
