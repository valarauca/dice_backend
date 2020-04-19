use std::hash::BuildHasher;
use std::sync::{Arc, Mutex};

use super::rand::rngs::{OsRng, StdRng};
use super::rand::{Rng, RngCore, SeedableRng};

use super::seahash::SeaHasher;

lazy_static! {
    static ref RANDOM_POOL: Arc<Mutex<StdRng>> = {
        Arc::new(Mutex::new(match StdRng::from_rng(OsRng) {
            Ok(x) => x,
            Err(e) => panic!("could not build RNG, error {:?}", e),
        }))
    };
}

#[derive(Clone, Copy)]
pub struct DefaultSeaHasher {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}
impl Default for DefaultSeaHasher {
    fn default() -> DefaultSeaHasher {
        let mut rng = RANDOM_POOL.as_ref().lock().unwrap();
        let a = rng.next_u64();
        let b = rng.next_u64();
        let c = rng.next_u64();
        let d = rng.next_u64();
        DefaultSeaHasher { a, b, c, d }
    }
}
impl BuildHasher for DefaultSeaHasher {
    type Hasher = SeaHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SeaHasher::with_seeds(self.a, self.b, self.c, self.d)
    }
}
