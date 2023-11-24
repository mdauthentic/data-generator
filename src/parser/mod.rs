pub mod provider;
pub mod r#type;
pub mod value;

use crate::model::datatypes::Field;
use crate::parser::provider::{default_value, provider};
use crate::parser::r#type::type_info;
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

pub fn model_parser(_input: Span) -> ParseResult<Vec<Field>> {
    //model model_name open_bracket Vec<Field> close_bracket
    todo!()
}

//model test_data {
/*pub fn model_field(input: Span) -> ParseResult<Field> {
    context(
        "model field",
        map(
            tag("model"), multispace1, identifier, open_brace, separated_list(model_field), closing_brace,
            |(iden, t_info, Some((prov, _, def_val, _)))| Field {
                field: iden.fragment().to_string(),
                type_info: t_info,
                provider: prov,
                default: def_val,
                maxlength: None,
            },
        ),
    )(input)
}*/

pub fn model_field(input: Span) -> ParseResult<Field> {
    context(
        "model field",
        map(
            tuple((
                identifier,
                ws_sep_colon,
                type_info,
                delimited(
                    open_brace,
                    opt(tuple((
                        opt(provider),
                        ws_sep_comma,
                        opt(default_value),
                        ws_sep_comma,
                    ))),
                    closing_brace,
                ),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_model_field() {
        //let res = r#""#;
        let model = r"col4: str {
            provider := @name,
            default   := many_of(s1, s2, s3),
        }";

        let (_, actual) = model_field(Span::new(model)).unwrap();
        println!("{:?}", actual);
        assert_eq!(1, 1)
    }
}
