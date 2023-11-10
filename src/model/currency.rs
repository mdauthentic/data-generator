use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Currency {
    pub start: Option<f64>,
    pub end: Option<f64>,
}

impl Provider for Currency {
    fn next(&mut self) -> Value {
        todo!()
    }
}