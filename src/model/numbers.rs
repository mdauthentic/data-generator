use crate::model::{random_float_in_range, random_num, Value};
use crate::provider::Provider;


#[derive(Debug, Clone)]
pub struct RandomNumber {
    /// The start inclusively.
    pub(crate) start: Option<i64>,
    /// The end exclusively.
    pub(crate) end: Option<i64>,
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
    pub(crate) start: Option<f64>,
    /// The end exclusively.
    pub(crate) end: Option<f64>,
}

impl Provider for RandomFloat {
    fn next(&mut self) -> Value {
        let value = random_float_in_range(self.start, self.end);
        Value::Number(value)
    }
}