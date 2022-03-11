mod core;
use crate::core::parser::node::parse_document_into_dom;

const HTML: &str = "\
    <body>\
      <span class=\"hide\">hide</span>\
      <div id=\"main\" class=\"content\">\
        <p>hello rust html parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>hello rust css parser!!</p>\
        <p>this is my browser</p>\
        \
      </div>\
    </body>\
";

fn main() {
    let res = parse_document_into_dom(HTML);
    println!("{:#?}", res);
}
