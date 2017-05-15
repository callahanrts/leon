#[cfg(test)]
use super::*;

#[test]
// Encountering EOF should:
fn eof() {
    let mut t = Tokenizer::new("");
    let tokens = t.consume_doctype_state();
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
//   Change to the BeforeDOCTYPENameState
fn whitespace() {
    let mut t = Tokenizer::new(" ");
    let tokens = t.consume_doctype_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::BeforeDOCTYPENameState => assert!(true),
        _ => assert!(false)
    }
}

#[test]
// Encountering anything else should:
//   Parse Error
//   Reconsume in the BeforeDOCTYPENameState
fn anything_else() {
    let mut t = Tokenizer::new("s");
    let tokens = t.consume_doctype_state();
    assert_eq!(tokens.len(), 0);

    match t.state {
        State::BeforeDOCTYPENameState => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(t.next_char(), 's');
}
