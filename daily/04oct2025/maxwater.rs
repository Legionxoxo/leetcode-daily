use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = height.len() - 1;
        let mut ans: i32 = 0;

        while i < j {
            ans = max(ans, ((j - i) as i32) * min(height[i], height[j]));
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        ans
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

    let result1 = Solution::max_area(height);
    println!("The max water contained is {}", result1); //49
}

//unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_water() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }
}
