#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace or [/>] characters should:
//   If 'script' is in the tmp buffer
//     Change state to ScriptDataEscapedState
//   Otherwise
//     Change state to ScriptDataDoubleEscapedState
//   Return the character in a CharToken
fn whitespace() {
    // With nothing in the tmp buffer
    let mut t = Tokenizer::new(" ");
    match *t.consume_script_data_double_escape_end_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, ' '),
        _ => assert!(false)
    }
    match t.state {
        State::ScriptDataDoubleEscapedState => assert!(true),
        _ => assert!(false)
    }

    // With 'script' in the tmp buffer
    let mut t = Tokenizer::new(" ");
    t.tmp_buffer = String::from("script");
    match *t.consume_script_data_double_escape_end_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, ' '),
        _ => assert!(false)
    }
    match t.state {
        State::ScriptDataEscapedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering an upper case ascii character should:
//   Append the lower case version to the tmp buffer
//   Return the current char in a CharToken
fn uppercase_ascii() {
    let mut t = Tokenizer::new("A");
    match *t.consume_script_data_double_escape_end_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, 'A'),
        _ => assert!(false)
    }
    assert_eq!(t.tmp_buffer, String::from("a"));
}

#[test]
// Encountering a lower case ascii character should:
//   Append the char to the tmp buffer
//   Return the current char in a CharToken
fn lowercase_ascii() {
    let mut t = Tokenizer::new("a");
    match *t.consume_script_data_double_escape_end_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, 'a'),
        _ => assert!(false)
    }
    assert_eq!(t.tmp_buffer, String::from("a"));
}


#[test]
// Encountering anything else should:
//   Reconsume the character
//   Change to the ScriptDataDoubleEscapedState
fn anything_else() {
    let mut t = Tokenizer::new("*");
    let tokens = t.consume_script_data_double_escaped_less_than_sign_state();
	assert_eq!(tokens.len(), 0);

    match t.state {
        State::ScriptDataDoubleEscapedState => assert!(true),
        _ => assert!(false)
    }
	assert_eq!(t.next_char(), '*');
}



