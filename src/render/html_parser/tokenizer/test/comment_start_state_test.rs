#[cfg(test)]
use super::*;

#[test]
// Encountering '-' should:
//   Change to the CommentStartDashState
fn hyphen() {
    let mut t = Tokenizer::new("-");
    let tokens = t.consume_comment_start_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentStartDashState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering starts with doctype (case insensitive) should:
//   Parse Error
//   Change to the DataState
//   Emit the current comment token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::CommentToken(String::from("test")));

    match *t.consume_comment_start_state().first().unwrap() {
        Token::CommentToken(ref comment) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Reconsume the character
//   Change to the comment state
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_comment_start_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}


