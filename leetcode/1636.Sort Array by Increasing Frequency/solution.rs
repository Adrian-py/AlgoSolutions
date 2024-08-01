use std::collections::HashMap;
impl Solution {
    pub fn merge_sort(arr: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        if arr.len() == 1 {
            return arr.to_vec();
        }

        let mid_point = arr.len() / 2;
        let left_arr = Self::merge_sort(arr[0..mid_point].to_vec());
        let right_arr = Self::merge_sort(arr[mid_point..arr.len()].to_vec());
        let final_arr = Self::merge(&left_arr, &right_arr);
        final_arr
    }
    pub fn merge(first: &Vec<(i32, i32)>, second: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut f_pointer = 0;
        let mut s_pointer = 0;
        let mut res = Vec::new();
        while f_pointer < first.len() && s_pointer < second.len() {
            if first[f_pointer].1 < second[s_pointer].1 {
                res.push(first[f_pointer]);
                f_pointer += 1;
            } else if first[f_pointer].1 == second[s_pointer].1 {
                if first[f_pointer].0 > second[s_pointer].0 {
                    res.push(first[f_pointer]);
                    f_pointer += 1;
                } else {
                    res.push(second[s_pointer]);
                    s_pointer += 1;
                }
            } else {
                res.push(second[s_pointer]);
                s_pointer += 1;
            }
        }

        while f_pointer < first.len() {
            res.push(first[f_pointer]);
            f_pointer += 1;
        }

        while s_pointer < second.len() {
            res.push(second[s_pointer]);
            s_pointer += 1;
        }
        res.to_vec()
    }
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums.iter() {
            let mut curr = *map.get(i).unwrap_or(&0);
            map.insert(*i, curr + 1);
        }
        let sorted = Self::merge_sort(map.into_iter().collect::<Vec<(i32, i32)>>());
        let mut ans: Vec<i32> = Vec::new();
        for i in sorted.iter() {
            for j in 0..i.1 {
                ans.push(i.0);
            }
        }
        ans
    }
}
