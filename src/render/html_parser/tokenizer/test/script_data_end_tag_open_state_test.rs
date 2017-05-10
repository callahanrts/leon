#[cfg(test)]
use super::*;

#[test]
// Encountering an ascii char should:
//   Set the current token to an EndTagToken with an empty name string
//   Reconsume the char
//   Change state to ScriptDataEndTagNameState;
fn ascii() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_script_data_end_tag_open_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.next_char(), 'a');

    match t.state {
        State::ScriptDataEndTagNameState => assert!(true),
        _ => assert!(false),
    }
    match t.current_token.unwrap() {
        Token::EndTagToken(tag) => {
            assert_eq!(tag.name, String::new());
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should
//   Reconsume the character
//   Change tstate to ScriptDataState
//   Emit '<' and '/' CharTokens
fn anything_else() {
    let mut t = Tokenizer::new("*");
    let tokens = t.consume_script_data_end_tag_open_state();
    let mut return_string = String::new();
    for token in tokens {
        match token {
            Token::CharToken(c) => return_string.push(c),
            _ => {}
        }
    }
    assert_eq!(return_string, String::from("</"));
    assert_eq!(t.next_char(), '*');

    match t.state {
        State::ScriptDataState => assert!(true),
        _ => assert!(false),
    }
}

