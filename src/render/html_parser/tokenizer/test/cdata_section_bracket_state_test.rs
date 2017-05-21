#[cfg(test)]
use super::*;

#[test]
// Encountering ']' should:
//   Change to the CDataSectionEndState
fn right_bracket() {
    let mut t = Tokenizer::new("]");
    let tokens = t.consume_cdata_section_bracket_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CDataSectionEndState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Return the character as a CharToken
fn anything_else() {
    let mut t = Tokenizer::new("a");
    match *t.consume_cdata_section_bracket_state().first().unwrap() {
        Token::CharToken(ref c) => assert_eq!(*c, ']'),
        _ => assert!(false)
    }

    match t.state {
        State::CDataSectionState => assert!(true),
        _ => assert!(false)
    }
}
