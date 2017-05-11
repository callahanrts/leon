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
