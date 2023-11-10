use std::fmt::{Display};
use crate::data::domains::DOMAIN_EXT;
use crate::model::{random_data, Value};
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Domain {}


impl Provider for Domain {
    fn next(&mut self) -> Value {
        let domain_ext = random_data(DOMAIN_EXT);
        Value::String(domain_ext.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct DomainName {}

impl Provider for DomainName {
    fn next(&mut self) -> Value {
        let domain = "random_name";
        let ext = Domain{}.next();
        let domain_name = format!("{}.{}", domain, ext.to_string());
        Value::String(domain_name)
    }
}