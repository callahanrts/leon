#[cfg(test)]
use super::*;
use super::super::*;

#[test]
fn test_consume_token() {
    let input = "div{ color: #fff; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    iterator.consume_token();
    match iterator.current_token() {
        Token::IdentToken(name) => assert_eq!(name, "div"),
        _ => assert!(false)
    }
}

#[test]
fn test_reconsume_token() {
    let input = "div{ color: #fff; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    let token_name = match iterator.current_token() {
        Token::IdentToken(name) => name,
        _ => String::new()
    };
    iterator.reconsume_token();
    iterator.consume_token();

    // The next token should not be consumed when reconsuming a token.
    match iterator.current_token() {
        Token::IdentToken(name) => assert_eq!(name, token_name),
        _ => assert!(false)
    }
}

#[test]
fn test_eot() {
    let input = "";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    iterator.consume_token();
    match iterator.current_token() {
        Token::EOFToken => assert!(true),
        _ => assert!(false)
    };
    assert!(iterator.eot());
}

#[test]
fn test_consume_while() {
    let input = "     {";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    let tokens = iterator.consume_while(|t| t == Token::WhitespaceToken);
    match iterator.current_token() {
        Token::LeftCurlyBracketToken => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn test_next_token() {
    let input = "div{ color: #fff; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    match iterator.next_token() {
        Token::LeftCurlyBracketToken => assert!(true),
        _ => assert!(false)
    }

    // EOF
    let input = "";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens);
    match iterator.next_token() {
        Token::EOFToken => assert!(true),
        _ => assert!(false)
    }
}


