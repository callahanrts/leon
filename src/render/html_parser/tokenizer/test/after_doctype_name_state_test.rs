#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    let tokens = t.consume_after_doctype_name_state();
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
    let tokens = t.consume_after_doctype_name_state();
    assert_eq!(tokens.len(), 0);
}

#[test]
// Encountering '>' should:
//   Switch to the data state
//   Emit the current DOCTYPEToken
fn greater_than() {
    let mut t = Tokenizer::new(">");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::new())));
    match *t.consume_after_doctype_name_state().first().unwrap() {
        Token::DoctypeToken(_) => assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else (public) should:
//   Consume PUBLIC chars
//   Change state to DOCTYPEPublicKeywordState
fn anything_else_public() {
    let mut t = Tokenizer::new("PUBLICs");
    let tokens = t.consume_after_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPEPublicKeywordState => assert!(true),
        _ => assert!(false)
    }
    assert_eq!(t.next_char(), 's');
}

#[test]
// Encountering anything else (system) should:
//   Consume SYSTEM chars
//   Change state to DOCTYPESystemKeywordState
fn anything_else_private() {
    let mut t = Tokenizer::new("SYSTEMs");
    let tokens = t.consume_after_doctype_name_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::DOCTYPESystemKeywordState => assert!(true),
        _ => assert!(false)
    }
    assert_eq!(t.next_char(), 's');
}

#[test]
// Encountering anything else (other) should:
//   Set the force_quirks flag
//   Change tot he BogusDOCTYPEState
fn anything_else_other() {
    let mut t = Tokenizer::new("sa");
    t.current_token = Some(Token::DoctypeToken(DoctypeData::new(String::from("ab"))));
    let tokens = t.consume_after_doctype_name_state();
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
    assert_eq!(t.next_char(), 'a');
}
