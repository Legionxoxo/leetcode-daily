pub struct Solution;

impl Solution {
    pub fn run(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let mut dp = vec![0; n];

        let mut ans = std::i32::MIN;

        for i in (n - 1).rev() {
            if i + k < n {
                dp[i] = energy[i] + dp[i + k];
            } else {
                dp[i] = energy[i] + 0;
            }
        }
    }
}
