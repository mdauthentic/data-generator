use crate::data::banking::{BANKS, CURRENCY_SYMBOLS};
use crate::model::{random_data, random_float_in_range, Value};
use crate::provider::Provider;

#[derive(Debug, Clone)]
pub struct Currency {
    pub start: Option<f64>,
    pub end: Option<f64>,
}

impl Provider for Currency {
    fn next(&mut self) -> Value {
        let currency_symbol = random_data(CURRENCY_SYMBOLS);
        let amount = random_float_in_range(self.start, self.end);
        Value::String(format!("{}{}", currency_symbol, amount))
    }
}


#[derive(Debug, Clone)]
pub struct Bank {}

impl Provider for Bank {
    fn next(&mut self) -> Value {
        let bank = format!("{}", random_data(BANKS));
        Value::String(bank)
    }
}