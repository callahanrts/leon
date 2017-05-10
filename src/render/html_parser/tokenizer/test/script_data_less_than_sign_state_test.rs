#[cfg(test)]
use super::*;

#[test]
// Encountering a solidus '/' should:
//   Set tmp buffer to the empty string
//   Change state to ScriptDataEndTagOpenState
fn solidus() {
    let mut t = Tokenizer::new("/");
    let tokens = t.consume_script_data_less_than_sign_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::ScriptDataEndTagOpenState => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Encountering an exclamation '!' should:
//   Change state to ScriptDataEscapeStartState
//   Return <, and ! CharTokens
fn exclamation() {
    let mut t = Tokenizer::new("!");
    let tokens = t.consume_script_data_less_than_sign_state();
    let mut return_string = String::new();
    for token in tokens {
        match token {
            Token::CharToken(c) => return_string.push(c),
            _ => assert!(false),
        }
    }
    assert_eq!(return_string, String::from("<!"));

    match t.state {
        State::ScriptDataEscapeStartState => assert!(true),
        _ => assert!(false),
    }
}

#[test]
// Encountering anything else should:
//   Reconsume the character
//   Emit a '<' CharToken
//   Change to ScriptDataState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_script_data_less_than_sign_state();
    match *tokens.first().unwrap() {
        Token::CharToken(c) => assert_eq!(c, '<'),
        _ => assert!(false)
    }

    match t.state {
        State::ScriptDataState => assert!(true),
        _ => assert!(false)
    }
}

