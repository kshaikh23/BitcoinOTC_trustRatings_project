// Module for functions that I made that didn't end up being used in project, but may be useful in future iterations of the project
mod unused_functions {
    use std::collections::HashSet;
    
    // Returns number of nodes in the graph dataset
    pub fn node_count(data: &Vec<(i32, i32, i32, f64)>) -> usize {
        let mut nodes = HashSet::new();
        for &(u, v, _, _) in data {
            nodes.insert(u);
            nodes.insert(v);
        }
        return nodes.len()
    }
}