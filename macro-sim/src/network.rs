use std::collections::HashMap;

// this should be directed.
pub struct AdjacencyList {
    edges: HashMap<usize, Vec<usize>> // 0 -> {1, 2}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_edges() {
        let mut adj = AdjacencyList { edges: HashMap::new() };
        adj.edges.insert(0, vec![1, 2]);
        adj.edges.insert(1, vec![2]);
        assert_eq!(adj.edges.get(&0), Some(&vec![1, 2]));
        assert_eq!(adj.edges.get(&1), Some(&vec![2]));
        assert_eq!(adj.edges.get(&2), None);
    }

    #[test]
    fn test_empty_adjacency_list() {
        let adj = AdjacencyList { edges: HashMap::new() };
        assert!(adj.edges.is_empty());
    }

    #[test]
    fn test_multiple_edges() {
        let mut adj = AdjacencyList { edges: HashMap::new() };
        adj.edges.insert(3, vec![4, 5, 6]);
        assert_eq!(adj.edges.get(&3), Some(&vec![4, 5, 6]));
    }
}
