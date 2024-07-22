impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    if i == 0 {
                        ans += 1;
                    }
                    if i == grid.len() - 1 {
                        ans += 1;
                    }
                    if j == 0 {
                        ans += 1;
                    }
                    if j == grid[i].len() - 1 {
                        ans += 1;
                    }

                    if i > 0 && grid[i - 1][j] == 0 {
                        ans += 1;
                    }

                    if j > 0 && grid[i][j - 1] == 0 {
                        ans += 1;
                    }

                    if i < grid.len() - 1 && grid[i + 1][j] == 0 {
                        ans += 1;
                    }

                    if j < grid[i].len() - 1 && grid[i][j + 1] == 0 {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
