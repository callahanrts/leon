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
// Encountering '-' should:
//   Change to the comment end state
fn hyphen() {
    let mut t = Tokenizer::new("-");
    let tokens = t.consume_comment_end_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentEndState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Append a hyphen to the comment
//   Reconsume the character
//   Change to the comment state
fn anything_else() {
    let mut t = Tokenizer::new("s");
    t.current_token = Some(Token::CommentToken(String::from("s")));
    let tokens = t.consume_comment_end_dash_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "s-");
        },
        _ => assert!(false)
    }

    match t.state {
        State::CommentState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}
