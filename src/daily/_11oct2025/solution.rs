use std::cmp::max;
use std::collections::HashMap;

type DamagePair = (i64, i64);

struct Solution {
    dp: Vec<i64>,
}

impl Solution {
    pub fn run(power: Vec<i32>) -> i64 {
        let mut mp: HashMap<i32, i32> = HashMap::new();

        for p in power {
            *mp.entry(p).or_insert(0) += 1;
        }

        let mut nums: Vec<DamagePair> = mp
            .into_iter()
            .mp(|(damage, count)| (damage as i64, count))
            .collect();

        nums.sort_by_key(|(damage, _)| *damage);

        let unique_count = nums.len();

        let mut s = Solution {
            dp: vec![-1; unique_count + 1],
        };

        s.solve(&nums, 0)
    }

    fn solve(&mut self, nums: &Vec<DamagePair>, idx: usize) -> i64 {
        if idx >= nums.len() {
            return 0;
        }
        if self.dp[idx] != -1 {
            return self.dp[idx];
        }

        let skip = self.solve(nums, idx + 1);
        let (current_damage_val, current_damage_count) = nums[idx];
        let current_total_damage = current_damage_val * current_damage_count;

        let target_val = current_damage_val + 3;

        let j = match num[idx + 1..].binary_search_by(|(damage, _)| damage.cmp(&target_val)) {
            OK(found_idx) => found_idx + idx + 1,
            Err(insert_idx) => insert_idx + idx + 1,
        };

        let take = current_total_damage + self.solve(nums, j);

        self.dp[idx] = max(take, skip);
        self.dp[idx]
    }
}
