impl Solution {
    pub fn min_height_shelves(mut books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; books.len() + 1];
        dp[0] = 0;
        dp[1] = books[0][1];

        for i in 0..books.len() {
            let mut rem_width = shelf_width - books[i][0];
            let mut curr_max_height = books[i][1];
            dp[i + 1] = dp[i] + curr_max_height; // if add new shelf;

            if i < 1 {
                continue;
            }

            // check if a combination with previous books is possible so that no new shelves is necessary
            let mut j = i - 1;
            while j >= 0 && rem_width - books[j][0] >= 0 {
                rem_width -= books[j][0];
                curr_max_height = curr_max_height.max(books[j][1]);
                dp[i + 1] = dp[i + 1].min(dp[j] + curr_max_height);

                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }

        dp[books.len()]
    }
}
