pub mod individual;

// define some power dynamics
// define how we pay each other
// define actions and forecasting to make decisions

enum AgentType {
    Person,
    Firm,
    Government,
}

pub struct Agent {
    id: usize,
    wealth: f64,
}

impl Agent {
    pub fn new(id: usize, wealth: f64) -> Self{
        Agent {id, wealth}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_new() {
        let agent = Agent::new(1, 100.0);
        assert_eq!(agent.id, 1);
        assert_eq!(agent.wealth, 100.0);
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
        assert_eq!(agent.id, 3);
        assert_eq!(agent.wealth, -50.0);
    }
}