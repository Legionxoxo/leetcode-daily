pub mod Solution;

impl Solution {
    pub fn run(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let m = mana.len();

        let mut done = vec![0; n + 1];

        for j in 0..m {
            for i in 0..n {
                done[i + 1] = done[i + 1].max(done[i]) + mana[j] * skill[i];
            }
            for i in (n - 1).rev() {
                done[i] = done[i + 1] - mana[j] * skill[i];
            }
        }
        return done[n];
    }
}
