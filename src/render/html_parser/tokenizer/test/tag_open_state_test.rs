#[cfg(test)]
use super::*;

#[test]
// An exclamation should result in moving to a MarkupDeclarationOpenState
fn exclamation_open_declaration_state() {
    let mut t = Tokenizer::new("!");
    match t.consume_tag_open_state() {
        Some(t) => assert!(false),
        None => {
            match t.state {
                State::MarkupDeclarationOpenState => assert!(true),
                _ => assert!(false)
            }
        }
    }
}

#[test]
// A solidus '/' should result in moving to an EndTagOpenState
fn solidus_end_tag_open_state() {
    let mut t = Tokenizer::new("/");
    match t.consume_tag_open_state() {
        Some(t) => assert!(false),
        None => {
            match t.state {
                State::EndTagOpenState => assert!(true),
                _ => assert!(false)
            }
        }
    }
}

#[test]
// Encountering an ascii character should send the state machine to the TagNameState
// It should also reconsume the current character and return an empty StartTagToken
fn ascii_character_tag_name_state() {
    let mut t = Tokenizer::new("abc");
    match t.consume_tag_open_state() {
        Some(token) => {
            match token {
                Token::StartTagToken(tag) => {
                    assert_eq!(tag.name, "");
                },
                _ => assert!(false),
            }

            match t.state {
                State::TagNameState => assert!(true),
                _ => assert!(false)
            }
        },
        None => assert!(false),
    }
    // Reconsumed
    assert_eq!(t.next_char(), 'a');
}

#[test]
// When encountering a ?, the state machine should reconsume the current character
// and change state to the BogusCommentState--with the comment value the empty string
fn question_bogus_comment_state() {
    let mut t = Tokenizer::new("?");
    match t.consume_tag_open_state() {
        Some(token) => {
            match token {
                Token::CommentToken(c) => {
                    assert_eq!(c, "");
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }

    match t.state {
        State::BogusCommentState => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(t.next_char(), '?');
}

#[test]
// For any other character, return a CharToken
fn other_should_return_less_than_token() {
    let mut t = Tokenizer::new("*");
    assert_char_token(&mut t, '<');
    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false),
    }
}

fn assert_char_token(t: &mut Tokenizer, expected: char) {
    match t.consume_tag_open_state() {
        Some(t) => {
            match t {
                Token::CharToken(c) => assert_eq!(c, expected),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}
