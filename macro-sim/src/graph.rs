use std::collections::HashMap;

// this should be directed. use petgraph?

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<u64, Vec<u64>> // u32 4billion max (might not be enough)
}

impl Graph {
    pub fn init_test() -> Self {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(0, vec![1, 2]); // 0 -> 1, 2 (this should be directed, cyclic?)
        adjacency_list.insert(1, vec![1, 2]);
        adjacency_list.insert(2, vec![1, 2]);
        
        Self { edges: adjacency_list }
    }
}

// point towards where it should go?
// do I need the "weighs" as well?

// there might be different types of networks (social, local, economic) that we have to account for...
// each node will have ins and outs, so this is a directed graph structure.
// "equilibrate" my runs to get some interesting configurations


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let adj = Graph::init_test();
        println!("{:?}", adj);
        assert_eq!(true, true);
    }
}
