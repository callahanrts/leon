#[cfg(test)]
use super::*;

#[test]
// An exclamation should result in moving to a MarkupDeclarationOpenState
fn exclamation_open_declaration_state() {
    let mut t = Tokenizer::new("!");
    let tokens = t.consume_tag_open_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::MarkupDeclarationOpenState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// A solidus '/' should result in moving to an EndTagOpenState
fn solidus_end_tag_open_state() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_tag_open_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::EndTagOpenState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering an ascii character should send the state machine to the TagNameState
// It should also reconsume the current character and return an empty StartTagToken
fn ascii_character_tag_name_state() {
    let mut t = Tokenizer::new("abc");
    let tokens = t.consume_tag_open_state();
    assert_eq!(tokens.len(), 0);
    // Reconsumed
    assert_eq!(t.next_char(), 'a');
    // match t.current_token {
    //     Some(token) => {
    //         match token {
    //             Token::StartTagToken(tag) => {
    //                 assert_eq!(tag.name, "");
    //             },
    //             _ => assert!(false),
    //         }
    //     },
    //     _ => assert!(false),
    // }

    match t.state {
        State::TagNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// When encountering a ?, the state machine should reconsume the current character
// and change state to the BogusCommentState--with the comment value the empty string
fn question_bogus_comment_state() {
    let mut t = Tokenizer::new("?");
    let tokens = t.consume_tag_open_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), '?');
    match t.state {
        State::BogusCommentState => assert!(true),
        _ => assert!(false),
    }
    // match t.current_token {
    //     None => assert!(false),
    //     Some(token) => {
    //         match token {
    //             Token::CommentToken(c) => {
    //                 assert_eq!(c, "");
    //             },
    //             _ => assert!(false),
    //         }
    //     },
    // }

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
    match *t.consume_tag_open_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, expected),
        _ => assert!(false),
    }
}
