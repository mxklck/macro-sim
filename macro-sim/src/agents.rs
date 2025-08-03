pub mod individual;

// define some power dynamics
// define how we pay each other
// define actions and forecasting to make decisions

enum AgentType {
    Person,
    Firm,
    Government,
}

struct Agent {
    id: usize,
    agent_type: AgentType,
}
