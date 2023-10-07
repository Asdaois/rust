#![allow(dead_code)]
use rand::{thread_rng, Rng};

pub fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}
