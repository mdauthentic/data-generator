use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Currency {
    pub default: Option<String>,
    pub start: f64,
    pub end: f64,
}

impl Provider for Currency {
    fn next(&mut self) -> Value {
        todo!()
    }
}