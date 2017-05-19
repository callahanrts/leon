#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace character should:
//   Change to the BeforeAttrNameState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_attr_value_unquoted_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '&' character should:
//   Set the return state to the AttrValueUnquotedState
//   Change state to the CharacterReferenceState
fn ampersand() {
    let mut t = Tokenizer::new("&");
    let tokens = t.consume_attr_value_unquoted_state();
    assert_eq!(tokens.len(), 0);
    match t.return_state {
        State::AttrValueUnquotedState => assert!(true),
        _ => assert!(false)
    }
    match t.state {
        State::CharReferenceState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a greater than should:
//   Switch to the DataState
//   Emit the curent tag token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    match *t.consume_attr_value_unquoted_state().first().unwrap() {
        Token::StartTagToken(_) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a null character
//   Append the U+FFFD replacement character to the current attribute's value
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    let mut tag = Tag::new(String::new());
    tag.append_attribute(Attribute{
        name: String::from("test"),
        value: String::from("te"),
    });
    t.current_token = Some(Token::StartTagToken(tag));
    let tokens = t.consume_attr_value_unquoted_state();
    assert_eq!(tokens.len(), 0);
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].value, String::from("te\u{FFFD}"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering " ' < = ` should:
//   Append the current character to the current attribute's value
fn various_chars() {
    let mut t = Tokenizer::new("<");
    let mut tag = Tag::new(String::new());
    tag.append_attribute(Attribute{
        name: String::from("test"),
        value: String::from("te"),
    });
    t.current_token = Some(Token::StartTagToken(tag));
    let tokens = t.consume_attr_value_unquoted_state();
    assert_eq!(tokens.len(), 0);
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].value, String::from("te<"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Append the current character to the current attribute's value
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let mut tag = Tag::new(String::new());
    tag.append_attribute(Attribute{
        name: String::from("test"),
        value: String::from("te"),
    });
    t.current_token = Some(Token::StartTagToken(tag));
    let tokens = t.consume_attr_value_unquoted_state();
    assert_eq!(tokens.len(), 0);
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].value, String::from("tes"));
        },
        _ => assert!(false)
    }
}

