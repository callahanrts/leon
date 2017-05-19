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
    let tokens = t.consume_comment_state();
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
// Encountering '<' should:
//   Append the current char to the comment
//   Change to the CommentLessThanSignState
fn less_than() {
    let mut t = Tokenizer::new("<");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "test<");
        },
        _ => assert!(false)
    }

    match t.state {
        State::CommentLessThanSignState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '-' should:
//   Change to the CommentEndState
fn hyphen() {
    let mut t = Tokenizer::new("-");
    let tokens = t.consume_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndDashState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '\u{0000}' should:
//   Append a replacement char to the comment token
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "test\u{FFFD}");
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Append the character to the comment
fn anything_else() {
    let mut t = Tokenizer::new("s");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "tests");
        },
        _ => assert!(false)
    }

}


