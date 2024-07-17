impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if mid != 0 && mid != nums.len() - 1 {
                if nums[mid - 1] < nums[mid] && nums[mid] > nums[mid + 1] {
                    return mid as i32;
                } else if nums[mid + 1] > nums[mid] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else if mid == 0 {
                if nums[mid + 1] > nums[mid] {
                    return mid as i32 + 1;
                }
                return mid as i32;
            } else if mid == nums.len() - 1 {
                if nums[mid - 1] > nums[mid] {
                    return mid as i32 - 1;
                }
                return mid as i32;
            }
        }

        left as i32
    }
}
