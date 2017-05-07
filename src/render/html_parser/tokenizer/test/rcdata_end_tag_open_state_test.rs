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
// Encountering an ascii character should send the state machine to the RCDataEndTagNameState
// It should also reconsume the current character and return an empty EndTagToken
fn ascii_character_tag_name_state() {
    let mut t = Tokenizer::new("abc");
    let tokens = t.consume_rcdata_end_tag_open_state();
    assert_eq!(tokens.len(), 0);
    // Reconsumed
    assert_eq!(t.next_char(), 'a');

    match t.state {
        State::RCDataEndTagNameState => assert!(true),
        _ => assert!(false)
    }

    // Should set the current token to EndTagToken with an empty string
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
// Encountering anything else should reconsume the char, emit '<' and '/' character tokens,
// and change to the RCDataState
fn greater_than_data_state() {
}
