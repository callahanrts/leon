#[cfg(test)]
use super::*;

#[test]
// Encountering a hex digit should:
//   Reconsume the char
//   Change to the HexCharReferenceState
fn hex() {
    let mut t = Tokenizer::new("0");
    let tokens = t.consume_decimal_char_reference_start_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), '0');
    match t.state {
        State::DecimalCharReferenceStartState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Reconsume teh char
//   Change to teh CharReferenceEndState
fn anything_else() {
    let mut t = Tokenizer::new("J");
    let tokens = t.consume_decimal_char_reference_start_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), 'J');
    match t.state {
        State::CharReferenceEndState => assert!(true),
        _ => assert!(false)
    }
}
