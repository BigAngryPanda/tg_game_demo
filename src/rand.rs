#[allow(unused_imports)]
use crate::log;

pub fn rand_in_range(min: u64, max: u64) -> u64 {
    (web_sys::js_sys::Math::random()*(max - min) as f64) as u64 + min
}