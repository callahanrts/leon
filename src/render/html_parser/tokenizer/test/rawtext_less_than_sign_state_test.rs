#[cfg(test)]
use super::*;

#[test]
// Encountering a solidus '/' should:
//   Set the temporary buffer to the empty string
//   Change state to RawtextEndTagOpenState
fn solidus() {
    let mut t = Tokenizer::new("/");
    t.tmp_buffer = String::from("test");
    let tokens = t.consume_rawtext_less_than_sign_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.tmp_buffer, String::new());
    match t.state {
        State::RawtextEndTagOpenState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Emit a '<' character token
//   Reconsume character
//   Enter the RawtextState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    match *t.consume_rawtext_less_than_sign_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '<'),
        _ => assert!(false)
    }
    match t.state {
        State::RawtextState => assert!(true),
        _ => assert!(false)
    }
    assert_eq!(t.next_char(), 'a');
}
