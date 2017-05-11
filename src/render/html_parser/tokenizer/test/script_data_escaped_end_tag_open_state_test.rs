#[cfg(test)]
use super::*;

#[test]
// Encountering an ascii character should:
//   Set the temp buffer to the empty string
//   Reconsume the character
//   Change to the ScriptDataDoubleEscapeStartState
//   Emit a '<' token
fn ascii() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_script_data_escaped_end_tag_open_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::ScriptDataEscapedEndTagNameState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 'a')
}


#[test]
// Encountering anything else should:
//   Reconsume the char
//   Change to the ScriptDataEscapedState
//   Emit a '<' token
fn anything_else() {
    let mut t = Tokenizer::new("*");
    let mut return_string = String::new();
    for token in t.consume_script_data_escaped_end_tag_open_state() {
        match token {
            Token::CharToken(c) => return_string.push(c),
            _ => assert!(false)
        }
    }
    assert_eq!(return_string, String::from("</"));

    match t.state {
        State::ScriptDataEscapedState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), '*')
}
