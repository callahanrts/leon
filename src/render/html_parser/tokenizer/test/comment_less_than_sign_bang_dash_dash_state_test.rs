#[cfg(test)]
use super::*;

#[test]
// Encountering '>' should:
//   Reconsume the character
//   Change to the commentEndState
fn greater_than() {
    let mut t = Tokenizer::new(">");
    let tokens = t.consume_comment_less_than_sign_bang_dash_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), '>');
}

#[test]
// Encountering anything else
//   Parse Error
//   Change to the comment end state
//   Reconsume the character
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_comment_less_than_sign_bang_dash_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}
