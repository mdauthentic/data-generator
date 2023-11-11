use crate::model::datatypes::{DataRange, FieldType};
use crate::parser::{closing_bracket, open_bracket, ParseResult, Span};
use crate::provider::Provider;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, value};
use nom::error::context;
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, separated_pair, tuple};

/// Parse all data type
pub fn type_identifier(input: Span) -> ParseResult<FieldType> {
    context(
        "schema data type",
        alt((scalar_type_identifier, arr_identifier)),
    )(input)
}
pub fn scalar_type_identifier(input: Span) -> ParseResult<FieldType> {
    context(
        "type identifier",
        alt((
            value(FieldType::Int, tag("int")),
            value(FieldType::String, tag("str")),
            value(FieldType::Boolean, tag("bool")),
            value(FieldType::Currency, tag("currency")),
            value(FieldType::Uuid, tag("uuid")),
            value(FieldType::DateTime, tag("date")),
        )),
    )(input)
}

pub fn arr_identifier(input: Span) -> ParseResult<FieldType> {
    context(
        "parse array types",
        map(
            tuple((
                tag("array"),
                delimited(tag("<"), scalar_type_identifier, tag(">")),
            )),
            |(_, typ)| FieldType::Array(Box::new(typ)),
        ),
    )(input)
}

/// Parses data range for example
/// `int(1..100)` yields `DataRange { start: 1, end: 100 }`
pub fn range_identifier(input: Span) -> ParseResult<DataRange> {
    context(
        "data range identifier",
        map(
            delimited(
                open_bracket,
                separated_pair(
                    recognize_float,
                    delimited(multispace0, alt((tag("->"), tag("until"))), multispace0),
                    recognize_float,
                ),
                closing_bracket,
            ),
            |(s, e)| DataRange {
                start: s.fragment().to_string().parse::<f64>().unwrap(),
                end: e.fragment().to_string().parse::<f64>().unwrap(),
            },
        ),
    )(input)
}

/*pub fn default_value(input: Span) -> ParseResult<DefaultValue> {
    context(
        "default value",
        tuple((
            alt((tag("one_of"), tag("many_of"))),
            delimited(open_bracket, many1(), closing_bracket),
        )),
    )
}*/

pub fn provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scalar_type_identifier() {
        let (_, actual_int) = scalar_type_identifier(Span::new("int")).unwrap();
        assert_eq!(actual_int, FieldType::Int);

        let (_, actual_int) = scalar_type_identifier(Span::new("str")).unwrap();
        assert_eq!(actual_int, FieldType::String);

        let (_, actual_int) = scalar_type_identifier(Span::new("bool")).unwrap();
        assert_eq!(actual_int, FieldType::Boolean);

        let (_, actual_int) = scalar_type_identifier(Span::new("currency")).unwrap();
        assert_eq!(actual_int, FieldType::Currency);

        let (_, actual_int) = scalar_type_identifier(Span::new("uuid")).unwrap();
        assert_eq!(actual_int, FieldType::Uuid)
    }

    #[test]
    fn test_arr_identifier() {
        let (_, actual) = arr_identifier(Span::new("array<int>")).unwrap();
        assert_eq!(actual, FieldType::Array(Box::new(FieldType::Int)));
    }

    #[test]
    fn test_range_identifier() {
        let expected = DataRange {
            start: 1f64,
            end: 1000f64,
        };
        let (_, actual) = range_identifier(Span::new("(1 until 1000)")).unwrap();

        assert_eq!(actual, expected)
    }
}
