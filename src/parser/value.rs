use crate::model::Value;
use crate::parser::{identifier, ws_sep_comma, ParseResult, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::error::context;
use nom::multi::separated_list1;
use nom::number::complete::recognize_float;

pub fn value_list(input: Span) -> ParseResult<Vec<Value>> {
    context("value list", alt((bool_vec, number_vec, string_vec)))(input)
}

pub fn number_vec(input: Span) -> ParseResult<Vec<Value>> {
    context(
        "numeric value",
        map(separated_list1(ws_sep_comma, recognize_float), |x| {
            let num_vec: Vec<Value> = x
                .iter()
                .map(|s: &Span| Value::Number(s.fragment().parse::<f64>().unwrap()))
                .collect();
            num_vec
        }),
    )(input)
}

pub fn string_vec(input: Span) -> ParseResult<Vec<Value>> {
    context(
        "string value",
        map(separated_list1(ws_sep_comma, identifier), |x| {
            let str_vec: Vec<Value> = x
                .iter()
                .map(|s: &Span| Value::String(s.fragment().to_string()))
                .collect();
            str_vec
        }),
    )(input)
}

pub fn bool_identifier(input: Span) -> ParseResult<Value> {
    context(
        "boolean identifier",
        alt((
            value(Value::Bool(false), tag("false")),
            value(Value::Bool(true), tag("true")),
        )),
    )(input)
}

pub fn bool_vec(input: Span) -> ParseResult<Vec<Value>> {
    context(
        "boolean value",
        separated_list1(ws_sep_comma, bool_identifier),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_vec() {
        let expected = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let (_, actual) = number_vec(Span::new("1.0, 2.0, 3.0")).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_str_vec() {
        let expected = vec![
            Value::String("s1".to_string()),
            Value::String("s2".to_string()),
            Value::String("s3".to_string()),
        ];
        let (_, actual) = string_vec(Span::new("s1, s2, s3")).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_boolean_identifier() {
        let (_, actual) = bool_identifier(Span::new("false")).unwrap();
        assert_eq!(actual, Value::Bool(false))
    }

    #[test]
    fn test_bool_vec() {
        let expected = vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)];
        let (_, actual) = bool_vec(Span::new("true, false, true")).unwrap();
        assert_eq!(actual, expected)
    }
}
