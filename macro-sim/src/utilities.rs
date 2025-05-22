mod rng;

pub enum Distribution {
    Uniform,
    Normal,
    Exponential,
}

pub struct DistributionSettings<T> {
    pub min: T,
    pub max: T,
    pub distribution: Distribution,
}

pub struct Settings {
    pub age_settings: DistributionSettings<f64>,
    pub net_worth_settings: DistributionSettings<f64>,
}

// read from an e.g. toml config file
impl Settings {
    pub fn new() -> Self {
        Settings {
            age_settings: DistributionSettings {
                min: 0.0,
                max: 100.0,
                distribution: Distribution::Uniform,
            },
            net_worth_settings: DistributionSettings {
                min: 0.0,
                max: 100.0,
                distribution: Distribution::Uniform,
            },
        }
    }
}

// use rand::{Rng, SeedableRng};
// use rand::rngs::StdRng;

// pub fn get_seeded_rng(seed: u64) -> StdRng {
//     let rng = StdRng::seed_from_u64(seed);
//     rng
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_generate_random_number() {
//         let mut test_rng = get_seeded_rng(42);
//         let test_int = test_rng.random_range(0..10);
//         assert_eq!(test_int, 1);
//         let test_float = test_rng.random_range(0.0..10.0);
//         assert_eq!(format!("{test_float:.2}"), "2.49");
//     }
// }
