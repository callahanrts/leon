#[cfg(test)]
use super::*;

// Parser
#[test]
fn test_new_parser() {
    let input = "div{ color: #fff; }";
    let mut parser = Parser::new(input);
    assert_eq!(parser.top_level, false);
    assert!(parser.tokens.len() > 0);
}

#[test]
fn test_newrule_data(){
    let rule = RuleData::new();
    assert_eq!(rule.name, String::new());
    assert_eq!(rule.prelude.len(), 0);
    match rule.block {
        Block::Empty => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_new_block_data(){
    let block = BlockData::new(Token::LeftCurlyBracketToken);
    assert_eq!(block.name, String::new());
    assert_eq!(block.value.len(), 0);
}

#[test]
fn test_new_declaration() {
    let dec = Declaration::new();
    assert_eq!(dec.name, String::new());
    assert_eq!(dec.value.len(), 0);
    assert_eq!(dec.important, false);
}

#[test]
fn test_parse_stylesheet() {
    let input = "@media screen { body{color: #fff;} } \ndiv { color: #fff; } \nspan { color: #000; }";
    let mut parser = Parser::new(input);
    let sheet = parser.parse_stylesheet();
    assert!(parser.top_level);
    assert_eq!(sheet.rules.len(), 3);
}

#[test]
fn test_parse_rules_list() {
    let input = "@media screen { body{color: #fff;} } \ndiv { color: #fff; } \nspan { color: #000; }";
    let mut parser = Parser::new(input);
    let rules = parser.parse_rules_list();
    assert_eq!(rules.len(), 3);
}

#[test]
fn test_parse_rule() {
    let input = "div { color: #fff; }";
    let mut parser = Parser::new(input);
    match parser.parse_rule() {
        Ok(rule) => {
            match rule {
                Rule::BasicRule(data) => {
                    // TODO: Test Block and other rule data
                    match data.prelude[0].clone() {
                        ComponentValue::Token(token) => {
                            match token {
                                Token::IdentToken(name) => assert_eq!(name, "div"),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false)
                    }
                }
                _ => assert!(false),
            }
        },
        Err(msg) => assert!(false)
    }
}

#[test]
fn test_parse_declaration() {
    let input = "color: #fff ! Important  ";
    let mut parser = Parser::new(input);
    match parser.parse_declaration() {
        Ok(dec) => {
            assert_eq!(dec.name, "color");
            // TODO:
            // assert!(dec.important);
        },
        Err(msg) => assert!(false)
    }
}

#[test]
fn test_parse_declaration_list() {
    let input = "color: #fff ! Important; border : 1px solid #000";
    let mut parser = Parser::new(input);
    let decs = parser.parse_declaration_list();
    assert_eq!(decs.len(), 2);
}

#[test]
fn test_parse_component_value() {
    let input = "{ color: #fff; }";
    let mut parser = Parser::new(input);
    match parser.parse_component_value() {
        Ok(cv) => {
            match cv {
                ComponentValue::Block(block) => assert_simple_block(block),
                _ => assert!(false)
            }
        },
        Err(msg) => assert!(false)
    }
}

#[test]
fn test_parse_component_value_list() {
    let input = "{ color: #fff; }";
    let mut parser = Parser::new(input);
    let cvs = parser.parse_component_value_list();
    assert_eq!(cvs.len(), 2);
}

// #[test]
// fn test_parse_csv_component_values() {
// }

#[test]
fn test_consume_rules() {
    let input = "@media screen { body{color: #fff;} } \ndiv { color: #fff; } \nspan { color: #000; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    let rules = consume_rules(&mut iterator, false);
    assert_eq!(rules.len(), 3);
}

#[test]
fn test_consume_qualified_rule() {
    let input = "div span {color: #fff;}";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    match consume_qualified_rule(&mut iterator) {
        Some(rule) => {
            match rule {
                Rule::BasicRule(data) => {
                    test_prelude(data.prelude);
                    test_block(data.block);
                }
                _ => assert!(false)
            }
        },
        None => assert!(false)
    }
}

fn test_prelude(prelude: Vec<ComponentValue>) {
    match prelude[0].clone() {
        ComponentValue::Token(token) => {
            match token {
                Token::IdentToken(name) => assert_eq!(name, "div".to_owned()),
                _ => assert!(false)
            }
        }
        _ => assert!(false)
    }
    match prelude[2].clone() {
        ComponentValue::Token(token) => {
            match token {
                Token::IdentToken(name) => assert_eq!(name, "span".to_owned()),
                _ => assert!(false)
            }
        }
        _ => assert!(false)
    }
}

fn test_block(block: Block) {
    match block {
        Block::SimpleBlock(data) => {
            match data.value[0].clone() {
                ComponentValue::Token(token) => {
                    match token {
                        Token::IdentToken(name) => assert_eq!(name, "color".to_owned()),
                        _ => assert!(false)
                    }
                },
                _ => assert!(false)
            }
        },
        _ => assert!(false)
    }
}

#[test]
fn test_consume_at_rule() {
    let input = "@media screen{}";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    match consume_at_rule(&mut iterator) {
        Some(rule) => {
            match rule {
                Rule::AtRule(data) => {
                    assert_eq!(data.name, "media");
                    match data.prelude[1].clone() {
                        ComponentValue::Token(token) => {
                            match token {
                                Token::IdentToken(name) => assert_eq!(name, "screen"),
                                _ => assert!(false)
                            }
                        },
                        _ => assert!(false)
                    }
                },
                _ => assert!(false)
            }
        },
        None => assert!(false)
    }
}

#[test]
fn test_consume_simple_block() {
    let input = "{ color: #fff; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    // Consume a token. The TokenIterator starts with the reconsume flag set. Since this function
    // is never called from a parse method, it should not start with the reconsume flag. For
    // testing, we'll just consume a token.
    iterator.consume_token();
    let block = consume_simple_block(&mut iterator);
    assert_simple_block(block);
}

#[test]
fn test_consume_component_value() {
    let input = " { color: #fff; }";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    iterator.consume_token();
    match consume_component_value(&mut iterator) {
        ComponentValue::Block(block) => assert_simple_block(block),
        _ => assert!(false)
    }
}

fn assert_simple_block(block: Block) {
    match block {
        Block::SimpleBlock(data) => {
            match data.value[0].clone() {
                ComponentValue::Token(token) => {
                    match token {
                        Token::WhitespaceToken => assert!(true),
                        _ => assert!(false),
                    }
                }
                _ => assert!(true)
            }

            match data.value[1].clone() {
                ComponentValue::Token(token) => {
                    match token {
                        Token::IdentToken(name) => assert_eq!(name, "color".to_owned()),
                        _ => assert!(false),
                    }
                }
                _ => assert!(true)
            }
        },
        _ => assert!(false)
    }
}

#[test]
fn test_consume_function() {
    let input = "('http://google.com')";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    match consume_function(&mut iterator, "url".to_owned()) {
        Block::FunctionBlock(data) => {
            assert!(data.value.len() > 0);
            match data.value[0].clone() {
                ComponentValue::Token(token) => {
                    match token {
                        Token::StringToken(url) => assert_eq!(url, "http://google.com".to_owned()),
                        _ => assert!(false),
                    }
                }
                _ => assert!(true)
            }
        },
        _ => assert!(false)
    }
}

#[test]
fn test_strip_important() {
    let mut values = Vec::new();
    values.push(ComponentValue::Token(Token::WhitespaceToken));
    values.push(ComponentValue::Token(Token::DelimToken('!')));
    values.push(ComponentValue::Token(Token::WhitespaceToken));
    let (vals, important) = strip_important(values.clone());
    assert!(!important);

    values.push(ComponentValue::Token(Token::IdentToken("Important".to_owned())));
    values.push(ComponentValue::Token(Token::WhitespaceToken));
    let (vals, important) = strip_important(values);
    assert!(important);
}

#[test]
fn test_consume_declaration() {
    let input = "color: #fff ! Important  ";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    match consume_declaration(&mut iterator) {
        Some(dec) => {
            assert_eq!(dec.name, "color");
            // TODO:
            // assert!(dec.important);
        },
        None => assert!(false)
    }

    let input = "color : #fff ! Important  ";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    match consume_declaration(&mut iterator) {
        Some(dec) => {
            assert_eq!(dec.name, "color");
            // TODO:
            // assert!(dec.important);
        },
        None => assert!(false)
    }
}

#[test]
fn test_consume_declarations() {
    let input = "color: #fff ! Important; border : 1px solid #000";
    let mut parser = Parser::new(input);
    let mut iterator = TokenIterator::new(parser.tokens.clone());
    let decs = consume_declarations(&mut iterator);
    assert_eq!(decs.len(), 2);
}

