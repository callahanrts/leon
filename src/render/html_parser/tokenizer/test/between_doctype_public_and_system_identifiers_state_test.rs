#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_between_doctype_public_and_system_identifiers_state();
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
// Encountering a whitespace characters should:
//   Ignore the character
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_after_attr_name_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering a '>' character should:
//   Switch to the data state
//   Emit the current tag token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::StartTagToken(Tag::new(String::new())));

    match *t.consume_after_attr_name_state().first().unwrap() {
        Token::StartTagToken(_) => assert!(true),
        _ => assert!(false)
    };

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering a '"' should:
//   Set the system_identifier to the empty string
//   Change to the DOCTYPESystemIdentifierDoubleQuotedState
fn double_quote() {
    let mut t = Tokenizer::new("\"");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_between_doctype_public_and_system_identifiers_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.system_identifier.unwrap(), String::new());
        },
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPESystemIdentifierDoubleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '\'' should:
//   Set the system_identifier to the empty string
//   Change to the DOCTYPESystemIdentifierSingleQuotedState
fn single_quote() {
    let mut t = Tokenizer::new("'");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_between_doctype_public_and_system_identifiers_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.system_identifier.unwrap(), String::new());
        },
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPESystemIdentifierSingleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Append the character to the attribute name
fn anything_else() {
    let mut t = Tokenizer::new("a");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_between_doctype_public_and_system_identifiers_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.force_quirks, true);
        },
        _ => assert!(false)
    }

    match t.state {
        State::BogusDOCTYPEState => assert!(true),
        _ => assert!(false)
    }
}

