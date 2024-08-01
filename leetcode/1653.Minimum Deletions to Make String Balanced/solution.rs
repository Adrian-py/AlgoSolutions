impl Solution {
    pub fn minimum_deletions(mut s: String) -> i32 {
        let mut num_of_b: i32 = 0;
        let mut num_of_a: i32 = 0;
        let mut num_of_del: i32 = i32::MAX;

        s.push('b');
        for i in s.chars() {
            if i == 'a' {
                num_of_a += 1;
            }
        }

        for i in s.chars() {
            num_of_del = num_of_del.min(num_of_b + num_of_a);
            if i == 'a' {
                num_of_a -= 1;
            } else if i == 'b' {
                num_of_b += 1;
            }
        }

        num_of_del
    }
}
