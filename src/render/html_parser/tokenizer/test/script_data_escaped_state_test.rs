#[cfg(test)]
use super::*;

#[test]
// Encountering an EOF
//   Emit an EOF token
fn eof() {
    let mut t = Tokenizer::new("");
    match *t.consume_script_data_escaped_state().first().unwrap() {
        Token::EOFToken => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Encountering a hyphen character should:
//   Change to ScriptDataEscapedDashState
//   Emit a hyphen character token
fn hyphen() {
    let mut t = Tokenizer::new("-");

    match *t.consume_script_data_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '-'),
        _ => assert!(false),
    }
    match t.state {
        State::ScriptDataEscapedDashState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '<' character should:
//   Change to ScriptDataEscapedLessThanSignState
fn less_than() {
    let mut t = Tokenizer::new("<");
    let tokens = t.consume_script_data_escaped_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::ScriptDataEscapedLessThanSignState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a NULL, U+0000 character should:
//   Emit a U+FFFD, replacement char token
fn null_char() {
    let mut t = Tokenizer::new("\u{0000}");
    match *t.consume_script_data_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '\u{FFFD}'),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   return the char as a CharToken
fn anything_else() {
    let mut t = Tokenizer::new("a");
    match *t.consume_script_data_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, 'a'),
        _ => assert!(false)
    }
}
