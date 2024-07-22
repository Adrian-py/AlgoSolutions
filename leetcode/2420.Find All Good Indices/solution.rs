impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut left_order: Vec<i32> = vec![1; nums.len()];
        let mut right_order: Vec<i32> = vec![1; nums.len()];
        for i in 1..n - 1 {
            if nums[i - 1] >= nums[i] {
                left_order[i] = left_order[i - 1] + 1;
            }
            if nums[n - i] >= nums[n - i - 1] {
                right_order[n - i - 1] = right_order[n - i] + 1;
            }
        }
        let mut ans: Vec<i32> = Vec::new();
        for i in 1..n - 1 {
            if left_order[i - 1] >= k && right_order[i + 1] >= k {
                ans.push(i as i32);
            }
        }
        ans
    }
}
