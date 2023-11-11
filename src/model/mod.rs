pub mod banking;
pub mod datatypes;
pub mod internet;
pub mod name;
pub mod numbers;
pub mod word;

use rand::distributions::{Alphanumeric, DistString, Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, ""),
            Value::Bool(b) => write!(f, "{}", b.to_string()),
            Value::Number(n) => write!(f, "{}", n.to_string()),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let result: Vec<String> = arr.into_iter().map(|x| x.to_string()).collect();
                write!(f, "[{}]", result.join(","))
            }
        }
    }
}

/// Select random value from an array
pub fn random_data<T: Clone>(arr: &[T]) -> T {
    let random_data_index: usize = thread_rng().gen_range(0..arr.len());
    arr[random_data_index].clone()
}

/// Select random value from a vector
pub fn random_data_from_vec<T: Clone>(vc: Vec<T>) -> T {
    let random_data_index: usize = thread_rng().gen_range(0..vc.len());
    vc[random_data_index].clone()
}

pub fn random_num(start: Option<i64>, end: Option<i64>) -> i64 {
    let mut rng = thread_rng();
    rng.gen_range(start.unwrap_or_default()..=end.unwrap_or(1000000))
}

/// Generates randoms numbers from float bounds
pub fn random_float_in_range(start: Option<f64>, end: Option<f64>) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(start.unwrap_or(0.0)..=end.unwrap_or(1000000.0))
}

/// Sample data uniformly within the given range
pub fn random_uniform_num_in_range(start: Option<i64>, end: Option<i64>) -> i64 {
    let between = Uniform::from(start.unwrap_or_default()..=end.unwrap_or(1000000));
    let mut rng = thread_rng();
    between.sample(&mut rng)
}

pub fn random_string(maxlength: Option<u8>) -> String {
    let mut rng = thread_rng();
    Alphanumeric.sample_string(&mut rng, maxlength.unwrap_or(10) as usize)
}
