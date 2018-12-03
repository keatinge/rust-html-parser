extern crate html_parser_f;
use html_parser_f as parser;





#[test]
fn stack_overflow() {
    let the_html = include_str!("../htmls/stack.html");
    let toks = parser::build_tokens_list(the_html);
    parser::parse_tree(&toks);
}

#[test]
fn reddit() {
    let the_html = include_str!("../htmls/reddit.html");

    let toks = parser::build_tokens_list(the_html);
    parser::parse_tree(&toks);
}



#[test]
fn twitter() {
    let the_html = include_str!("../htmls/twitter.html");
    let toks = parser::build_tokens_list(the_html);
    parser::parse_tree(&toks);
}






