#[cfg(test)]
use super::*;

#[test]
// Encountering '[xX]' should:
//   Append x to the tmp buffer
//   Change to the HexCharReferenceStartState
fn x() {
    let mut t = Tokenizer::new("x");
    let tokens = t.consume_numeric_char_reference_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.tmp_buffer, String::from("x"));
    match t.state {
        State::HexCharReferenceStartState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Reconsume the char
//   Change to the DecimalCharReferenceState
fn anything_else() {
    let mut t = Tokenizer::new("1");
    let tokens = t.consume_numeric_char_reference_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), '1');
    match t.state {
        State::DecimalCharReferenceStartState => assert!(true),
        _ => assert!(false)
    }
}
