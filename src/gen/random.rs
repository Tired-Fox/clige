use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;

/// Struct for generating platform independant random numbers
/// 
/// The generators can be created from seeds of strings or i64
pub struct Random(i64, ChaCha8Rng);

impl<'rng> Random {
    pub fn seed(&self) -> i64 {
        self.0
    }

    pub fn generator(&'rng mut self) -> &'rng mut ChaCha8Rng {
        &mut self.1
    }

    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::from_entropy(); 
        let seed = rng.get_seed();
        let index = rng.gen_range(0..4);

        let seed = u64::from_ne_bytes(seed[(0 + 8*index)..(8 + 8*index)].try_into().unwrap());
        Random(seed as i64, ChaCha8Rng::seed_from_u64(seed))
    }
}

impl From<&str> for Random {
    fn from(value: &str) -> Self {
        let seed = match value.parse::<i64>() {
            Ok(s) => s as u64,
            Err(_) => {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                hasher.finish()
            }
        };

        Random(seed as i64, ChaCha8Rng::seed_from_u64(seed))
    }
}

impl From<String> for Random {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<i64> for Random {
    fn from(value: i64) -> Self {
        Random(value, ChaCha8Rng::seed_from_u64(value as u64))
    }
}
