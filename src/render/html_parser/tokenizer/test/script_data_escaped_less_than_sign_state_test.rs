#[cfg(test)]
use super::*;

#[test]
// Encountering a solidus character should:
//   Set the temp buffer to an empty string
//   Change to ScriptDataEscapedEndTagOpenState
fn solidus() {
    let mut t = Tokenizer::new("/");

    let tokens = t.consume_script_data_escaped_less_than_sign_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::ScriptDataEscapedEndTagOpenState => assert!(true),
        _ => assert!(false)
    }

}

#[test]
// Encountering an ascii character should:
//   Set the temp buffer to the empty string
//   Reconsume the character
//   Change to the ScriptDataDoubleEscapeStartState
//   Emit a '<' token
fn ascii() {
    let mut t = Tokenizer::new("a");
    match *t.consume_script_data_escaped_less_than_sign_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '<'),
        _ => assert!(false)
    }

    match t.state {
        State::ScriptDataDoubleEscapeStartState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 'a')
}


#[test]
// Encountering anything else should:
//   Reconsume the char
//   Change to the ScriptDataEscapedState
//   Emit a '<' token
fn anything_else() {
    let mut t = Tokenizer::new("*");
    match *t.consume_script_data_escaped_less_than_sign_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '<'),
        _ => assert!(false)
    }

    match t.state {
        State::ScriptDataEscapedState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), '*')
}
