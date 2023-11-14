use crate::model::{random_float_in_range, random_num, Value};
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct RandomNumber {
    /// The start inclusively.
    pub start: Option<i64>,
    /// The end exclusively.
    pub end: Option<i64>,
}

impl Provider for RandomNumber {
    fn next(&mut self) -> Value {
        let value = random_num(self.start, self.end);
        Value::Number(value as f64)
    }
}

#[derive(Debug, Clone)]
pub struct RandomFloat {
    /// The start inclusively.
    pub start: Option<f64>,
    /// The end exclusively.
    pub end: Option<f64>,
}

impl Provider for RandomFloat {
    fn next(&mut self) -> Value {
        let value = random_float_in_range(self.start, self.end);
        Value::Number(value)
    }
}
