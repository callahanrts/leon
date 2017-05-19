#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_doctype_name_state();
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
//   Change to the doctype name state
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPENameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Change to the datastate
//   Emit the current token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    match *t.consume_doctype_name_state().first().unwrap() {
        Token::DoctypeToken(_) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering uppercase ascii character should:
//   Append the lowercase version to the doctype token's name
fn uppercase_ascii() {
    let mut t = Tokenizer::new("C");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::from("ab"))));
    let tokens = t.consume_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("abc"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering null character should:
//   Append the replacement char to the token's name
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::from("ab"))));
    let tokens = t.consume_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("ab\u{FFFD}"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Append the character to the token's name
fn anything_else() {
    let mut t = Tokenizer::new("c");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::from("ab"))));
    let tokens = t.consume_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.name, String::from("abc"));
        },
        _ => assert!(false)
    }
}
