use rand;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Uniform, Normal};
// use chacha for portability in tests

enum Distribution {
    Uniform(Uniform<f64>),
    Normal(Normal<f64>),
}

pub struct Sampler<R> {
    rng: R,
    distribution: Distribution,
}


impl<R: Rng> Sampler<R> {

    pub fn new(rng: R) -> Self {
        Sampler { rng }
    }

    pub fn get_rng(&mut self) -> &mut R {
        &mut self.rng
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler() {
        let rng = StdRng::seed_from_u64(42);
        let mut sampler = Sampler::new(rng);
        let random_number: f64 = sampler.get_rng().random_range(0.0..1.0);
        assert_eq!(format!("{random_number:.2}"), "0.53");

    }
}