#[cfg(test)]
use super::*;


#[test]
// Encountering EOF should:
//   Emit the comment.
//   Emit an end of file token
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_bogus_comment_state();
    assert_eq!(tokens.len(), 2);
    for token in tokens {
        match token {
            Token::CommentToken(comment) => {
                assert_eq!(comment, String::from("test"));
            },
            Token::EOFToken => assert!(true),
            _ => assert!(false)
        }
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a greater than should:
//   Switch to the current data state
//   Emit the comment token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    match *t.consume_bogus_comment_state().first().unwrap() {
        Token::CommentToken(ref comment) => {
            assert_eq!(*comment, String::from("test"));
        },
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering the null U+0000 character should:
//   Append the replacement character to the comment token's data
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    t.current_token = Some(Token::CommentToken(String::from("te")));
    let tokens = t.consume_bogus_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, String::from("te\u{FFFD}"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Append the current input token to the comment token's data
fn anything_else() {
    let mut t = Tokenizer::new("s");
    t.current_token = Some(Token::CommentToken(String::from("te")));
    let tokens = t.consume_bogus_comment_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, String::from("tes"));
        },
        _ => assert!(false)
    }
}





