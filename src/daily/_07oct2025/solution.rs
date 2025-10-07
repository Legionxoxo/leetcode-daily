use std::collections::{BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn run(rains: Vec<i32>) -> Vec<i32> {
        let mut n = rains.len();
        let mut ans = vec![1; n];

        let mut lakes = HashMap::new();
        let mut dry_days = BTreeSet::new();

        for i in 0..n {
            let rain = rains[i];

            if rain == 0 {
                dry_days.insert(i);
            } else {
                ans[i] = -1;

                if let Some(&last_rain_day) = lakes.get(&rain) {
                    if let Some(&dry_day) = dry_days.range((last_rain_day as usize + 1)..).next() {
                        ans[dry_day] = rain;
                        dry_days.remove(&dry_day);
                    } else {
                        return vec![];
                    }
                }
            }
            lakes.insert(rain, i as i32)
        }
    }
    ans
}
