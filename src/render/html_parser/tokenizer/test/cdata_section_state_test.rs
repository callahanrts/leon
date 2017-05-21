#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
//   Return an EOF Token
fn eof() {
    let mut t = Tokenizer::new("");
    match *t.consume_cdata_section_state().first().unwrap() {
        Token::EOFToken => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering ']' should:
//   Change to the CdataSectionBracketState
fn right_bracket() {
    let mut t = Tokenizer::new("]");
    let tokens = t.consume_cdata_section_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::CDataSectionBracketState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Return the character as a CharToken
fn anything_else() {
    let mut t = Tokenizer::new("a");
    match *t.consume_cdata_section_state().first().unwrap() {
        Token::CharToken(ref c) => assert_eq!(*c, 'a'),
        _ => assert!(false)
    }
}

