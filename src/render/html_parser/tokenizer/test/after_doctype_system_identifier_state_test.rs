#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_after_doctype_system_identifier_state();
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
// Encoutering whitespace should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_after_doctype_system_identifier_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering '>' should:
//   Change to the data state
//   Emit the doctype token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));

    match *t.consume_after_doctype_system_identifier_state().first().unwrap() {
        Token::DoctypeToken(_) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Change to the BogusDOCTYPEState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let data = DoctypeData::new(String::new());
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_after_doctype_system_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::BogusDOCTYPEState => assert!(true),
        _ => assert!(false)
    }
}

