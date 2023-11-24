use crate::model::datatypes::DefaultOption::{ManyOf, OneOf};
use crate::model::datatypes::DefaultValue;
use crate::model::name::Name;
use crate::model::numbers::{RandomFloat, RandomNumber};
use crate::parser::value::value_list;
use crate::parser::{an_equal, closing_bracket, open_bracket, ParseResult, Span};
use crate::provider::{Provider, ProviderClone};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    error::context,
    sequence::{delimited, tuple},
};

pub fn default_value(input: Span) -> ParseResult<DefaultValue> {
    context(
        "default value",
        map(
            tuple((
                tag("default"),
                an_equal,
                alt((value(OneOf, tag("one_of")), value(ManyOf, tag("many_of")))),
                delimited(open_bracket, value_list, closing_bracket),
            )),
            |(_, _, x1, x2)| DefaultValue {
                choice: x1,
                values: x2,
            },
        ),
    )(input)
}

#[inline]
pub fn name_provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    map(value(Name {}, tag("name")), |x| x.clone_box())(input)
}

pub fn number_provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    map(
        value(
            RandomNumber {
                start: None,
                end: None,
            },
            tag("number"),
        ),
        |x| x.clone_box(),
    )(input)
}

pub fn float_provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    map(
        value(
            RandomFloat {
                start: None,
                end: None,
            },
            tag("number"),
        ),
        |x| x.clone_box(),
    )(input)
}

#[inline]
pub fn word_provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    map(value(Name {}, tag("name")), |x| x.clone_box())(input)
}

pub fn provider(input: Span) -> ParseResult<Box<dyn Provider>> {
    context(
        "provider",
        map(
            tuple((
                tag("provider"),
                an_equal,
                tag("@"),
                alt((name_provider, word_provider, float_provider)),
            )),
            |(_, _, _, x1)| x1,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Value;

    #[test]
    fn test_provider() {
        let (_, mut actual) = provider(Span::new("provider := @name")).unwrap();
        let actual_name = match actual.next() {
            Value::String(_) => 1,
            _ => 0,
        };
        assert_eq!(1, actual_name);

        let (_, mut actual1) = provider(Span::new("provider := @number")).unwrap();
        let actual_res = match actual1.next() {
            Value::Number(_) => 1,
            _ => 0,
        };
        assert_eq!(1, actual_res)
    }

    #[test]
    fn test_default_value() {
        let values = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let expected = DefaultValue {
            choice: ManyOf,
            values,
        };
        let (_, actual) = default_value(Span::new("default := many_of(1.0, 2.0, 3.0)")).unwrap();
        assert_eq!(actual, expected)
    }
}
