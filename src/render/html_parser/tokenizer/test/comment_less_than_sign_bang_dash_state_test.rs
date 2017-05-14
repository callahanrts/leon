#[cfg(test)]
use super::*;

#[test]
// Encountering '-' should:
//   Change to the less than sign bang dash dash state
fn bang() {
    let mut t = Tokenizer::new("!");
    let tokens = t.consume_comment_less_than_sign_bang_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentLessThanSignBangDashDashState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Change to the comment end dash state
//   Reconsume the character
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_comment_less_than_sign_bang_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndDashState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}
