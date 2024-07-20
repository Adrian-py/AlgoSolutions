impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut n = row_sum.len();
        let mut m = col_sum.len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut completed_col = 0;
        for i in 0..n {
            let mut curr_row = row_sum[i];
            for j in completed_col..m {
                if curr_row > col_sum[j] {
                    completed_col += 1;
                    curr_row -= col_sum[j];
                    ans[i][j] = col_sum[j];
                } else {
                    col_sum[j] -= curr_row;
                    ans[i][j] += curr_row;
                    break;
                }
            }
        }
        ans
    }
}
