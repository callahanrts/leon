#[cfg(test)]
use super::*;

#[test]
// Encountering a hyphen  character should:
//   Change to the ScriptDataDoubleEscapedDashState
//   Emit a hyphen CharToken
fn hyphen() {
    let mut t = Tokenizer::new("-");
    match *t.consume_script_data_double_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '-'),
        _ => assert!(false)
    }
    match t.state {
        State::ScriptDataDoubleEscapedDashState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '<' character should:
//   Change to the ScriptDataDoubleEscapedLessThanSignState
//   Emit a '<' CharToken
fn less_than() {
    let mut t = Tokenizer::new("<");
    match *t.consume_script_data_double_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '<'),
        _ => assert!(false)
    }
    match t.state {
        State::ScriptDataDoubleEscapedLessThanSignState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a null char U+0000 character should:
//   Emit a U+FFFD replacement CharToken
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    match *t.consume_script_data_double_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '\u{FFFD}'),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Return the character CharToken
fn anything_else() {
    let mut t = Tokenizer::new("a");
    match *t.consume_script_data_double_escaped_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, 'a'),
        _ => assert!(false)
    }
}
