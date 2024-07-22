impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people: Vec<(String, i32)> = Vec::new();
        for i in 0..names.len() {
            people.push((names[i].to_string(), heights[i]));
        }
        people.sort_unstable_by_key(|x| -x.1);
        people.into_iter().map(|p| p.0).collect::<Vec<String>>()
    }
}
