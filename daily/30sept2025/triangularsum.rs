struct Solution;

impl Solution{
    pub fn triangular_sum(mut nums:Vec<i32>)->  i32{
        let n = nums.len();

        for i in (1..n).rev(){
            for j in 0..1 {
                nums[j] = (nums[j] + nums[j+1])%10;
            }
        }
        return nums[0];
    }
}

fn main(){
    let nums = vec![1,2,3,4,5];
    let result = Solution::triangular_sum(nums);
    println!("triangular sum = {}", result);
}