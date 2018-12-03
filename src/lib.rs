

use std::str;
use std::fmt;
use std::iter;


pub enum HtmlNode<'a> {
    El(Element<'a>),
    Text(&'a str)
}

impl<'a> HtmlNode<'a> {
    fn display_as_tree(&self, indent:usize, mut f:&mut fmt::Formatter) -> fmt::Result {
        let indent_str = iter::repeat(' ').take(indent).collect::<String>();

        match *self {
            HtmlNode::El(ref el) => {
                write!(f, "{}<{}>\n", indent_str, el.tag_data.name)?;
                for (i, ref child) in (&el.children).iter().enumerate() {
                    child.display_as_tree(indent+4_usize, &mut f)?;


                    if i != el.children.len()-1 {
                        write!(f, "\n")?;
                    }
                }

                if el.children.len() > 0 {
                    write!(f, "\n{}</{}>", indent_str, el.tag_data.name)?;
                }
                    else {
                        write!(f, "{}</{}>", indent_str, el.tag_data.name)?;
                    }
            },
            HtmlNode::Text(ref txt) => {
                write!(f, "{}{}", indent_str, txt.trim())?;
            }
        }
        Result::Ok(())
    }

    fn get_token_count(&self) -> usize {
        match *self {
            HtmlNode::El(ref element) => element.token_count,
            HtmlNode::Text(_) => 1
        }
    }
}

impl<'a> fmt::Debug for HtmlNode<'a> {
    fn fmt(&self, mut f:&mut fmt::Formatter) -> fmt::Result {
        self.display_as_tree(0, &mut f)
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    tag_data: &'a TagData<'a>,
    children: Vec<HtmlNode<'a>>,
    token_count: usize
}

#[derive(PartialEq, Eq)]
pub struct TagData<'a> {
    pub name: &'a str,
    pub full_text: &'a str,
}

impl<'a> fmt::Debug for TagData<'a>{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> TagData<'a> {
    fn new(name:&'a str, contents:&'a str) -> TagData<'a> {
        assert!(!name.contains("/"));
        TagData {name, full_text: contents }
    }
}


#[derive(PartialEq, Eq)]
pub enum Token<'a> {
    VoidTag(TagData<'a>),
    OpenTag(TagData<'a>),
    CloseTag(TagData<'a>),
    Text(&'a str),
}

impl<'a> fmt::Debug for Token<'a>{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::VoidTag(ref td) => write!(f, "<VT-{:?}/>", td),
            &Token::OpenTag(ref td) => write!(f, "<OT-{:?}>", td),
            &Token::CloseTag(ref td) => write!(f, "<CT-/{:?}>", td),
            &Token::Text(ref td) => write!(f, "<TEXT>{:?}</TEXT>", td)
        }
    }
}


fn is_void_tag_name(tag_name:&str) -> bool {
    let void_tags = ["area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link", "meta", "param", "source", "track", "wbr", "circle",
        "ellipse", "line", "path", "polygon", "polyline", "rect", "stop", "use", "!DOCTYPE"];
    void_tags.iter().any(|x| tag_name == *x)
}

fn to_str<'a>(data:&'a[u8]) -> &'a str {
    ::std::str::from_utf8(&data).unwrap()
}

//http://w3c.github.io/html-reference/syntax.html#tag-name
fn name_is_html(name:&str) -> bool {

    for char in name.chars() {
        match char {
            '0'...'9' => continue,
            'a'...'z' => continue,
            'A'...'Z' => continue,
            _=> {
                //println!("----Invalidating the name {:?}", name);
                return false
            },
        }
    }
    true
}

impl<'a> Token<'a> {
    fn from_str_checked(tok_text:&'a str) -> Option<Token<'a>> {


        if tok_text.as_bytes()[0] != '<' as u8 {
            return Some(Token::Text(tok_text));
        }

        let mut name_end_chr_i = 2_usize; // Starts at two to skip over the "</" of close tags this is fine since <> isn't html

        while tok_text.as_bytes()[name_end_chr_i] != (' ' as u8) && tok_text.as_bytes()[name_end_chr_i] != ('/' as u8) && tok_text.as_bytes()[name_end_chr_i] != ('>' as u8) && tok_text.as_bytes()[name_end_chr_i] != '\n' as u8 && tok_text.as_bytes()[name_end_chr_i] != '\r' as u8{
            name_end_chr_i += 1;
        }


        if tok_text.as_bytes()[name_end_chr_i] == '/' as u8 || is_void_tag_name(to_str(&tok_text.as_bytes()[1..name_end_chr_i])){
            // VOID TAG
            let name = &tok_text.as_bytes()[1..name_end_chr_i];

            if !name_is_html(to_str(name)) {return None}
            Some(Token::VoidTag(TagData::new(to_str(name), tok_text)))

        }
        else if tok_text.as_bytes()[1] == '/' as u8 {
            //CLOSING TAG
            let name = &tok_text.as_bytes()[2..name_end_chr_i];
            if !name_is_html(to_str(name)) {return None}
            Some(Token::CloseTag(TagData::new(to_str(name), tok_text)))

        } else {
            // OPENING TAG
            let name = &tok_text.as_bytes()[1..name_end_chr_i];
            if !name_is_html(to_str(name)) {return None}
            Some(Token::OpenTag(TagData::new(to_str(name), tok_text)))

        }
    }

    pub fn from_str(s: &'a str) -> Token<'a> {
        Token::from_str_checked(s).expect("Called from_str on an invalid token")
    }


    fn closes(&self, opener:&'a Token) -> bool {
        if let Token::OpenTag(ref opener_td) = *opener {
            match *self {
                Token::CloseTag(ref closer_td) => closer_td.name == opener_td.name,
                _ => false
            }
        }
            else {
                false
            }

    }
}



fn scan_to_end_of(full_text:&str, mut index:usize, end_str:&str) -> usize {
    while !is_start_of(full_text, end_str, index) {
        index += 1
    }
    index + end_str.len()

}


fn is_start_of(haystack:&str, needle:&str, index:usize) -> bool { //TOdo: double check

    if !haystack.is_char_boundary(index+needle.len()) {
        false
    }
    else {
        index+needle.len() <= haystack.len() && haystack[index..index+needle.len()] == *needle

    }
}



pub fn build_tokens_list<'a>(full_html:&'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::<Token<'a>>::new();
    let bytes = full_html.as_bytes();

    let mut char_index = 0_usize;


    while &full_html[char_index..char_index+5] != "<html" {
        char_index += 1
    }

    'outer: while char_index < full_html.len() {
        let mut first_non_ws_i = char_index;
        // Scan to find the first non whitespace character
        while bytes[first_non_ws_i] == ' ' as u8 || bytes[first_non_ws_i] == '\n' as u8 || bytes[first_non_ws_i] == '\r' as u8 || bytes[first_non_ws_i] == '\t' as u8 {
            first_non_ws_i += 1;

            if first_non_ws_i == full_html.len() {
                break 'outer;
            }
        }

        let first_non_ws_chr = bytes[first_non_ws_i] as char;


        if is_start_of(full_html, "<!--", first_non_ws_i) {
            char_index = scan_to_end_of(full_html, first_non_ws_i, "-->");

            continue 'outer;
        }
        else if is_start_of(full_html, "<script>", first_non_ws_i) {
            char_index = scan_to_end_of(full_html, first_non_ws_i, "</script>");
            continue 'outer;
        }
        else if first_non_ws_chr == '<' { // Scan to find the closing ">"
            let mut closing_bracket_i:usize = first_non_ws_i + 1;
            let mut inside_a_string = false;

            while !(bytes[closing_bracket_i] == '>' as u8 && !inside_a_string){

                if !inside_a_string && bytes[closing_bracket_i] == '<' as u8 {
                    closing_bracket_i -= 1;
                    break;
                }
                closing_bracket_i += 1;

                if bytes[closing_bracket_i] == '"' as u8 {
                    inside_a_string = !inside_a_string;
                }
            }

            char_index = closing_bracket_i + 1;


            if let Some(tok) = Token::from_str_checked(to_str(&bytes[first_non_ws_i..(closing_bracket_i + 1)])) {
                tokens.push(tok)
            }
        } else {
            // Scan to find the end of the text, aka the next opening bracket

            let mut next_opening_bracket_i: usize = first_non_ws_i + 1;

            while bytes[next_opening_bracket_i] != '<' as u8 {
                next_opening_bracket_i += 1
            }


            if let Some(tok) = Token::from_str_checked(to_str(&bytes[char_index..next_opening_bracket_i])) {
                tokens.push(tok);

            }
            char_index = next_opening_bracket_i;
        }
    }

    tokens
}






pub fn parse_tree<'a>(tokens:&'a [Token<'a>]) -> HtmlNode<'a> {
    let first_element = &tokens[0];
    let mut children = Vec::<HtmlNode<'a>>::new();
    assert!(tokens.len() >= 2);
    let next_element = &tokens[1];


    match *first_element {
        Token::Text(ref str_data) => {
            HtmlNode::Text(str_data)
        },
        Token::VoidTag(ref td) => {
            HtmlNode::El(Element {tag_data: td, children: children, token_count: 1_usize})
        },
        Token::OpenTag(ref td) if next_element.closes(&first_element) => {
            HtmlNode::El(Element {tag_data: td, children: children, token_count: 2_usize})
        },
        Token::OpenTag(ref td) => {

            let mut snd_el_index = 1;
            let mut tokens_used = 2; // The opener and the closer


            while !tokens[snd_el_index].closes(&first_element) {

//                println!("{:?} didn't close {:?} so gonna recusrse", tokens[snd_el_index], first_element);
                let child = parse_tree(&tokens[snd_el_index..]);
                snd_el_index += child.get_token_count();
                tokens_used += child.get_token_count();
                children.push(child);
            }

//            println!("Stack popped, my closer={:?}", tokens[snd_el_index]);

            HtmlNode::El(Element {tag_data: &td, children: children, token_count: tokens_used})

        }
        Token::CloseTag(_) => {
            panic!("Somehow first_element is a close tag, this is a bug or invalid html {:?}", first_element)
        }

    }
}











