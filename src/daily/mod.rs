pub mod _05oct2025;

use std::collections::HashMap;

// Define type: a function that returns i32
pub type SolutionFn = fn() -> Vec<Vec<i32>>;

// Wrapper function for each day
fn _05oct2025_wrapper() -> Vec<Vec<i32>> {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    _05oct2025::solution::Solution::run(heights)
}

// Register all solutions
pub fn all_solutions() -> HashMap<&'static str, SolutionFn> {
    let mut map: HashMap<&str, SolutionFn> = HashMap::new();
    map.insert("05oct2025", _05oct2025_wrapper);
    map
}
