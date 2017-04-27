
#[cfg(test)]
use super::*;

#[test]
// Look ahead to the next character
fn test_next_char() {
    let mut s = scanner("Hello".to_owned());
    assert_eq!(s.next_char(), 'H');
}

#[test]
fn test_nth_char() {
    let mut s = scanner("Hello".to_owned());
    s.consume_char();
    assert_eq!(s.nth_char(4), 'o');
}

#[test]
fn starts_with() {
    let mut s = scanner("Hello".to_owned());
    assert!(s.starts_with("He"));
    assert!(!s.starts_with("lo"));
}

#[test]
fn test_eof() {
    let mut s = scanner("H".to_owned());
    assert_eq!(s.eof(), false);
    s.consume_char();
    assert_eq!(s.eof(), true);
}

#[test]
fn test_consume_char() {
    let mut s = scanner("Hello".to_owned());
    assert_eq!(s.consume_char(), 'H');
    assert_eq!(s.consume_char(), 'e');
    assert_eq!(s.consume_char(), 'l');
    assert_eq!(s.consume_char(), 'l');
    assert_eq!(s.consume_char(), 'o');
}

#[test]
fn test_consume_while() {
    let mut s = scanner("Hello World".to_owned());
    s.consume_while(|c| match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' => true,
        _ => false
    });
    s.consume_whitespace();
    assert_eq!(s.next_char(), 'W');
}

#[test]
fn test_consume_token() {
}

#[test]
fn test_consume_comment() {
    let input = String::from("/* bunch of \n* comment \n* stuff */hello");
    let mut s = scanner(input);
    s.consume_comment();
    assert!(s.starts_with("hello"));
}

#[test]
fn test_consume_whitespace() {
    // Spaces, tabs, newlines
    let mut s = scanner(" \t	\r\nHello".to_owned());
    s.consume_whitespace();
    assert_eq!(s.next_char(), 'H');
}

// #[test]
// fn test_consume_string_token() {
//     test_double_quotes();
//     test_single_quotes();
//     test_bad_string();
//     test_unmatched_string();
//     test_escaped_newline();
// }

#[test]
fn test_double_quotes() {
    let input = String::from("\"This is a string\" hello");
    let mut s = scanner(input);
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string"),
        _ => assert!(false),
    };
    assert!(s.starts_with(" hello"));
}

#[test]
fn test_single_quotes() {
    let input = String::from("'This is a string' hello");
    let mut s = scanner(input);
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string"),
        _ => assert!(false),
    };
    assert!(s.starts_with(" hello"));
}

#[test]
// Return a <bad-string-token> if there is a newline inside a string
fn test_bad_string() {
    let input = String::from("'This is a string\n' hello");
    let mut s = scanner(input);
    match s.consume_string_token() {
        Token::BadStringToken => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_unmatched_string() {
    let input = String::from("'This is a string hello");
    let mut s = scanner(input);
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string hello"),
        _ => assert!(false),
    };
}

// #[test]
// This doesn't work currently
// fn test_escaped_newline() {
//     let input = format!("Multi{} line{} string", '\n', '\n');
//     let mut s = scanner(input);
//     match s.consume_string_token() {
//         Token::StringToken(s) => {
//             println!("STR: '{}'", s);
//             assert!(s == "Multi line string")
//         },
//         _ => assert!(false),
//     };
// }

fn text_consume_string() {
}

#[test]
fn test_consume_hash_token() {
    let input = String::from("#test2-id_w div");
    let mut s = scanner(input);
    match s.consume_hash_token() {
        Token::HashToken{hash_type, name} => {
            assert_eq!(hash_type, "id");
            assert_eq!(name, "test2-id_w");
        },
        _ => assert!(false),
    }
}

fn test_consume_plus() {
}

fn test_consume_minus() {
}

fn test_consume_number_token() {
}

fn test_consume_number() {
}

fn test_consume_cdc() {
}

fn test_consume_ident() {
}

fn test_consume_url_token() {
}

fn test_consume_bad_url_remnants() {
}

fn test_consume_full_stop() {
}

fn test_consume_less_than() {
}

fn test_consume_cdo_token() {
}

fn test_consume_token() {
}

//
// Helpers
//
#[test]
fn test_name_start_code_point() {
    assert!(name_start_code_point('b'));
    assert!(name_start_code_point('B'));
    assert!(name_start_code_point('_'));
    assert!(!name_start_code_point('5'));
    assert!(!name_start_code_point('-'));
    assert!(!name_start_code_point('@'));
}

#[test]
fn test_name_code_point() {
    assert!(name_code_point('b'));
    assert!(name_code_point('B'));
    assert!(name_code_point('5'));
    assert!(name_code_point('-'));
    assert!(name_code_point('_'));
    assert!(!name_code_point('@'));
}

#[test]
fn test_would_be_identifier() {
    let input = String::from("#abc-de_fg23 div");
    let mut s = scanner(input);
    s.consume_char(); // Consume #
    assert!(would_be_identifier(&s));

    let input = String::from("#2abc");
    let mut s = scanner(input);
    s.consume_char(); // Consume #
    assert!(!would_be_identifier(&s));

    let input = String::from("#_abc");
    let mut s = scanner(input);
    assert!(!would_be_identifier(&s));

    let input = String::from("#-abc");
    let mut s = scanner(input);
    assert!(!would_be_identifier(&s));
}

#[test]
fn test_is_number(){
    assert!(is_number('0'));
    assert!(is_number('5'));
    assert!(!is_number('a'));
}

fn test_would_be_identifier() {
}

fn scanner(input: String) -> Scanner {
    Scanner{
        pos: 0,
        input: input,
    }
}

