#[cfg(test)]
use super::*;

#[test]
// If the parser is currently at the EOF, an EOF token should be returned
fn eof_should_return_eof_token() {
    let mut t = Tokenizer::new("");
    match *t.consume_rawtext_state().first().unwrap() {
        Token::EOFToken => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Encountering an ascii character should send the state machine to the TagNameState
// It should also reconsume the current character and return an empty StartTagToken
fn ascii_character_tag_name_state() {
    let mut t = Tokenizer::new("abc");
    let tokens = t.consume_end_tag_open_state();
    assert_eq!(tokens.len(), 0);
    // Reconsumed
    assert_eq!(t.next_char(), 'a');

    match t.state {
        State::TagNameState => assert!(true),
        _ => assert!(false)
    }

    // match t.current_token {
    //     None => assert!(false),
    //     Some(token) => {
    //         match token {
    //             Token::EndTagToken(ref tag) => {
    //                 assert_eq!(tag.name, "");
    //             },
    //             _ => assert!(false),
    //         }
    //     },
    // }
}

#[test]
// Encountering a '>' will send the state machine to the DataState
fn greater_than_data_state() {
    let mut t = Tokenizer::new(">");
    let tokens = t.consume_end_tag_open_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false),
    }
}


#[test]
// When encountering non-ascii, non '>', the state machine should reconsume the current character
// and change state to the BogusCommentState--with the comment value the empty string
fn other_bogus_comment_state() {
    let mut t = Tokenizer::new("*-");
    let tokens = t.consume_end_tag_open_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::BogusCommentState => assert!(true),
        _ => assert!(false),
    }

    assert_eq!(t.next_char(), '*');

    // match t.current_token {
    //     None => assert!(false),
    //     Some(token) => {
    //         match token {
    //             Token::CommentToken(c) => {
    //                 assert_eq!(c, "");
    //             },
    //             _ => assert!(false),
    //         }
    //     }
    // }
}
