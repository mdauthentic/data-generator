pub mod r#type;
pub mod value;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::multispace0,
    combinator::{not, peek},
    error::context,
    sequence::{delimited, preceded, terminated},
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
