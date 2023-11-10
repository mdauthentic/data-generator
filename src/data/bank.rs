use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Bank {}

impl Provider for Bank {
    fn next(&mut self) -> Value {
        todo!()
    }
}