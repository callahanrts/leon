#[cfg(test)]
use super::*;

#[test]
// Encountering a hyphen character should:
//   Change to ScriptDataEscapeStartDashState
//   Emit a hyphen character token
fn hyphen() {
    let mut t = Tokenizer::new("-");

    match *t.consume_script_data_escape_start_dash_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '-'),
        _ => assert!(false),
    }
    match t.state {
        State::ScriptDataEscapedDashDashState => assert!(true),
        _ => assert!(false)
    }
}


#[test]
// Encountering a anything else should:
//   Change to ScriptDataState
//   Reconsume the character
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_script_data_escape_start_dash_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), 'a');

    match t.state {
        State::ScriptDataState => assert!(true),
        _ => assert!(false)
    }
}
