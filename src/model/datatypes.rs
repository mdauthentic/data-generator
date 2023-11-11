use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Field {
    pub field: String,
    pub data_type: FieldType,
    pub provider: Box<dyn Provider>,
    pub range: Option<DataRange>,
    pub default: Option<DefaultValue>,
    pub maxlength: Option<u8>,
}

impl Field {
    fn new(field: String, data_type: FieldType, provider: Box<dyn Provider>) -> Self {
        Field {
            field,
            data_type,
            provider,
            range: None,
            default: None,
            maxlength: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum FieldType {
    Int,
    Float,
    String,
    Boolean,
    Currency,
    Uuid,
    DateTime,
    Array(Box<FieldType>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct DataRange {
    pub start: f64,
    pub end: f64,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DefaultValue {
    OneOf(Vec<Value>),
    ManyOf(Vec<Value>),
}
