struct Solution;

impl Solution {
    pub fn num_water_bottles_ii(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut total_bottles = num_bottles;

        while num_bottles >= num_exchange {
            // how many new bottles we can get with current exchange rate
            num_bottles -= num_exchange;
            total_bottles += 1;
            num_bottles += 1; // drink the new bottle, adds empty

            num_exchange += 1; // increase cost for next exchange
        }
        total_bottles
    }
}

fn main() {
    let result1 = Solution::num_water_bottles_ii(13, 6);
    println!("Test 1 -> {}", result1);

    let result2 = Solution::num_water_bottles_ii(10, 3);
    println!("Test 2 -> {}", result2);
}
