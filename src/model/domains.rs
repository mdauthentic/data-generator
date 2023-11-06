use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Domain {}


impl Provider for Domain {
    fn next(&mut self) -> Value {
        todo!()
    }
}