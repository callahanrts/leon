#[cfg(test)]
use super::*;

#[test]
// If the parser is currently at the EOF, an EOF token should be returned
fn eof_should_return_eof_token() {
    let mut t = Tokenizer::new("");
    match t.consume_script_data_state() {
        Some(t) => match t {
            Token::EOFToken => assert!(true),
            _ => assert!(false),
        },
        None => assert!(false),
    }
}

#[test]
// A '<' denotes the beginning of a tag so the state should be changed to
// a RawtextLessThanSignState
fn less_than_should_open_tag_state() {
    let mut t = Tokenizer::new("<div>");
    match t.consume_script_data_state() {
        Some(t) => assert!(false),
        None => {
            match t.state {
                State::ScriptDataLessThanSignState => assert!(true),
                _ => assert!(false),
            }
        }
    }
}

#[test]
// In the case of a null character '\u{0000}', it is a parse error. Return a CharToken
fn null_should_return_char_token() {
    let mut t = Tokenizer::new("\u{0000}");
    assert_char_token(&mut t, '\u{FFFD}');
}

#[test]
// For any other character, return a CharToken
fn chars_should_return_char_token() {
    let mut t = Tokenizer::new("abc");
    assert_char_token(&mut t, 'a');
}

fn assert_char_token(t: &mut Tokenizer, expected: char) {
    match t.consume_script_data_state() {
        Some(t) => {
            match t {
                Token::CharToken(c) => assert_eq!(c, expected),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}
