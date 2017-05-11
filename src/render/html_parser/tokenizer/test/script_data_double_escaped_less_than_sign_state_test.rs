#[cfg(test)]
use super::*;

#[test]
// Encountering a hyphen  character should:
//   Change to the ScriptDataDoubleEscapedDashState
//   Emit a hyphen CharToken
fn solidus() {
    let mut t = Tokenizer::new("/");
    match *t.consume_script_data_double_escaped_less_than_sign_state().first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '/'),
        _ => assert!(false)
    }
	match t.state {
		State::ScriptDataDoubleEscapeEndState => assert!(true),
		_ => assert!(false)
    }

	assert_eq!(t.tmp_buffer, String::new());
}

#[test]
// Encountering anything else should:
//   Return the character CharToken
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_script_data_double_escaped_less_than_sign_state();
	assert_eq!(tokens.len(), 0);

    match t.state {
        State::ScriptDataDoubleEscapedState => assert!(true),
        _ => assert!(false)
    }
	assert_eq!(t.next_char(), 'a');
}


