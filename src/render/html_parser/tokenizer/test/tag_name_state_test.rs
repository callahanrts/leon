#[cfg(test)]
use super::*;

#[test]
// If the parser is currently at the EOF, an EOF token should be returned
fn eof_should_return_eof_token() {
    let mut t = Tokenizer::new("");
    match *t.consume_tag_name_state().first().unwrap() {
        Token::EOFToken => assert!(true),
        _ => assert!(false),
    }
}



#[test]
// Tab, Line feed, form feed, or space should send the state machine to the
// BeforeAttributeNameState
fn whitespace_to_before_attr_name_state() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_tag_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Solidus '/' should send the state machine to the SelfClosingStartTagState
fn solidus_self_close_start_tag() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_tag_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::SelfClosingStartTagState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Greater than '>' should switch to the DataState and emit the current tag token
fn gt_data_state(){
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    let tokens = t.consume_tag_name_state();
    assert!(tokens.len() > 0);
    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}


#[test]
// Upper case ASCII char should append the lowercase version of the current character to the
// current token's tag name
fn upper_ascii_tag_name() {
    assert_append_char(String::new(), "A", String::from("a"));
}

#[test]
// The null character should append a U+FFFD replacement character to the current
// token's tag name
fn null_char_tag_name() {
    assert_append_char(String::new(), "\u{0000}", String::from("\u{FFFD}"));
}

#[test]
// Anything else should append the current character to the current token's tag name
fn char_tag_name() {
    assert_append_char(String::from("a"), "b", String::from("ab"));
}

fn assert_append_char(cur: String, input: &str, end: String) {
    let mut t = Tokenizer::new(input);
    t.current_token = Some(Token::StartTagToken(Tag::new(cur)));
    let tokens = t.consume_tag_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token {
        Some(token) => {
            match token {
                Token::StartTagToken(tag) => {
                    assert_eq!(tag.name, String::from(end));
                },
                Token::EOFToken => {}
                _ => assert!(false)
            }
        },
        None => assert!(false)
    }
}
