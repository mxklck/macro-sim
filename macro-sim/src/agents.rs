pub mod individual;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Agent {
    id: usize,
    pub wealth: f64,
    age: f64, // lets work in "months" (so not f64 - should be discrete)
}

impl Agent {
    pub fn new(id: usize, wealth: f64) -> Self {
        Agent {
            id,
            wealth,
            age: 100.0,
        }
    }

    pub fn dump(&self) -> String {
        format!("id: {}, wealth: {:.2}, age: {:.2}", self.id, self.wealth, self.age)
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

pub struct Universe {
    agents: HashMap<usize, Agent>,
    connections: HashMap<usize, Vec<usize>>,
    time: usize,
}

impl Universe {
    pub fn new(n_agents: usize, wealth_distribution: &str) -> Self {
        println!("Using: {wealth_distribution}");

        let mut agents: HashMap<usize, Agent> = HashMap::with_capacity(n_agents);
        let mut connections: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n_agents);

        for i in 0..n_agents {
            let agent = Agent::new(i, 100.0);
            agents.insert(i, agent);
            connections.insert(i, Vec::new()); // TODO: should sample from a distribution
        }

        Universe {
            agents,
            connections,
            time: 0,
        }
    }

    pub fn from_agents(
        agents: HashMap<usize, Agent>,
        connections: HashMap<usize, Vec<usize>>,
    ) -> Result<Self, String> {
        let keys_agents: HashSet<_> = agents.keys().collect();
        let keys_connections: HashSet<_> = connections.keys().collect();

        if keys_agents != keys_connections {
            return Err("Agent and connection keys do not match".to_string());
        }

        Ok(Universe {
            agents,
            connections,
            time: 0,
        })
    }

    pub fn get_time(&self) -> usize {
        self.time
    }

    pub fn increment_time(&mut self) {
        self.make_payments();
        self.time += 1;
    }

    pub fn make_payments(&mut self) {
        for (sender_id, receiver_ids) in self.connections.iter() {
            println!("{sender_id}, {receiver_ids:#?}");

            let mut sender_change = 0.0;
            for receiver_id in receiver_ids {
                let wealth_change = 10.0;
                
                let receiver = self.agents.get_mut(receiver_id).unwrap();
                receiver.wealth += wealth_change;
                sender_change -= wealth_change;
            }

            let sender: &mut Agent = self.agents.get_mut(sender_id).unwrap();
            sender.wealth += sender_change;

        }
    }

    pub fn dump_state(&self) {
        // TODO: remove (or store)
        // - seems expensive to sort every time we dump
        let mut ids: Vec<_> = self.agents.keys().collect();
        ids.sort();

        for id in ids {
            let agent = self.agents.get(id).unwrap();
            let agent_dump = agent.dump();
            let agent_dump = format!("agent: {}, {}", id, agent_dump);
            println!("{}", agent_dump);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_time_step() {
        let mut agents = HashMap::new();
        agents.insert(0, Agent::new(0, 100.0));
        agents.insert(1, Agent::new(1, 100.0));
        agents.insert(2, Agent::new(2, 100.0));
        agents.insert(3, Agent::new(3, 100.0));
        agents.insert(4, Agent::new(4, 100.0));

        // test network
        /*
               a0
              / \    ^
            a1  a2   | connections point up
           /  \
          a3->a4

        */
        let mut connections = HashMap::new();
        connections.insert(0, Vec::new());
        connections.insert(1, vec![0]);
        connections.insert(2, vec![0]);
        connections.insert(3, vec![1, 4]);
        connections.insert(4, vec![1]);

        let mut universe = Universe::from_agents(agents, connections).unwrap();

        assert_eq!(universe.get_time(), 0);
        universe.dump_state();
        universe.increment_time();
        assert_eq!(universe.get_time(), 1);
        universe.dump_state();

    }

    #[test]
    fn test_agent_new() {
        let agent = Agent::new(1, 100.0);
        assert_eq!(agent.get_id(), 1);
        assert_eq!(agent.get_wealth(), 100.0);
    }

    #[test]
    fn test_agent_new_zero_wealth() {
        let agent = Agent::new(2, 0.0);
        assert_eq!(agent.id, 2);
        assert_eq!(agent.wealth, 0.0);
    }

    #[test]
    fn test_agent_new_negative_wealth() {
        let agent = Agent::new(3, -50.0);
        assert_eq!(agent.get_id(), 3);
        assert_eq!(agent.get_wealth(), -50.0);
        assert_eq!(agent.get_age(), 100.0);
    }
}
