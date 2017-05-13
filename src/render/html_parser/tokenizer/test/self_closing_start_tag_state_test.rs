#[cfg(test)]
use super::*;

#[test]
// Encountering a greater than should:
//   Set the self-closing flag on the current tag token
//   Switch to the DataState
//   Emit the current tag token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
    match *t.consume_self_closing_start_tag_state().first().unwrap() {
        Token::StartTagToken(ref tag) => {
            assert_eq!(tag.self_closing, true);
        },
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
    let tokens = t.consume_self_closing_start_tag_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}




