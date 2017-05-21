#[cfg(test)]
use super::*;

#[test]
// Encountering whitespace should:
//   Reconsume the character
//   Change to the CharReferenceEndState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_char_reference_state();
    match t.state {
        State::CharReferenceEndState => assert!(true),
        _ => assert!(false)
    }
    assert_eq!(t.next_char(), ' ');
}

#[test]
// Encountering '#' should:
//   Append the current char to the tmp_buffer
fn number() {
    let mut t = Tokenizer::new("#");
    let tokens = t.consume_char_reference_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.tmp_buffer, String::from("##"));

    match t.state {
        State::NumericCharReferenceState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
fn anything_else() {
}

