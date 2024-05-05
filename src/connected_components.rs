pub mod connected_components {
    use std::collections::HashMap;

    // Ignoring direction and not doing strongly CC as I don't care about direction to find communities
    // Uses trees to track each connected component
    pub struct ConnectedComponents {
        // Holds parent of each node, root nodes are where index and value are the same
        parent: Vec<usize>,

        // Measure to help keep each tree as flat as possible
        rank: Vec<usize>,

        // Stores the amount of nodes in each component for each root node
        size: Vec<usize>,
    }

    impl ConnectedComponents {
        // Takes n which is the amount of nodes in a dataset
        fn new(n: usize) -> Self {
            ConnectedComponents {
                parent: (0..n).collect(), 
                rank: vec![0; n],
                size: vec![1; n],
            }
        }
    
        // To find the root node of a node, recursively calls and makes every node along the way point to the root node, reducing the height of the tree
        fn find_root(&mut self, u: usize) -> usize {
            if self.parent[u] != u {
                self.parent[u] = self.find_root(self.parent[u]);
            }
            return self.parent[u]; 
        }
    
        // To merge two components that connect between the nodes u and v
        fn merge(&mut self, u: usize, v: usize) {
            let root_u = self.find_root(u);
            let root_v = self.find_root(v); 
            if root_u != root_v {
                if self.rank[root_u] > self.rank[root_v] {
                    self.parent[root_v] = root_u;
                    self.size[root_u] += self.size[root_v];
                } else {
                    self.parent[root_u] = root_v;
                    self.size[root_v] += self.size[root_u];
                    if self.rank[root_u] == self.rank[root_v] {
                        self.rank[root_v] += 1;
                    } 
                }
            }
        }
    }

    // Returns number of connected components and size of each component
    pub fn components_and_sizes(data: &Vec<(i32, i32, i32, f64)>) -> (usize, Vec<usize>) {
        let mut node_map = HashMap::new();
        let mut index = 0;

        // Map each unique node to an index (This is done specifically for the strong_ratings_data)
        for &(u, v, _, _) in data {
            node_map.entry(u).or_insert_with(|| {let i = index; index += 1; i});
            node_map.entry(v).or_insert_with(|| {let i = index; index += 1; i});
        }

        let mut cc = ConnectedComponents::new(node_map.len());

        // Run merge function on each edge in data using the node_map
        for &(u, v, _, _) in data {
            cc.merge(*node_map.get(&u).unwrap(), *node_map.get(&v).unwrap());
        }

        // Count connected components and their sizes
        let mut component_sizes = vec![0; node_map.len()];
        let mut components: usize = 0; 
        let mut seen = vec![false; node_map.len()];
        for &index in node_map.values() {
            let root = cc.find_root(index);
            if !seen[root] {
                seen[root] = true;
                components += 1;
                component_sizes[root] = cc.size[root];
            }
        }

        // Removes the not needed 0 entries
        component_sizes.retain(|&x| x > 0);

        return (components, component_sizes)
    }
}