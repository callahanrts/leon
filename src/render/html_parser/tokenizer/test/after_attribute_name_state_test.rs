#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace characters should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_after_attr_name_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering a '/' character should:
//   Switch to the SelfClosingStartTagState
fn solidus() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_after_attr_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::SelfClosingStartTagState => assert!(true),
        _ => assert!(false)
    }
}


#[test]
// Encountering an '=' character should:
//   Switch to the BeforeAttrNameState
fn equals() {
    let mut t = Tokenizer::new("=");
    let tokens = t.consume_after_attr_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrValueState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '>' character should:
//   Switch to the data state
//   Emit the current tag token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));

    match *t.consume_after_attr_name_state().first().unwrap() {
        Token::StartTagToken(_) => assert!(true),
        _ => assert!(false)
    };

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}


#[test]
// Encountering anything else should:
//   Append the character to the attribute name
fn anything_else() {
    let mut t = Tokenizer::new("a");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    let tokens = t.consume_after_attr_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes.first().unwrap().name, String::new());
        },
        _ => assert!(false),
    }
}

