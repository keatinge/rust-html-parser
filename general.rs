extern crate html_parser_f;
use html_parser_f::*;



#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! open_tag {
        ($name:expr, $contents:expr) => {
            Token::OpenTag(TagData {name: $name, full_text: $contents})
        }
    }

    macro_rules! text_tag {
        ($text:expr) => {
            Token::Text($text)
        }
    }

    macro_rules! close_tag {
        ($name:expr, $contents:expr) => {
            Token::CloseTag(TagData {name: $name, full_text: $contents})
        }
    }

    macro_rules! void_tag {
        ($name:expr, $contents:expr) => {
            Token::VoidTag(TagData {name: $name, full_text: $contents})
        }
    }

    #[test]
    fn test_tokenize_open_tags() {
        let regular_tok= "<html>";
        let spaces_tok = "<div class=\"1235\" id=\"22\">";
        let many_spaces = "<p       >";


        assert_eq!(Token::OpenTag(TagData {name: "html", full_text:regular_tok}), Token::from_str(regular_tok));
        assert_eq!(Token::OpenTag(TagData {name : "div", full_text:spaces_tok}), Token::from_str(spaces_tok));
        assert_eq!(Token::OpenTag(TagData {name : "p", full_text:many_spaces}), Token::from_str(many_spaces));
    }

    #[test]
    fn test_tokenize_close_tags() {

        let regular_tok = "</html>";
        let spaces_tok = "</div   >";

        assert_eq!(Token::CloseTag(TagData {name: "html", full_text: regular_tok}), Token::from_str(regular_tok));
        assert_eq!(Token::CloseTag(TagData {name: "div", full_text: spaces_tok}), Token::from_str(spaces_tok));
    }

    #[test]
    fn test_tokenize_void_tags() {
        let regular_tok_no_backslash = "<input type=\"text\">";
        let regular_tok_w_backslash = "<br/>";
        let backslash_spc = "<img id=\"asdf\">";
        let no_backslash_space = "<area  style='color:red'   >";


        assert_eq!(Token::VoidTag(TagData {name: "input", full_text:regular_tok_no_backslash}), Token::from_str(regular_tok_no_backslash));
        assert_eq!(Token::VoidTag(TagData {name: "br", full_text:regular_tok_w_backslash}), Token::from_str(regular_tok_w_backslash));
        assert_eq!(Token::VoidTag(TagData {name: "img", full_text:backslash_spc}), Token::from_str(backslash_spc));
        assert_eq!(Token::VoidTag(TagData {name: "area", full_text:no_backslash_space}), Token::from_str(no_backslash_space));
    }




    #[test]
    fn test_tokenize_text() {
        let text = " the quick brown fox";

        assert_eq!(Token::Text(text), Token::from_str(text));
    }


    #[test]
    fn test_tokenize_entire_simple() {
        let html = r#"
        <!DOCTYPE HTML>
        <html>
            <div>
                <input type="text">
                Some Text
            </div>

        </html>
        "#;


        let tokens = build_tokens_list(html);

        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 6_usize);
        assert_eq!(tokens, vec![open_tag!("html", "<html>"), open_tag!("div", "<div>"), void_tag!("input", "<input type=\"text\">"), text_tag!("\n                Some Text\n            "), close_tag!("div", "</div>"), close_tag!("html", "</html>")])
    }

    #[test]
    fn test_tokenize_entire_comment() {
        let html = r#"
        <html>
            <!-- <div> <div <div> <p> hello </p> </div></div> --></html>
        "#;

        let tokens = build_tokens_list(html);
        assert_eq!(tokens.len(), 2_usize);
        assert_eq!(tokens, vec![open_tag!("html", "<html>"), close_tag!("html", "</html>")]);
    }


    #[test]
    fn test_tokenize_entire_script() {
        let inp = "<html><script><div><p></p></div></script></html>";

        let tokens = build_tokens_list(inp);
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_tokenize_with_bad_attrs() {
        let input = "<html><div name=\"<% variable %>\"></div></html>";
        println!("input for test = {}", input);
        let tokens = build_tokens_list(input);
        assert_eq!(tokens.len(), 4);
    }


    #[test]
    fn test_tokenize_for_bs_tag() {

        let input = "<html><%- i'm not a tag don't parse me %></html>";
        let tokens = build_tokens_list(input);
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn actual_broken_html_from_reddit() {
        let html = r##"
        <html>
            <li class="report-reason-item report-reason-reddit"
                <div class="asdff">
                    this is some messed up html
                </div>
            </li>
        </html>"##;
        let tokens = build_tokens_list(html);
        println!("Tokens = {:?}", tokens);
        assert_eq!(tokens.len(), 7);
    }


    #[test]
    fn actual_html_from_twitter() {
        let html = r##"
        <html>
            <div
                class="test">
            </div>
        </html>
        "##;
        let toks = build_tokens_list(html);
        assert_eq!(toks.len(), 4);
    }

    #[test]
    fn more_twitter() {

        let html = r##"<html>            <div class="AdaptiveMediaOuterContainer">
    <div class="AdaptiveMedia


        is-video

        has-autoplayable-media
        "
      >
      <div class="AdaptiveMedia-container">
          <div class="AdaptiveMedia-video">
  <div class="AdaptiveMedia-videoContainer">
      <div class="PlayableMedia PlayableMedia--video">


  <div class="PlayableMedia-container">
    <div
      class="PlayableMedia-player

        "
      data-playable-media-url=""
        data-use-react-player


        data-border-top-left-radius=""
        data-border-top-right-radius=""
        data-border-bottom-left-radius=""
        data-border-bottom-right-radius=""
      style="padding-bottom: 56.25%; background-image:url('https://pbs.twimg.com/media/DSu3bGXW4AEWhZZ.jpg')">
    </div>

  </div>
</div>

  </div>
</div>

      </div>
    </div>
  </div>
</html>"##;

        let toks = build_tokens_list(html);
        println!("toks={:?}", toks);
        assert_eq!(toks.len(), 18);
    }



}