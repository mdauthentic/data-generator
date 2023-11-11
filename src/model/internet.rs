use crate::data::internet::{DOMAIN_EXT, EMAIL_EXT};
use crate::model::{random_data, random_string, Value};
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
        let domain = random_string(Some(12));
        let ext = Domain {}.next();
        let domain_name = format!("{}.{}", domain, ext.to_string());
        Value::String(domain_name)
    }
}

#[derive(Debug, Clone)]
pub struct Email {}

impl Provider for Email {
    fn next(&mut self) -> Value {
        let user = random_string(None);
        let email_ext = random_data(EMAIL_EXT);
        Value::String(format!("{}@{}", user, email_ext))
    }
}
