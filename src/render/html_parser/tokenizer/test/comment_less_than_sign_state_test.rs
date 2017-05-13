#[cfg(test)]
use super::*;

#[test]
// Encountering '!' should:
//   Append the char to the comment
//   Change to the less than sign bang state
fn bang() {
    let mut t = Tokenizer::new("!");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_less_than_sign_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "test!");
        },
        _ => assert!(false)
    }

    match t.state {
        State::CommentLessThanSignBangState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '<' should:
//   Append teh character to the comment
fn less_than() {
    let mut t = Tokenizer::new("<");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_comment_less_than_sign_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => {
            assert_eq!(comment, "test<");
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Change to the comment state
//   Reconsume the character
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_comment_less_than_sign_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CommentState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}



