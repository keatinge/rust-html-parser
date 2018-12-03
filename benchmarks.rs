#![feature(test)]
extern crate test;

extern crate html_parser_f;
use html_parser_f::*;

#[cfg(test)]
mod bench {
    use super::*;
    #[bench]
    fn twitter_bench_tokenizer(b: &mut test::Bencher) {
        let the_html = include_str!("../htmls/twitter.html");
        b.iter(|| build_tokens_list(the_html));
    }

    #[bench]
    fn reddit_bench_tokenizer(b: &mut test::Bencher) {
        let html = include_str!("../htmls/reddit.html");
        b.iter(|| build_tokens_list(html));
    }

    #[bench]
    fn stack_bench_tokenizer(b: &mut test::Bencher) {
        let html = include_str!("../htmls/stack.html");
        b.iter(|| build_tokens_list(html));
    }

    #[bench]
    fn twitter_bench_tree(b: &mut test::Bencher) {
        let the_html = include_str!("../htmls/twitter.html");
        let toks = build_tokens_list(the_html);

        b.iter(|| parse_tree(&toks));

    }

    #[bench]
    fn stack_bench_tree(b: &mut test::Bencher) {
        let the_html = include_str!("../htmls/stack.html");
        let toks = build_tokens_list(the_html);

        b.iter(|| parse_tree(&toks));
    }

    #[bench]
    fn reddit_bench_tree(b: &mut test::Bencher) {
        let the_html = include_str!("../htmls/reddit.html");
        let toks = build_tokens_list(the_html);

        b.iter(|| parse_tree(&toks));
    }
}


