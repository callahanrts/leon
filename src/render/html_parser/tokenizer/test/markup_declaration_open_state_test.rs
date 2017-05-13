#[cfg(test)]
use super::*;

#[test]
// Encountering -- should:
//   Consume the 2 characters
//   Set the current token to a CommentToken with the empty string
//   Change to the Comment Start State
fn dashdash() {
    let mut t = Tokenizer::new("--a");
    let tokens = t.consume_markup_declaration_open_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::CommentStartState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 'a');
}

#[test]
// Encountering starts with doctype (case insensitive) should:
//   Consume the 7 chars
fn greater_than() {
    let mut t = Tokenizer::new("DOCtypea");
    t.current_token = Some(Token::CommentToken(String::from("test")));
    let tokens = t.consume_markup_declaration_open_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPEState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 'a');
}

// #[test]
// TODO
// Encountering some complicated shit
// fn nullchar() {
//     let mut t = Tokenizer::new("\u{0000}");
//     t.current_token = Some(Token::CommentToken(String::from("te")));
//     let tokens = t.consume_markup_declaration_open_state();
//     assert_eq!(tokens.len(), 0);

//     match t.current_token() {
//         Token::CommentToken(comment) => {
//             assert_eq!(comment, String::from("te\u{FFFD}"));
//         },
//         _ => assert!(false)
//     }
// }

#[test]
// Encountering anything else
//   Parse error
//   Create a comment token with the empty string
//   Switch to the bogus comment state (don't consume anything here)
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_markup_declaration_open_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::CommentToken(comment) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::BogusCommentState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}

