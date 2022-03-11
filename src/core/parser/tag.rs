use crate::core::interfaces::html_element::AttrMap;

use super::attribute::attributes;

use combine::error::ParseError;
use combine::parser::char::{char, letter};
use combine::{between, many1, Parser, Stream};
use combine::{
    many,
    parser::char::{newline, space},
};

pub fn open_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let open_tag_name = many1::<String, _, _>(letter());
    let open_tag_content = (
        open_tag_name,
        many::<String, _, _>(space().or(newline())),
        attributes(),
    )
        .map(|v: (String, _, AttrMap)| (v.0, v.2));
    between(char('<'), char('>'), open_tag_content)
}

pub fn close_tag<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let close_tag_name = many1::<String, _, _>(letter());
    let close_tag_content = (char('/'), close_tag_name).map(|v| v.1);
    between(char('<'), char('>'), close_tag_content)
}
