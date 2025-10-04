struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_bottles = num_bottles;
        while num_bottles >= num_exchange {
            total_bottles += num_bottles / num_exchange;
            num_bottles = (num_bottles / num_exchange) + (num_bottles % num_exchange)
        }
        total_bottles
    }
}

fn main() {
    let result1 = Solution::num_water_bottles(9, 3);
    println!("Test 1 -> {}", result1); // Expected 13

    let result2 = Solution::num_water_bottles(15, 4);
    println!("Test 2 -> {}", result2); // Expected 19
}
