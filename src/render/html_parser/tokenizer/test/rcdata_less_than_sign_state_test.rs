#[cfg(test)]
use super::*;

#[test]
// A solidus char should send the state machine to an RCDataEndTagOpenState
fn solidus_send_end_tag_open_state() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_rcdata_less_than_sign_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::RCDataEndTagOpenState => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Any other character should reconsume in the RCDataState and return a '<' charToken
fn reconsume_in_rcdata_state() {
    let mut t = Tokenizer::new("d");
    assert_char_token(&mut t, '<');
}

fn assert_char_token(t: &mut Tokenizer, expected: char) {
    match *t.consume_rcdata_less_than_sign_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, expected),
        _ => assert!(false),
    }
}
