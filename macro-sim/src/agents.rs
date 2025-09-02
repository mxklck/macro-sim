pub mod individual;

#[derive(Debug)]
pub struct Agent {
    id: usize,
    pub wealth: f64,
    age: usize, // lets work in "months" (so not f64 - should be discrete)
}

// rename to Entity?
impl Agent {
    pub fn new(id: usize, wealth: f64) -> Self {
        Agent {
            id,
            wealth,
            age: 100,
        }
    }

    pub fn dump(&self) -> String {
        format!(
            "id: {}, wealth: {:.2}, age: {:.2}",
            self.id, self.wealth, self.age
        )
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_wealth(&self) -> f64 {
        self.wealth
    }

    pub fn get_age(&self) -> usize {
        self.age
    }
}

// I might need an AgentCollection object to store multiple agents...

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
        assert_eq!(agent.get_age(), 100);
    }
}
