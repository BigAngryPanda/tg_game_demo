#[allow(unused_imports)]
use crate::log;

use std::hash::{
    RandomState,
    BuildHasher,
    Hasher
};

// https://blog.orhun.dev/zero-deps-random-in-rust/
#[derive(Debug)]
pub struct RandomGenerator {
    random: u64,
}

impl RandomGenerator {
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            random: RandomState::new().build_hasher().finish(),
        }
    }

    pub fn rand_mod(&mut self, m: u64) -> u64 {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 17;
        self.random ^= self.random << 5;

        self.random % m
    }

    pub fn rand_in_range(&mut self, min: u64, max: u64) -> u64 {
        min + self.rand_mod(max - min)
    }
}

pub fn rand_in_range(min: u64, max: u64) -> u64 {
    (web_sys::js_sys::Math::random()*(max - min) as f64) as u64 + min
}