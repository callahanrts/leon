#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace characters should:
//   Reconsume the character
//   Change to the AfterAttrNameState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_attr_name_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), ' ');
    match t.state {
        State::AfterAttrNameState => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Encountering an '=' character should:
//   Switch to the BeforeAttrNameState
fn equals() {
    let mut t = Tokenizer::new("=");
    let tokens = t.consume_attr_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering uppercase ascii character should:
//   Append the lowercase version to the attribute name
fn uppercase_ascii() {
    let mut t = Tokenizer::new("A");
    t.current_token = Some(token());
    let tokens = t.consume_attr_name_state();
    assert_eq!(tokens.len(), 0);
    assert_attr_name(&mut t, String::from("ba"));
}

#[test]
// Encountering a '"' character should:
//   It's a parse error
//   Append the character to the attribute name
fn quote() {
    let mut t = Tokenizer::new("\"");
    t.current_token = Some(token());
    let tokens = t.consume_attr_name_state();
    assert_eq!(tokens.len(), 0);
    assert_attr_name(&mut t, String::from("b\""));
}

#[test]
// Encountering anything else should:
//   Append the character to the attribute name
fn anything_else() {
    let mut t = Tokenizer::new("a");
    t.current_token = Some(token());
    let tokens = t.consume_attr_name_state();
    assert_eq!(tokens.len(), 0);
    assert_attr_name(&mut t, String::from("ba"));
}

fn token() -> Token {
    let mut tag = Tag::new(String::from("test"));
    tag.append_attribute(Attribute{
        name: String::from("b"),
        value: String::new(),
    });
    Token::StartTagToken(tag)
}

fn assert_attr_name(t: &mut Tokenizer, name: String) {
    match t.current_token() {
        Token::StartTagToken(tag) => {
            assert_eq!(tag.attributes[0].name, name);
        },
        _ => assert!(false),
    }
}
