
use std::hash::{BuildHasher};

use super::seahash::{SeaHasher};


#[derive(Default,Clone,Copy)]
pub struct DefaultSeaHasher {
    /// avoid zero-sized nonsense.
    #[allow(dead_code)] arg: usize
}
impl BuildHasher for DefaultSeaHasher {
    type Hasher = SeaHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SeaHasher::default()
    }
}
