use nom::{bytes::complete::tag, character::complete::multispace0, sequence::delimited, IResult};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type ParseResult<'a, T> = IResult<Span<'a>, T>;

pub fn ws_sep_comma(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(","), multispace0)(i)
}

pub fn ws_sep_colon(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(":"), multispace0)(i)
}

pub fn an_equal(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag(":="), multispace0)(i)
}

pub fn open_brace(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag("{"), multispace0)(i)
}

pub fn closing_brace(i: Span) -> ParseResult<Span> {
    delimited(multispace0, tag("}"), multispace0)(i)
}