impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let row_topological = Self::topological_sort(&row_conditions, k as usize);
        let col_topological = Self::topological_sort(&col_conditions, k as usize);
        if row_topological.len() == 0 || col_topological.len() == 0 {
            return Vec::new();
        }
        let mut ans_matrix: Vec<Vec<i32>> = vec![vec![0; k as usize]; k as usize];
        for i in 0..k {
            for j in 0..k {
                if row_topological[i as usize] == col_topological[j as usize] {
                    ans_matrix[i as usize][j as usize] = row_topological[i as usize];
                    break;
                }
            }
        }
        ans_matrix
    }
    pub fn topological_sort(edges: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut visited = vec![0; k + 1];
        let mut adjacency_list: Vec<Vec<i32>> = vec![Vec::new(); k + 1];
        for e in edges.iter() {
            adjacency_list[e[0] as usize].push(e[1]);
        }

        let mut has_cycle = false;
        let mut topological_order = Vec::new();
        for i in 1..=k {
            if visited[i] == 0 {
                Self::dfs(
                    i,
                    &adjacency_list,
                    &mut visited,
                    &mut has_cycle,
                    &mut topological_order,
                );
                if has_cycle {
                    return Vec::new();
                }
            }
        }
        topological_order.into_iter().rev().collect::<Vec<i32>>()
    }
    pub fn dfs(
        curr: usize,
        edges: &Vec<Vec<i32>>,
        visited: &mut Vec<i32>,
        has_cycle: &mut bool,
        order: &mut Vec<i32>,
    ) {
        visited[curr] = 1;
        for i in edges[curr].iter() {
            if visited[*i as usize] == 0 {
                Self::dfs(*i as usize, edges, visited, has_cycle, order);
                if *has_cycle {
                    return;
                }
            } else if visited[*i as usize] == 1 {
                *has_cycle = true;
                return;
            }
        }
        visited[curr] = 2;
        order.push(curr as i32);
    }
}
