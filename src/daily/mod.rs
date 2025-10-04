pub mod _04oct2025;

use std::collections::HashMap;

// Define type: a function that returns i32
pub type SolutionFn = fn() -> i32;

// Wrapper function for each day
fn _04oct2025_wrapper() -> i32 {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7]; // inputs
    _04oct2025::solution::Solution::run(height)
}

// Register all solutions
pub fn all_solutions() -> HashMap<&'static str, SolutionFn> {
    let mut map: HashMap<&str, SolutionFn> = HashMap::new();
    map.insert("04oct2025", _04oct2025_wrapper);
    map
}
