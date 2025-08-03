pub mod individual;

// define some power dynamics
// define how we pay each other
// define actions and forecasting to make decisions
// payment "network"

enum AgentType {
    Person,
    Firm,
    Government,
}

#[derive(Debug)]
pub struct Agent {
    id: usize,
    wealth: f64,
    age: f64, // complex, keep as constant for now
}

impl Agent {
    pub fn new(id: usize, wealth: f64) -> Self {
        // dealing with age later on
        // companies can have an age, but they
        // can also die
        Agent {
            id,
            wealth,
            age: 100.0,
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

#[cfg(test)]
mod tests {
    use super::*;

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
