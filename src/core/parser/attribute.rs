use crate::core::interfaces::html_element::AttrMap;
use combine::error::ParseError;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{between, many1, sep_by, Parser, Stream};
use combine::{
    many,
    parser::char::{newline, space},
};
use combine::{
    parser::char::{char, letter},
    satisfy,
};

pub fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by::<Vec<(String, String)>, _, _, _>(
        attribute(),
        many::<String, _, _>(space().or(newline())),
    )
    .map(|attrs: Vec<(String, String)>| {
        let m: AttrMap = attrs.into_iter().collect();
        m
    })
}

fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let attribute_name = many1::<String, _, _>(letter());
    let attribute_inner_value =
        many1::<String, _, _>(satisfy(|c: char| c != '"')).map(|x| x.replace("&quot;", "\""));
    let attribute_value = between(char('"'), char('"'), attribute_inner_value);
    (
        attribute_name,
        many::<String, _, _>(space().or(newline())),
        char('='),
        many::<String, _, _>(space().or(newline())),
        attribute_value,
    )
        .map(|v| (v.0, v.4))
}
