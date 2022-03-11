

use combine::{
    error::{ParseError, StreamError},
    many1, satisfy, {attempt, choice, many, parser, Parser, Stream},
};

use crate::core::interfaces::{node::Node, text_element::Text, html_element::Element};

use super::{error::HTMLParseError, tag::{open_tag, close_tag}};

parser! {
    fn nodes[Input]()(Input) -> Vec<Box<Node>>
    where [Input: Stream<Token = char>]
    {
        nodes_()
    }
}

pub fn parse_document_into_dom(document: &str) -> Result<Vec<Box<Node>>, HTMLParseError> {
    nodes()
        .parse(document)
        .map(|(nodes, _)| nodes)
        .map_err(|e| HTMLParseError::InvalidResourceError(e))
}

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(many(choice((attempt(element()), attempt(text())))))
}

fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c: char| c != '<')).map(|t| Text::new(t))
}

fn element<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (open_tag(), nodes(), close_tag()).and_then(
        |((open_tag_name, attributes), children, close_tag_name)| {
            if open_tag_name == close_tag_name {
                Ok(Element::new(open_tag_name, attributes, children))
            } else {
                Err(<Input::Error as combine::error::ParseError<
                    char,
                    Input::Range,
                    Input::Position,
                >>::StreamError::message_static_message(
                    "tag name of open tag and close tag mismatched",
                ))
            }
        },
    )
}
