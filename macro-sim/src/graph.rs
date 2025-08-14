// use petgraph::graph::Graph;
// use std::collections::HashMap;

// use crate::agents::Agent;

// // this should be directed. use petgraph?
// // point towards where it should go?
// // do I need the "weighs" as well?

// // there might be different types of networks (social, local, economic) that we have to account for...
// // each node will have ins and outs, so this is a directed graph structure.
// // "equilibrate" my runs to get some interesting configurations
// // agents live on a graph, but how tightly should this be coupled?

// struct PaymentNetwork {
//     graph: Graph<Agent, u64>,
// }

// impl PaymentNetwork {
//     pub fn default() {
//         let mut graph: Graph<Agent, u64> = Graph::new();
//         // do one connection and visualize it
//         // hierarchical graphs?
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_construction() {
//         let adj = 1;
//         println!("{:?}", adj);
//         assert_eq!(true, true);
//     }
// }
