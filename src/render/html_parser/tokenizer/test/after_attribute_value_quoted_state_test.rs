#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace character should:
//   Change to the BeforeAttrNameState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_after_attr_value_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a solidus should:
//   Change to the SelfClosingStartTagState
fn solidus() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_after_attr_value_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::SelfClosingStartTagState => assert!(true),
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
    match *t.consume_after_attr_value_quoted_state().first().unwrap() {
        Token::StartTagToken(ref tag) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else
//   Parse Error. Reconsume in the BeforeAttrNameState
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_after_attr_value_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}


