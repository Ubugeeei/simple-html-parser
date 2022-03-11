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
use crate::core::interfaces::html_element::AttrMap;

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

// #[cfg(test)]
// mod tests {
//     use url::Url;

//     use super::*;
//     use crate::fetch::{HeaderMap, Response};
//     use crate::{
//         dom::{AttrMap, Document, Element, Text},
//         fetch::{HTTPStatus, ResponseType},
//     };

//     // parsing tests of attributes
//     #[test]
//     fn test_parse_attribute() {
//         assert_eq!(
//             attribute().easy_parse("test=\"foobar\""),
//             Ok((("test".to_string(), "foobar".to_string()), ""))
//         );

//         assert_eq!(
//             attribute().easy_parse("test = \"foobar\""),
//             Ok((("test".to_string(), "foobar".to_string()), ""))
//         );

//         assert_eq!(
//             attribute().easy_parse("test = \"&quot;&quot;\""),
//             Ok((("test".to_string(), "\"\"".to_string()), ""))
//         )
//     }

//     #[test]
//     fn test_parse_attributes() {
//         let mut expected_map = AttrMap::new();
//         expected_map.insert("test".to_string(), "foobar".to_string());
//         assert_eq!(
//             attributes().easy_parse("test=\"foobar\""),
//             Ok((expected_map, ""))
//         );

//         assert_eq!(attributes().easy_parse(""), Ok((AttrMap::new(), "")))
//     }

//     // parsing tests of open tags
//     #[test]
//     fn test_parse_open_tag_without_attributes() {}

//     #[test]
//     fn test_parse_open_tag() {
//         {
//             assert_eq!(
//                 open_tag().easy_parse("<p>aaaa"),
//                 Ok((("p".to_string(), AttrMap::new()), "aaaa"))
//             );
//         }
//         {
//             let mut attributes = AttrMap::new();
//             attributes.insert("id".to_string(), "test".to_string());
//             assert_eq!(
//                 open_tag().easy_parse("<p id=\"test\">"),
//                 Ok((("p".to_string(), attributes), ""))
//             )
//         }

//         {
//             let result = open_tag().easy_parse("<p id=\"test\" class=\"sample\">");
//             let mut attributes = AttrMap::new();
//             attributes.insert("id".to_string(), "test".to_string());
//             attributes.insert("class".to_string(), "sample".to_string());
//             assert_eq!(result, Ok((("p".to_string(), attributes), "")));
//         }

//         {
//             assert!(open_tag().easy_parse("<p id>").is_err());
//         }
//     }

//     // parsing tests of close tags
//     #[test]
//     fn test_parse_close_tag() {
//         let result = close_tag().easy_parse("</p>");
//         assert_eq!(result, Ok(("p".to_string(), "")))
//     }

//     // parsing tests of an element
//     #[test]
//     fn test_parse_element() {
//         assert_eq!(
//             element().easy_parse("<p></p>"),
//             Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
//         );

//         assert_eq!(
//             element().easy_parse("<p>Hello World</p>"),
//             Ok((
//                 Element::new(
//                     "p".to_string(),
//                     AttrMap::new(),
//                     vec![Text::new("Hello World".to_string())]
//                 ),
//                 ""
//             ))
//         );

//         assert!(element().easy_parse("<p>Hello World</div>").is_err());
//     }

//     // parsing tests of a tag
//     #[test]
//     fn test_parse_text() {
//         {
//             assert_eq!(
//                 text().easy_parse("Hello World"),
//                 Ok((Text::new("Hello World".to_string()), ""))
//             );
//         }
//         {
//             assert_eq!(
//                 text().easy_parse("Hello World<"),
//                 Ok((Text::new("Hello World".to_string()), "<"))
//             );
//         }
//     }

//     // parsing tests of documents
//     #[test]
//     fn test_parse_single_without_nest() {
//         let url = "http://example.com";
//         let s = Response {
//             url: Url::parse(url).unwrap(),
//             status: HTTPStatus::OK,
//             rtype: ResponseType::Basic,
//             headers: HeaderMap::new(),
//             data: "<p>Hello World</p>".as_bytes().to_vec(),
//         };
//         let got = parse(s);
//         let expected = Ok(Document::new(
//             Url::parse(url).unwrap().to_string(),
//             Url::parse(url).unwrap().to_string(),
//             Element::new(
//                 "p".to_string(),
//                 AttrMap::new(),
//                 vec![Text::new("Hello World".to_string())],
//             ),
//         ));
//         assert_eq!(got, expected)
//     }

//     #[test]
//     fn test_parse_two_without_nest() {
//         let url = "http://example.com";
//         let s = Response {
//             url: Url::parse(url).unwrap(),
//             status: HTTPStatus::OK,
//             rtype: ResponseType::Basic,
//             headers: HeaderMap::new(),
//             data: "<p>Hello World (1)</p><p>Hello World (2)</p>"
//                 .as_bytes()
//                 .to_vec(),
//         };
//         let expected = Ok(Document::new(
//             Url::parse(url).unwrap().to_string(),
//             Url::parse(url).unwrap().to_string(),
//             Element::new(
//                 "html".to_string(),
//                 AttrMap::new(),
//                 vec![
//                     Element::new(
//                         "p".to_string(),
//                         AttrMap::new(),
//                         vec![Text::new("Hello World (1)".to_string())],
//                     ),
//                     Element::new(
//                         "p".to_string(),
//                         AttrMap::new(),
//                         vec![Text::new("Hello World (2)".to_string())],
//                     ),
//                 ],
//             ),
//         ));
//         assert_eq!(parse(s), expected)
//     }

//     #[test]
//     fn test_parse_with_nest() {
//         let url = "http://example.com";
//         let s = Response {
//             url: Url::parse(url).unwrap(),
//             status: HTTPStatus::OK,
//             rtype: ResponseType::Basic,
//             headers: HeaderMap::new(),
//             data: "<div><p>nested (1)</p><p>nested (2)</p></div>"
//                 .as_bytes()
//                 .to_vec(),
//         };
//         let expected = Ok(Document::new(
//             Url::parse(url).unwrap().to_string(),
//             Url::parse(url).unwrap().to_string(),
//             Element::new(
//                 "div".to_string(),
//                 AttrMap::new(),
//                 vec![
//                     Element::new(
//                         "p".to_string(),
//                         AttrMap::new(),
//                         vec![Text::new("nested (1)".to_string())],
//                     ),
//                     Element::new(
//                         "p".to_string(),
//                         AttrMap::new(),
//                         vec![Text::new("nested (2)".to_string())],
//                     ),
//                 ],
//             ),
//         ));
//         assert_eq!(parse(s), expected)
//     }
// }
