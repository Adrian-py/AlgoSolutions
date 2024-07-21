impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited: Vec<i32> = vec![0; num_courses as usize];
        let mut adj_list: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
        let mut order: Vec<i32> = Vec::new();
        let mut has_cycle: bool = false;
        for i in prerequisites.iter() {
            adj_list[i[0] as usize].push(i[1]);
        }
        for i in 0..num_courses {
            if visited[i as usize] == 0 {
                Self::dfs(i, &mut order, &adj_list, &mut visited, &mut has_cycle);
                if has_cycle {
                    return Vec::new();
                }
            }
        }
        order
    }

    pub fn dfs(
        curr: i32,
        order: &mut Vec<i32>,
        edges: &Vec<Vec<i32>>,
        visited: &mut Vec<i32>,
        has_cycle: &mut bool,
    ) {
        visited[curr as usize] = 1;
        for i in edges[curr as usize].iter() {
            if visited[*i as usize] == 0 {
                Self::dfs(*i, order, edges, visited, has_cycle);
                if *has_cycle {
                    return;
                }
            } else if visited[*i as usize] == 1 {
                *has_cycle = true;
                return;
            }
        }
        visited[curr as usize] = 2;
        order.push(curr);
    }
}
