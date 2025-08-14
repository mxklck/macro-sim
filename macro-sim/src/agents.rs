pub mod individual;
use std::collections::{HashMap, HashSet};

// define some power dynamics
// define how we pay each other
// define actions and forecasting to make decisions
// payment "network"

#[derive(Debug)]
pub struct Agent {
    id: usize,
    wealth: f64,
    age: f64,            // complex, keep as constant for now
    owes_to: Vec<usize>, // store agent IDs + magnitude - cannot owe self
}

impl Agent {
    pub fn new(id: usize, wealth: f64, owes_to: Vec<usize>) -> Self {
        Agent {
            id,
            wealth,
            age: 100.0,
            owes_to,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_wealth(&self) -> f64 {
        self.wealth
    }

    pub fn get_age(&self) -> f64 {
        self.age
    }
}

fn depth_first_search(agents: &HashMap<usize, Agent>, start_id: usize, visited: &mut HashSet<usize>) {
    // a recursive implementation that I don't quite understand.
    if !visited.insert(start_id) {
        return;
    }

    let agent = &agents[&start_id];
    for &neighbor_id in &agent.owes_to {
        depth_first_search(agents, neighbor_id, visited);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agents_traversal() {
        // test network
        /*
               a0
              / \    ^ 
            a1  a2   | owing goes up
           /  \
          a3->a4

        */
        let a0 = Agent::new(0, 100.0, Vec::new());
        let a1 = Agent::new(1, 100.0, vec![0]);
        let a2 = Agent::new(2, 100.0, vec![0]);
        let a3 = Agent::new(3, 100.0, vec![1, 4]);
        let a4 = Agent::new(4, 100.0, vec![1]);

        // then we need to traverse and update
        let mut agent_hashmap: HashMap<usize, Agent> = HashMap::new();
        agent_hashmap.insert(0, a0);
        agent_hashmap.insert(1, a1);
        agent_hashmap.insert(2, a2);
        agent_hashmap.insert(3, a3);
        agent_hashmap.insert(4, a4);
        
        let agent_hashmap = agent_hashmap; // make immutable
        println!("{agent_hashmap:#?}");

        let mut visited_nodes: HashSet<usize> = HashSet::new();

        // mutable references have to be declared directly...
        depth_first_search(&agent_hashmap, 0, &mut visited_nodes);

        // unique keys that haven't occurred before -> static sampler?



    }

    #[test]
    fn test_agent_new() {
        let agent = Agent::new(1, 100.0, Vec::new());
        assert_eq!(agent.get_id(), 1);
        assert_eq!(agent.get_wealth(), 100.0);
    }

    #[test]
    fn test_agent_new_zero_wealth() {
        let agent = Agent::new(2, 0.0, Vec::new());
        assert_eq!(agent.id, 2);
        assert_eq!(agent.wealth, 0.0);
    }

    #[test]
    fn test_agent_new_negative_wealth() {
        let agent = Agent::new(3, -50.0, Vec::new());
        assert_eq!(agent.get_id(), 3);
        assert_eq!(agent.get_wealth(), -50.0);
        assert_eq!(agent.get_age(), 100.0);
    }
}
