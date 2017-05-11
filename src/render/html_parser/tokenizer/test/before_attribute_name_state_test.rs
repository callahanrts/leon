#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace characters should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_before_attr_name_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering a '/' or '>' character should:
//   Reconsume in the AfterAttributeNameState
fn end_char() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_before_attr_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AfterAttrNameState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), '/');
}

#[test]
// Encountering an '=' character should:
//   Start a new attribute with name as the char and value as the empty string
//   Switch to the AttrNameState
fn equals() {
    let mut t = Tokenizer::new("=");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    let tokens = t.consume_before_attr_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes.first().unwrap().name, String::from("="));
        },
        _ => assert!(false),
    }

    match t.state {
        State::AttrNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Start a new attribute with an empty name and value
//   Reconsume the character
//   Change to the AttrNameState
fn anything_else() {
    let mut t = Tokenizer::new("b");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    let tokens = t.consume_before_attr_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes.first().unwrap().name, String::new());
        },
        _ => assert!(false),
    }

    match t.state {
        State::AttrNameState => assert!(true),
        _ => assert!(false)
    }
}
