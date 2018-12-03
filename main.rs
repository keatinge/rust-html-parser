#![feature(test)]

mod lib;
use lib as parser;
use std::io::*;


fn debug_toks(toks:&Vec<parser::Token>) {
    let mut i = 0;
    for ref tok in toks {
        match **tok {
             parser::Token::OpenTag(ref td) => {
                match td.name {
                    "div" => {
                        println!("divtext = {:?}", td.full_text);
                        let mut s = String::new();


                        let stdin = std::io::stdin();
                        stdin.lock().lines().next();
                        i += 1
                    }
                    _ => {}


                }
            },
            _=> {}

        }
    }

    println!("i = {}", i);
}
fn main() {


//    let html = r##"
//        <html>
//            <div
//                class="test">
//            </div>
//        </html>
//        "##;
//    let toks = parser::build_tokens_list(html);
//    println!("{:?}", toks);
//
//    assert_eq!(toks.len(), 4);



    let the_html = include_str!("../htmls/twitter.html");
    let toks = parser::build_tokens_list(the_html);
    let tree = parser::parse_tree(&toks);

    println!("twitter={:?}", tree);


}
