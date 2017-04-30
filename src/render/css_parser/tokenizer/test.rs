
#[cfg(test)]
use super::*;

#[test]
// Look ahead to the next character without consuming it
fn test_next_char() {
    let mut s = Tokenizer::new("Hello");
    assert_eq!(s.next_char(), 'H');
    assert_eq!(s.next_char(), 'H');
}

#[test]
// Look ahead to the nth character
fn test_nth_char() {
    let mut s = Tokenizer::new("Hello");
    s.consume_char();
    assert_eq!(s.nth_char(4), 'o');
}

#[test]
fn starts_with() {
    let mut s = Tokenizer::new("Hello");
    assert!(s.starts_with("He"));
    assert!(!s.starts_with("lo"));
}

#[test]
fn test_eof() {
    let mut s = Tokenizer::new("H");
    assert_eq!(s.eof(), false);
    s.consume_char();
    assert_eq!(s.eof(), true);
}

//
// Consumption
//

#[test]
fn test_consume_char() {
    let mut s = Tokenizer::new("Hello");
    assert_eq!(s.consume_char(), 'H');
    assert_eq!(s.consume_char(), 'e');
    assert_eq!(s.consume_char(), 'l');
    assert_eq!(s.consume_char(), 'l');
    assert_eq!(s.consume_char(), 'o');
}

#[test]
fn test_consume_while() {
    let mut s = Tokenizer::new("Hello World");
    s.consume_while(|c| match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' => true,
        _ => false
    });
    s.consume_whitespace();
    assert_eq!(s.next_char(), 'W');
}

#[test]
fn test_consume_comment() {
    let mut s = Tokenizer::new("/* bunch of \n* comment \n* stuff */hello");
    s.consume_comment();
    assert!(s.starts_with("hello"));
}

#[test]
fn test_consume_whitespace() {
    // Spaces, tabs, newlines
    let mut s = Tokenizer::new(" \t	\r\nHello");
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
    let mut s = Tokenizer::new("\"This is a string\" hello");
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string"),
        _ => assert!(false),
    };
    assert!(s.starts_with(" hello"));
}

#[test]
fn test_single_quotes() {
    let mut s = Tokenizer::new("'This is a string' hello");
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string"),
        _ => assert!(false),
    };
    assert!(s.starts_with(" hello"));
}

#[test]
// Return a <bad-string-token> if there is a newline inside a string
fn test_bad_string() {
    let mut s = Tokenizer::new("'This is a string\n' hello");
    match s.consume_string_token() {
        Token::BadStringToken => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_unmatched_string() {
    let mut s = Tokenizer::new("'This is a string hello");
    match s.consume_string_token() {
        Token::StringToken(s) => assert!(s == "This is a string hello"),
        _ => assert!(false),
    };
}

// #[test]
// This doesn't work currently
// fn test_escaped_newline() {
//     let input = format!("Multi{} line{} string", '\n', '\n');
//     let mut s = Tokenizer::new(input);
//     match s.consume_string_token() {
//         Token::StringToken(s) => {
//             println!("STR: '{}'", s);
//             assert!(s == "Multi line string")
//         },
//         _ => assert!(false),
//     };
// }

// Tested through consume_string_token
// fn test_consume_string(){}

#[test]
fn test_consume_hash_token() {
    let mut s = Tokenizer::new("#test2-id_w div");
    match s.consume_hash_token() {
        Token::HashToken{hash_type, name} => {
            assert_eq!(hash_type, "id");
            assert_eq!(name, "test2-id_w");
        },
        _ => assert!(false),
    }
}

#[test]
fn test_consume_number() {
    assert_consume_number("23 ", 23.0, "integer");
    assert_consume_number("-23 ", -23.0, "integer");
    assert_consume_number("23.5 ", 23.5, "number");
    assert_consume_number("+23.5 ", 23.5, "number");
    // Not sure if this is even supported?
    //assert_consume_number(".023.5e+3 ", 23.5, "number");
}

fn assert_consume_number(input: &str, val: f32, tpe: &str) {
    let mut s = Tokenizer::new(input);
    let (v, t) = s.consume_number();
    assert_eq!(val, v);
    assert_eq!(t, String::from(tpe));
}

#[test]
fn test_consume_number_token() {
    let mut s = Tokenizer::new("+123;");
    match s.consume_number_token() {
        Token::NumberToken{value: v, num_type: n} => assert_eq!(v, 123.0),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("23%");
    match s.consume_number_token() {
        Token::PercentageToken(v) => assert_eq!(v, 23.0),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("23px");
    match s.consume_number_token() {
        Token::DimensionToken{value: v, num_type: n, unit: u} => {
            assert_eq!(v, 23.0);
            assert_eq!(u, "px");
        }
        _ => assert!(false),
    }
}

#[test]
fn test_consume_plus() {
    let mut s = Tokenizer::new("+123;");
    match s.consume_plus() {
        Token::NumberToken{value: v, num_type: n} => {
            assert_eq!(v, 123.0);
            assert_eq!(n, "integer");
        },
        _ => assert!(false),
    }
    assert_eq!(s.next_char(), ';');
}

#[test]
fn test_consume_minus() {
    // CDC Token
    let mut s = Tokenizer::new("--> ");
    match s.consume_minus() {
        Token::CDCToken => assert!(true),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("-23 ");
    match s.consume_minus() {
        Token::NumberToken{value: v, num_type: n} => assert_eq!(v, -23.0),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("-- ");
    match s.consume_minus() {
        Token::DelimToken(v) => assert_eq!(v, '-'),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("-test ");
    match s.consume_minus() {
        Token::IdentToken(v) => assert_eq!(v, "test"),
        _ => assert!(false),
    }
}

fn test_consume_ident() {
}

#[test]
fn test_consume_url_token() {
    let mut s = Tokenizer::new("  http://www.url.com/);");
    match s.consume_url_token() {
        Token::UrlToken(u) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(s.next_char(), ';');

    let mut s = Tokenizer::new("  http://www.url.(com/  ;");
    match s.consume_url_token() {
        Token::BadUrlToken => assert!(true),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("  http://www.url.com/'  ;");
    match s.consume_url_token() {
        Token::BadUrlToken => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_consume_bad_url_remnants() {
    let mut s = Tokenizer::new("  bad(url**\\''://\"ww.url.com/');");
    s.consume_bad_url_remnants();
    assert_eq!(s.next_char(), ';');
}

#[test]
fn test_consume_full_stop() {
    let mut s = Tokenizer::new(".23;");
    match s.consume_full_stop()  {
        Token::NumberToken{value: v, num_type: n} => assert_eq!(v, 0.23),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new(".d");
    match s.consume_full_stop() {
        Token::DelimToken(c) => assert_eq!(c, '.'),
        _ => assert!(false),
    }
}

#[test]
fn test_consume_less_than() {
    let mut s = Tokenizer::new("<!--  ;");
    match s.consume_less_than()  {
        Token::CDOToken => assert!(true),
        _ => assert!(false),
    }

    let mut s = Tokenizer::new("<fdsa");
    match s.consume_less_than() {
        Token::DelimToken(c) => assert_eq!(c, '<'),
        _ => assert!(false),
    }
}

fn test_consume_token() {
}

//
// Helpers
//

#[test]
// name-start code point:
//     A letter, a non-ASCII code point, or U+005F LOW LINE (_).
//     Digits are thrown in here because that has become an accepted char by modern browsers
fn test_name_start_code_point() {
    assert!(name_start_code_point('b'));
    assert!(name_start_code_point('B'));
    assert!(name_start_code_point('_'));
    assert!(!name_start_code_point('5'));
    assert!(!name_start_code_point('-'));
    assert!(!name_start_code_point('@'));
}

#[test]
// name code point
//     A name-start code point, a digit, or U+002D HYPHEN-MINUS (-).
fn test_name_code_point() {
    assert!(name_code_point('b'));
    assert!(name_code_point('B'));
    assert!(name_code_point('5'));
    assert!(name_code_point('-'));
    assert!(name_code_point('_'));
    assert!(!name_code_point('@'));
}

#[test]
// https://drafts.csswg.org/css-syntax/#check-if-three-code-points-would-start-an-identifier
fn test_would_be_identifier() {
    let mut s = Tokenizer::new("#abc-de_fg23 div");
    s.consume_char(); // Consume #
    assert!(would_be_identifier(&s));

    let mut s = Tokenizer::new("#2abc");
    s.consume_char(); // Consume #
    assert!(!would_be_identifier(&s));

    let mut s = Tokenizer::new("#_abc");
    assert!(!would_be_identifier(&s));

    let mut s = Tokenizer::new("#-abc");
    assert!(!would_be_identifier(&s));
}

#[test]
fn test_is_number(){
    assert!(is_number('0'));
    assert!(is_number('5'));
    assert!(!is_number('a'));
}

#[test]
// https://drafts.csswg.org/css-syntax/#starts-with-a-number
fn test_start_of_number() {
    let mut s = Tokenizer::new("0");
    assert!(start_of_number(&s));

    let mut s = Tokenizer::new(".01");
    assert!(start_of_number(&s));

    let mut s = Tokenizer::new("+1");
    assert!(start_of_number(&s));

    let mut s = Tokenizer::new("-1");
    assert!(start_of_number(&s));

    let mut s = Tokenizer::new("-1e+3");
    assert!(start_of_number(&s));

    let mut s = Tokenizer::new("1e-3");
    assert!(start_of_number(&s));
}

#[test]
fn test_newline() {
    assert!(newline('\n'));
    assert!(newline('\r'));
    assert!(!newline(' '));
}
