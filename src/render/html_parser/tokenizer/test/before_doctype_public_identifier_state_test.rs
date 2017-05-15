#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_before_doctype_public_identifier_state();
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
//   Change to the BeforeDOCTYPEPublicIdentifierState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_before_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering '"' should:
//   Set the public_identifier of the current token to an empty string
//   Change to the DOCTYPEPublicIdentifierDoubleQuotedState
fn double_quote() {
    let mut t = Tokenizer::new("\"");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_before_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.public_identifier.unwrap(), String::new());
        },
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPEPublicIdentifierDoubleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '\'' should:
//   Set the public_identifier of the current token to an empty string
//   Change to the DOCTYPEPublicIdentifierSingleQuotedState
fn single_quote() {
    let mut t = Tokenizer::new("'");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_before_doctype_public_identifier_state();
    assert_eq!(tokens.len(), 0);

    match t.current_token() {
        Token::DoctypeToken(data) => {
            assert_eq!(data.public_identifier.unwrap(), String::new());
        },
        _ => assert!(false)
    }

    match t.state {
        State::DOCTYPEPublicIdentifierSingleQuotedState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering '>' should:
//   Set the force-quirks flag
//   Switch to the data state
//   Emit the doctype token
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));

    match *t.consume_before_doctype_public_identifier_state().first().unwrap() {
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
fn anything_else() {
    let mut t = Tokenizer::new("s");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_before_doctype_public_identifier_state();
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
