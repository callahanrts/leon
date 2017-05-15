#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_doctype_public_identifier_double_quoted_state();
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
// Encountering '"' should:
//   Change to the AfterDOCTYPEPublicIdentifierState
fn double_quote() {
    let mut t = Tokenizer::new("\"");
    let tokens = t.consume_doctype_public_identifier_double_quoted_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::AfterDOCTYPEPublicIdentifierState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a null character should:
//   Append a replacement character
fn nullchar() {
    let mut t = Tokenizer::new("\u{0000}");
    let mut data = DoctypeData::new(String::new());
    data.public_identifier = Some(String::from("s"));
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_doctype_public_identifier_double_quoted_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.public_identifier.unwrap(), String::from("s\u{FFFD}"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Set the force_quirks flag
//   Change to the data state
//   Emit the current token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));

    match *t.consume_doctype_public_identifier_double_quoted_state().first().unwrap() {
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
//   Append a character to the current token
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let mut data = DoctypeData::new(String::new());
    data.public_identifier = Some(String::from("s"));
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_doctype_public_identifier_double_quoted_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.public_identifier.unwrap(), String::from("sa"));
        },
        _ => assert!(false)
    }
}
