impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_in_col : Vec<i32> = vec![i32::MIN; matrix[0].len()];
        let mut min_in_row : Vec<i32> = vec![i32::MAX; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                min_in_row[i] = min_in_row[i].min(matrix[i][j]);
                max_in_col[j] = max_in_col[j].max(matrix[i][j]);
            }
        }
        let mut lucky_numbers : Vec<i32> = Vec::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == min_in_row[i] && matrix[i][j] == max_in_col[j] {
                    lucky_numbers.push(matrix[i][j]);
                }
            }
        }
        lucky_numbers
    }
}