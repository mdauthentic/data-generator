pub mod banking;
pub mod datatypes;
pub mod internet;
pub mod name;
pub mod numbers;
pub mod word;

use crate::model::banking::{Bank, Currency};
use crate::model::internet::{DomainName, Email};
use crate::model::name::Name;
use crate::model::numbers::{RandomFloat, RandomNumber};
use crate::model::word::{EmptyString, Strings};
use crate::provider::Provider;
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
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let result: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
                write!(f, "[{}]", result.join(","))
            }
        }
    }
}

pub fn string_to_provider(
    provider: &str,
    start: Option<f64>,
    end: Option<f64>,
) -> Box<dyn Provider> {
    match provider {
        "name" => Box::new(Name {}),
        "word" => Box::new(Strings {}),
        "int" => Box::new(RandomNumber {
            start: start.map(|s| s as i64),
            end: end.map(|e| e as i64),
        }),
        "float" => Box::new(RandomFloat { start, end }),
        "currency" => Box::new(Currency { start, end }),
        "bank" => Box::new(Bank {}),
        "domain" => Box::new(DomainName {}),
        "email" => Box::new(Email {}),
        _ => Box::new(EmptyString {}),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Value::String;
    use std::any::Any;

    #[test]
    fn test_provider_type() {
        let expected = String("".to_string()).type_id();
        let actual = string_to_provider("name", None, None).next().type_id();

        assert_eq!(actual, expected)
    }
}
