#[cfg(test)]
use super::*;

#[test]
// Encountering an ascii character should:
//   Set the current token to a new tag token with the empty string as its name
//   Reconsume the char
//   Change state to the RawtextEndTagNameState
fn ascii() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_rawtext_end_tag_open_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::EndTagToken(tag) => {
            assert_eq!(tag.name, String::new());
        },
        _ => assert!(false)
    }

    match t.state {
        State::RawtextEndTagNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Reconsume the character
//   Change state to the RawtextState
//   Emit '<' and '/' tokens
fn anything_else() {
    let mut t = Tokenizer::new("*");
    let tokens = t.consume_rawtext_end_tag_open_state();
    let mut return_string = String::new();
    for token in tokens {
        match token {
            Token::CharToken(c) => return_string.push(c),
            _ => assert!(false)
        }
    }
    assert_eq!(return_string, "</");

    match t.state {
        State::RawtextState => assert!(true),
        _ => assert!(false)
    }
    assert_eq!(t.next_char(), '*');
}
