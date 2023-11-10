use crate::data::name::{FIRST_NAME_F, LAST_NAME};
use crate::model::{random_data, Value};
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Name {}

impl Provider for Name {
    fn next(&mut self) -> Value {
        let first_name = random_data(FIRST_NAME_F);
        let last_name = random_data(LAST_NAME);
        let name_str = format!("{} {}", first_name, last_name);
        Value::String(name_str)
    }
}