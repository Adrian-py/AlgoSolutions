impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut num_of_a: i32 = 0;
        for i in s.chars() {
            if i == 'a' {
                num_of_a += 1;
            }
        }

        for i in s.chars() {
            if i == 'b' && num_of_a > 0 {
                return false;
            } else if i == 'a' {
                num_of_a -= 1;
            }
        }
        true
    }
}
