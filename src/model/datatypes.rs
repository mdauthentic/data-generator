use crate::model::Value;
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Field {
    pub field: String,
    pub type_info: TypeInfo,
    pub provider: Option<Box<dyn Provider>>,
    pub default: Option<DefaultValue>,
    pub maxlength: Option<u8>,
}

impl Field {
    fn new(field: String, type_info: TypeInfo, provider: Option<Box<dyn Provider>>) -> Self {
        Field {
            field,
            type_info,
            provider,
            default: None,
            maxlength: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TypeInfo {
    pub data_type: FieldType,
    pub range: Option<DataRange>,
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
pub struct DefaultValue {
    pub choice: DefaultOption,
    pub values: Vec<Value>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DefaultOption {
    OneOf,
    ManyOf,
}
