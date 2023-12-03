pub mod provider;
pub mod r#type;
pub mod value;

use crate::model::datatypes::Field;
use crate::parser::provider::{default_value, provider};
use crate::parser::r#type::type_info;

use nom::multi::separated_list1;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::multispace0,
    combinator::{map, not, opt, peek},
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type ParseResult<'a, T> = IResult<Span<'a>, T>;

pub fn identifier(input: Span) -> ParseResult<Span> {
    context(
        "identifier",
        preceded(not(peek(keyword)), take_while1(is_identifier)),
    )(input)
}

pub fn is_identifier(chr: char) -> bool {
    chr.is_alphanumeric() || chr == '_' || chr == '@'
}

pub fn keyword(input: Span) -> ParseResult<Span> {
    alt((
        terminated(tag_no_case("create"), keyword_follow_char),
        terminated(tag_no_case("table"), keyword_follow_char),
        terminated(tag_no_case("as"), keyword_follow_char),
        terminated(tag_no_case("not"), keyword_follow_char),
        terminated(tag_no_case("null"), keyword_follow_char),
        terminated(tag_no_case("check"), keyword_follow_char),
        terminated(tag_no_case("default"), keyword_follow_char),
        terminated(tag_no_case("autoincrement"), keyword_follow_char),
        terminated(tag_no_case("primary"), keyword_follow_char),
        terminated(tag_no_case("unique"), keyword_follow_char),
    ))(input)
}

fn keyword_follow_char(input: Span) -> ParseResult<Span> {
    peek(alt((
        tag(" "),
        tag("\n"),
        tag(";"),
        tag("("),
        tag(")"),
        tag("\t"),
        tag(","),
        tag("="),
    )))(input)
}

pub fn ws_sep_comma(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(","), multispace0)(i)
}

pub fn ws_sep_colon(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(":"), multispace0)(i)
}

pub fn an_equal(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(":="), multispace0)(i)
}

pub fn open_bracket(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag("("), multispace0)(i)
}

pub fn closing_bracket(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(")"), multispace0)(i)
}

pub fn open_brace(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag("{"), multispace0)(i)
}

pub fn closing_brace(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag("}"), multispace0)(i)
}

// field parser
pub fn model_field(input: Span) -> ParseResult<Field> {
    context(
        "model field",
        map(
            tuple((
                identifier,
                ws_sep_colon,
                type_info,
                opt(delimited(
                    open_brace,
                    tuple((
                        opt(provider),
                        ws_sep_comma,
                        opt(default_value),
                        ws_sep_comma,
                    )),
                    closing_brace,
                )),
            )),
            |(iden, _, t_info, others)| {
                let (provider, default) = match others {
                    Some((prov, _, def_val, _)) => (prov, def_val),
                    None => (None, None),
                };
                Field {
                    field: iden.fragment().to_string(),
                    type_info: t_info,
                    provider,
                    default,
                    maxlength: None,
                }
            },
        ),
    )(input)
}

pub fn schema_parser(input: Span) -> ParseResult<Vec<Field>> {
    //model model_name open_bracket Vec<Field> close_bracket
    context(
        "schema parser",
        map(
            tuple((
                open_brace,
                separated_list1(ws_sep_comma, model_field),
                closing_brace,
            )),
            |(_, field, _)| field,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_model_field() {
        let model = r"col4: str {
            provider := @name,
            default  := many_of(s1, s2, s3),
        }";

        let (_, actual) = model_field(Span::new(model)).unwrap();
        println!("{:?}", actual);
        let model2 = r"col4: int(1 until 10) {
            provider := @number,
            default  := many_of(1, 2, 3),
        }";
        let (_, actual2) = model_field(Span::new(model2)).unwrap();
        println!("{:?}", actual2);
        assert_eq!(1, 1)
    }

    #[test]
    fn test_schema_parser() {
        let schema = r"{
        col1: str {
            provider := @name,
            default  := many_of(s1, s2, s3),
        },
        col2: int(1 until 20)
        }
        ";

        let (_, actual) = schema_parser(Span::new(schema)).unwrap();
        println!("{:?}", actual);
        assert_eq!(1, 1)
    }
}
