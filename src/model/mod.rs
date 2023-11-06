pub mod numbers;
pub mod ratings;
pub mod domains;
pub mod currency;

use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};


#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    IntegersNum(i64),
    FloatNum(f64),
    String(String),
    Array(Vec<Value>),
}


pub fn random_data<T: Clone>(arr: &[T]) -> T {
    let random_data_index: usize = thread_rng().gen_range(0..arr.len());
    arr[random_data_index].clone()
}

pub fn random_num(start: Option<i64>, end: Option<i64>) -> i64 {
    let mut rng = thread_rng();
    rng.gen_range(start.unwrap_or_default()..=end.unwrap_or_else(1000000))
}

/// Generates randoms numbers from float bounds
pub fn random_float_in_range(start: Option<f64>, end: Option<f64>) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(start.unwrap_or_else(0.0)..=end.unwrap_or_else(1000000.0))
}

/// Sample data uniformly within the given range
pub fn random_uniform_num_in_range(start: Option<i64>, end: Option<i64>) -> i64 {
    let between = Uniform::from(start.unwrap_or_default()..=end.unwrap_or_else(1000000));
    let mut rng = thread_rng();
    between.sample(&mut rng)
}