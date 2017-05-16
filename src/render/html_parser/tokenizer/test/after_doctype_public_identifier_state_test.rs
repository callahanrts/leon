#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_after_doctype_public_identifier_state();
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
//   Change to the BetweenDOCTYPEPublicAndSystemIdentifiersState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_after_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::BetweenDOCTYPEPublicAndSystemIdentifiersState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Change to the data state
//   Emit the doctype token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));

    match *t.consume_after_doctype_public_identifier_state().first().unwrap() {
        Token::DoctypeToken(ref data) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encoutering '"' should
//   Set the system identifier to the empty string
//   Change to the DOCTYPESystemIdentifierDoubleQuotedState
fn double_quote() {
    let mut t = Tokenizer::new("\"");
    let mut data = DoctypeData::new(String::new());
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_after_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.system_identifier.unwrap(), String::new());
        }
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPESystemIdentifierDoubleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encoutering '\'' should
//   Set the system identifier to the empty string
//   Change to the DOCTYPESystemIdentifierDoubleQuotedState
fn single_quote() {
    let mut t = Tokenizer::new("'");
    let mut data = DoctypeData::new(String::new());
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_after_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.system_identifier.unwrap(), String::new());
        }
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPESystemIdentifierSingleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Set the force_quirks flag
//   Change to the BogusDOCTYPEState
fn anything_else() {
    let mut t = Tokenizer::new("a");
    let mut data = DoctypeData::new(String::new());
    t.current_token = Some(Token::DoctypeToken(data));
    let tokens = t.consume_after_doctype_public_identifier_state();
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
