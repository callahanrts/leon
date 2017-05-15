#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    let tokens = t.consume_before_doctype_name_state();
    assert_eq!(tokens.len(), 2);
    for token in tokens {
        match token {
            Token::DoctypeToken(data) => {
                assert_eq!(data.force_quirks, true);
            },
            Token::EOFToken => assert!(true),
            _ => assert!(false)
        }
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering whitespace should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_before_doctype_name_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering uppercase ascii character should:
//   Create a new doctype token
//   Set the tokens name to the lowercase version of the input char
//   Switch to the DOCTYPENameState
fn uppercase_ascii() {
    let mut t = Tokenizer::new("A");
    let tokens = t.consume_before_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPENameState => assert!(true),
        _ => assert!(false)
    }

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("a"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering null character should:
//   Parse Error
//   Create a new DOCTYPEToken
//   Set the token's name to the replacement character
//   Switch to the doctype name state
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    let tokens = t.consume_before_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPENameState => assert!(true),
        _ => assert!(false)
    }

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("\u{FFFD}"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' character should:
//   Parse Error
//   Create a new DOCTYPEToken
//   Set its force-quirks falg to on
//   Switch to the DataState
//   Emit the token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    match *t.consume_before_doctype_name_state().first().unwrap() {
        Token::DoctypeToken(ref data) => {
            assert_eq!(data.force_quirks, true);
        },
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }

}

#[test]
// Encountering anything else should:
//   Create a new DOCTYPE Token
//   Set the tokens' name to the current character
//   Switch to the doctype name state
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let tokens = t.consume_before_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPENameState => assert!(true),
        _ => assert!(false)
    }

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("a"));
        },
        _ => assert!(false)
    }
}
