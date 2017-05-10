#[cfg(test)]
use super::*;

#[test]
// Encountering a whitespace character should:
// If the EndTagToken is an appropriate end tag token
//   Change state to State::BeforeAttrNameState
//   Return an empty token vector
// Otherwise
//   Treat as "anything else"
fn whitespace() {
    // Appropriate end tag token
    let mut t = Tokenizer::new(" ");
    t.tokens.push(Token::StartTagToken(Tag::new(String::from("div"))));
    t.tokens.push(Token::EndTagToken(Tag::new(String::from("div"))));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::from("div"))));

    let tokens = t.consume_script_data_end_tag_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::BeforeAttrNameState => assert!(true),
        _ => assert!(false)
    }

    // Otherwise
    let mut u = Tokenizer::new(" ");
    assert_any_other_character(&mut u, String::from("nam"));
}

#[test]
// Encountering a SOLIDUS '/' should:
// If the EndTagToken is an appropriate end tag token
//   Change state to State::SelfClosingStartTagState
//   Return an empty token vector
// Otherwise
//   Treat as "anything else"
fn solidus() {
    // Appropriate end tag token
    let mut t = Tokenizer::new("/");
    t.tokens.push(Token::StartTagToken(Tag::new(String::from("div"))));
    t.tokens.push(Token::EndTagToken(Tag::new(String::from("div"))));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::from("div"))));

    let tokens = t.consume_script_data_end_tag_name_state();
    assert_eq!(tokens.len(), 0);
    match t.state {
        State::SelfClosingStartTagState => assert!(true),
        _ => assert!(false)
    }

    // Otherwise
    let mut u = Tokenizer::new("/");
    assert_any_other_character(&mut u, String::from("nam"));
}

#[test]
// Encountering a '>' should:
// If the EndTagToken is an appropriate end tag token
//   Change state to State::DataState
//   Emit the current tag token
// Otherwise
//   Treat as "anything else"
fn greater_than() {
    // Appropriate end tag token
    let mut t = Tokenizer::new(">");
    t.tokens.push(Token::StartTagToken(Tag::new(String::from("div"))));
    t.tokens.push(Token::EndTagToken(Tag::new(String::from("div"))));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::from("div"))));

    match *t.consume_script_data_end_tag_name_state().first().unwrap() {
        Token::EndTagToken(_) =>  assert!(true),
        _ => assert!(false)
    }

    match t.state {
        State::DataState => assert!(true),
        _ => assert!(false)
    }

    // Otherwise
    let mut u = Tokenizer::new(">");
    assert_any_other_character(&mut u, String::from("nam"));
}

#[test]
// Encountering capital ascii char should:
//   Append the char to the current tag name as lower case
//   Append the character to the tmp_buffer
//   Return an empty token vector
fn uppercase_ascii() {
    let mut t = Tokenizer::new("A");
    t.tokens.push(Token::EndTagToken(Tag::new(String::new())));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::new())));
    let tokens = t.consume_script_data_end_tag_name_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.tmp_buffer, String::from("A"));
    match t.current_token() {
        Token::EndTagToken(tag) => {
            assert_eq!(tag.name, String::from("a"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering lower case ascii char should:
//   Append the char to the current tag name
//   Append the char to the tmp_buffer
//   Return an empty token vector
fn lowercase_ascii() {
    let mut t = Tokenizer::new("a");
    t.tokens.push(Token::EndTagToken(Tag::new(String::new())));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::new())));
    let tokens = t.consume_script_data_end_tag_name_state();
    assert_eq!(tokens.len(), 0);
    assert_eq!(t.tmp_buffer, String::from("a"));
    match t.current_token() {
        Token::EndTagToken(tag) => {
            assert_eq!(tag.name, String::from("a"));
        },
        _ => assert!(false)
    }
}

#[test]
// Encountering any other character should:
//   Push the following tokens:
//     - <
//     - /
//     - A charToken with every char in the tmp_buffer
//  Reconsume the current character
//  Change to the RCDataState
fn any_other_character() {
    let mut t = Tokenizer::new("*");
    assert_any_other_character(&mut t, String::from("nam"));
}


fn assert_any_other_character(t: &mut Tokenizer, buf: String) {
    let start_char = t.next_char();
    let chars = format!("</{}", buf);

    t.tmp_buffer = buf;
    t.tokens.push(Token::EndTagToken(Tag::new(String::new())));
    t.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

    // Assert tokens
    let mut returned_string = String::new();
    let tokens = t.consume_script_data_end_tag_name_state();
    for token in tokens {
        match token {
            Token::CharToken(c) => {
                returned_string.push(c);
            },
            _ => assert!(false),
        }
    }
    assert_eq!(returned_string, chars);

    // Reconsume and change state
    assert_eq!(t.next_char(), start_char);
    match t.state {
        State::ScriptDataState => assert!(true),
        _ => assert!(false)
    }
}

