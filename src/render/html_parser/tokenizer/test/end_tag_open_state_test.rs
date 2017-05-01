#[cfg(test)]
use super::*;

#[test]
// If the parser is currently at the EOF, an EOF token should be returned
fn eof_should_return_eof_token() {
    let mut t = Tokenizer::new("");
    match t.consume_rawtext_state() {
        Some(t) => match t {
            Token::EOFToken => assert!(true),
            _ => assert!(false),
        },
        None => assert!(false),
    }
}

#[test]
// Encountering an ascii character should send the state machine to the TagNameState
// It should also reconsume the current character and return an empty StartTagToken
fn ascii_character_tag_name_state() {
    let mut t = Tokenizer::new("abc");
    match t.consume_end_tag_open_state() {
        Some(tokens) => {
            match tokens[0] {
                Token::EndTagToken(ref tag) => {
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
// Encountering a '>' will send the state machine to the DataState
fn greater_than_data_state() {
    let mut t = Tokenizer::new(">");
    match t.consume_end_tag_open_state() {
        Some(tokens) => assert!(false),
        None => assert!(true),
    }
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
    match t.consume_end_tag_open_state() {
        Some(tokens) => {
            match tokens[0] {
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
    assert_eq!(t.next_char(), '*');
}
