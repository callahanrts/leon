#[cfg(test)]
use super::*;

#[test]
// Encountering ']' should:
//   Change to the CDataSectionEndState
fn right_bracket() {
    let mut t = Tokenizer::new("]");
    match *t.consume_cdata_section_end_state().first().unwrap() {
        Token::CharToken(ref c) => assert_eq!(*c, ']'),
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Change to the DataState
fn greater_than() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_cdata_section_end_state();
    assert_eq!(tokens.len(), 2);

    match t.state {
        State::CDataSectionState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Emit 2 ] tokens
//   Change to the CDataSectionState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_cdata_section_end_state();
    assert_eq!(tokens.len(), 2);
    for token in tokens {
        match token {
            Token::CharToken(ref c) => assert_eq!(*c, ']'),
            _ => assert!(false)
        }
    }

    match t.state {
        State::CDataSectionState => assert!(true),
        _ => assert!(false)
    }
}
