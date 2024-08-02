impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut group_length: i32 = 0;
        for i in nums.iter() {
            group_length += *i;
        }

        let mut ones_before: Vec<i32> = vec![0; nums.len()];
        let mut left = 0;
        let mut right = 0;
        let mut curr_ones = 0;
        while right < nums.len() {
            if (right - left) as i32 == group_length {
                curr_ones -= nums[left];
                left += 1;
            }
            curr_ones += nums[right];
            ones_before[right] = curr_ones;
            right += 1;
        }

        for i in left + 1..nums.len() {
            curr_ones -= nums[i - 1];
            curr_ones += nums[i - left - 1];
            ones_before[i - left - 1] = curr_ones;
        }
        let mut ans = i32::MIN;
        for i in ones_before.iter() {
            ans = ans.max(*i);
        }
        group_length - ans
    }
}
