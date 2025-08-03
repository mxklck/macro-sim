use crate::agents::Agent;
/// The `universe` module defines the core structures and logic for the macro-sim universe.

/// Represents the Universe, containing all entities and simulation state.
pub struct Universe {
    time: u64, // repents months
    agents: Vec<Agent>, // this will become a more complex network structure.
    // for now it's just a vector of agents.
    // this should contain all the agents (is there a hierarchy?)
}

impl Universe {
    /// Creates a new Universe instance.
    pub fn new() -> Self {
        Universe {
            time: 0,
            agents: vec![Agent::new(0, 10.0)],
        }
    }

    /// Advances the simulation by one time step
    pub fn advance(&mut self) {
        // Advance the simulation state here.
        // If advancing is successful, increment time; otherwise, handle the error or rollback.
        // For now, we assume success.
        self.time += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_step() {
        let mut universe = Universe::new();

        universe.advance();
        assert_eq!(universe.time, 1);

        universe.advance();
        assert_eq!(universe.time, 2);
    }
}