use crate::agents::Agent;
use std::collections::{HashMap, HashSet};

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

    // add a add_agents method, that can be used to insert custom agents into an initialized Universe

    pub fn get_agent(&self, id: usize) -> Option<&Agent> {
        self.agents.get(&id)
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
        assert_eq!(universe.get_agent(0).unwrap().get_wealth(), 100.0);
        assert_eq!(universe.get_agent(1).unwrap().get_wealth(), 100.0);
        assert_eq!(universe.get_agent(2).unwrap().get_wealth(), 100.0);
        assert_eq!(universe.get_agent(3).unwrap().get_wealth(), 100.0);
        assert_eq!(universe.get_agent(4).unwrap().get_wealth(), 100.0);

        universe.increment_time();

        assert_eq!(universe.get_time(), 1);
        assert_eq!(universe.get_agent(0).unwrap().get_wealth(), 120.0);
        assert_eq!(universe.get_agent(1).unwrap().get_wealth(), 110.0);
        assert_eq!(universe.get_agent(2).unwrap().get_wealth(), 90.0);
        assert_eq!(universe.get_agent(3).unwrap().get_wealth(), 80.0);
        assert_eq!(universe.get_agent(4).unwrap().get_wealth(), 100.0);
    }
}
