#[cfg(test)]
use super::*;

#[test]
// Encountering a '"' character should:
//   Change to the AfterAttrValueQuotedState
fn single_quote() {
    let mut t = Tokenizer::new("'");
    let tokens = t.consume_attr_value_single_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::AfterAttrValueQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '&' character should:
//   Set the return state to the AttrValueDoubleQuotedState
//   Change state to the CharacterReferenceState
fn ampersand() {
    let mut t = Tokenizer::new("&");
    let tokens = t.consume_attr_value_single_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.return_state {
        State::AttrValueDoubleQuotedState => assert!(true),
        _ => assert!(false)
    }
    match t.state {
        State::CharReferenceState => assert!(true),
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
    let tokens = t.consume_attr_value_single_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].value, String::from("te\u{FFFD}"));
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
    let tokens = t.consume_attr_value_single_quoted_state();
    assert_eq!(tokens.len(), 0);
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].value, String::from("tes"));
        },
        _ => assert!(false)
    }
}
