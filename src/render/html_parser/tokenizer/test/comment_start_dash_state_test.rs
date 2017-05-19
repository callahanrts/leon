#[cfg(test)]
use super::*;

#[test]
// Encountering EOF
//   Parse Error.
//   Emit the comment token
//   Emit an end of file token
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_start_dash_state();
    assert_eq!(tokens.len(), 2);

    for token in tokens {
        match token {
            Token::EOFToken => assert!(true),
            Token::CommentToken(_) => assert!(true),
            _ => assert!(false)
        }
    }
}

#[test]
// Encountering '-' should:
//   Change to the CommentEndState
fn hyphen() {
    let mut t = Tokenizer::new("-");
    let tokens = t.consume_comment_start_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Change to the DataState
//   Emit the current comment token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::CommentToken(String::from("test")));

    match *t.consume_comment_start_dash_state().first().unwrap() {
        Token::CommentToken(_) => assert!(true),
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
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_start_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "tests");
        },
        _ => assert!(false)
    }

    match t.state {
        State::CommentState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}

