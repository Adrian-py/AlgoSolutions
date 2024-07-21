impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjacency_list: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
        for i in prerequisites.iter() {
            adjacency_list[i[0] as usize].push(i[1]);
        }
        let mut has_cycle = false;
        let mut visited: Vec<i32> = vec![0; num_courses as usize];
        for i in 0..num_courses {
            if visited[i as usize] == 0 {
                Self::dfs(i, &adjacency_list, &mut visited, &mut has_cycle);
                i
            }
        }
        !has_cycle
    }
    pub fn dfs(curr: i32, edges: &Vec<Vec<i32>>, visited: &mut Vec<i32>, has_cycle: &mut bool) {
        visited[curr as usize] = 1;
        for i in edges[curr as usize].iter() {
            if visited[*i as usize] == 0 {
                Self::dfs(*i, edges, visited, has_cycle);
                if *has_cycle {
                    return;
                }
            } else if visited[*i as usize] == 1 {
                *has_cycle = true;
                return;
            }
        }
        visited[curr as usize] = 2;
    }
}
