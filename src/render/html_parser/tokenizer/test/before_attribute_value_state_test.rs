#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace characters should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_before_attr_value_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering a '"' character should:
//   Switch to the AttrValueDoubleQuotedState
fn double_quote() {
    let mut t = Tokenizer::new("\"");
    let tokens = t.consume_before_attr_value_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AttrValueDoubleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a ''' character should:
//   Switch to the AttrValueSingleQuotedState
fn single_quote() {
    let mut t = Tokenizer::new("\'");
    let tokens = t.consume_before_attr_value_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AttrValueSingleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '>' character should:
//   Reconsume the character
//   Change to the AttributeValueUnquotedState
fn greater_than() {
    let mut t = Tokenizer::new(">");
    let tokens = t.consume_before_attr_value_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AttrValueUnquotedState => assert!(true),
        _ => assert!(false)
    }
}


#[test]
// Encountering anything else should:
//   Reconsume the character
//   Change to the AttributeValueUnquotedState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_before_attr_value_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AttrValueUnquotedState => assert!(true),
        _ => assert!(false)
    }
}


