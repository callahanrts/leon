#[cfg(test)]
use super::*;

#[test]
// Encountering a hex digit should:
//   Reconsume the char
//   Change to the HexCharReferenceState
fn hex() {
    let mut t = Tokenizer::new("3aB");
    t.consume_hex_char_reference_state();
    t.consume_hex_char_reference_state();
    let tokens = t.consume_hex_char_reference_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.char_reference_code, 939);
}

#[test]
// Encountering a ';' should:
//   Change to the CharReferenceEndState
fn semicolon() {
    let mut t = Tokenizer::new(";");
    let tokens = t.consume_hex_char_reference_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::CharReferenceEndState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Reconsume the character
//   Change to the NumericCharReferenceEndState
fn anything_else() {
    let mut t = Tokenizer::new("J");
    let tokens = t.consume_hex_char_reference_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), 'J');
    match t.state {
        State::NumericCharReferenceEndState => assert!(true),
        _ => assert!(false)
    }
}

