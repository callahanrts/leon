#[cfg(test)]
mod test;
mod token_iterator;

use tokenizer::{Tokenizer,Token};
use self::token_iterator::{TokenIterator};

pub struct StyleSheet{
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Declaration{
     pub name: String,
     pub value: Vec<ComponentValue>,
     pub important: bool,
     pub rule: Option<Rule>,
}

impl Declaration {
    fn new() -> Declaration {
        Declaration {
            name: String::new(),
            value: Vec::new(),
            important: false,
            rule: None,
        }
    }

    pub fn string_value(&mut self) -> &str {
        let ref component_value = self.value[0];
        match *component_value {
            ComponentValue::Token(ref token) => {
                match *token {
                    Token::IdentToken(ref name) => {
                        name
                    },
                    _ => ""
                }
            },
            _ => ""
        }
    }

    pub fn number_value(&mut self) -> Option<Token> {
        let ref component_value = self.value[0];
        match *component_value {
            ComponentValue::Token(ref token) => {
                Some(token.clone())
            },
            _ => None
        }
    }
}

#[derive(Clone)]
pub enum Rule{
    BasicRule(RuleData),
    AtRule(RuleData),
}

#[derive(Clone)]
pub struct RuleData {
    pub name: String,
    pub prelude: Vec<ComponentValue>,
    pub block: Block
}

impl RuleData {
    pub fn new() -> RuleData {
        RuleData {
            name: String::new(),
            prelude: Vec::new(),
            block: Block::Empty,
        }
    }
}

#[derive(Clone)]
pub enum Block {
    Empty,
    SimpleBlock(BlockData),
    FunctionBlock(BlockData),
}

#[derive(Clone)]
pub struct BlockData {
    pub name: String, // NOTE: Name only indicates function name
    pub value: Vec<ComponentValue>, // Can be parsed into declarations
}

#[derive(Clone)]
pub enum ComponentValue {
    Block(Block),
    Token(Token),
}

impl BlockData {
    pub fn new(token: Token) -> BlockData {
        BlockData {
            name: String::new(),
            value: Vec::new(),
        }
    }
}

pub struct Parser {
    top_level: bool,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(css: &str) -> Parser {
        let mut t = Tokenizer::new(css);
        let tokens = t.consume_tokens();

        Parser {
            top_level: false,
            tokens: tokens,
        }
    }

    //
    // Parsing
    //
    // https://drafts.csswg.org/css-syntax/#parse-stylesheet
    pub fn parse_stylesheet(&mut self) -> StyleSheet {
        // Create a new stylesheet.
        let mut sheet = StyleSheet{rules: Vec::new()};

        self.top_level = true; // Set the 'top-level' flag
        let mut iterator = TokenIterator::new(self.tokens.clone());

        // Assign rules to the stylesheet's value.
        // Consume a list of rules from the stream of tokens, with the top-level flag set.
        // Let the return value be rules.
        sheet.rules = consume_rules(&mut iterator, self.top_level);

        // If the first rule in rules is an at-rule with a name that is an ASCII
        // case-insensitive match for "charset", remove it from rules.
        // TODO:

        return sheet; // Return the stylesheet.
    }

    pub fn parse_rules_list(&mut self) -> Vec<Rule> {
        let mut iterator = TokenIterator::new(self.tokens.clone());
        return consume_rules(&mut iterator, false);
    }

    pub fn parse_rule(&mut self) -> Result<Rule, &str> {
        let mut rule_data = RuleData::new();
        let mut rule = Rule::BasicRule(rule_data);
        let mut iterator = TokenIterator::new(self.tokens.clone());

        // While the next input token is a <whitespace-token>, consume the next input token.
        iterator.consume_while(|c| is_whitespace(c));

        match iterator.current_token() {
            // If the next input token is an <EOF-token>, return a syntax error.
            Token::EOFToken => return Err("syntax"),

            // Otherwise, if the next input token is an <at-keyword-token>
            Token::AtKeywordToken(name) => {
                // consume an at-rule, and let rule be the return value.
                if let Some(r) = consume_at_rule(&mut iterator) {
                    rule = r;
                }
            }

            // Otherwise,
            _ => {
                // This is necessary because of the way the token iterator is set up to begin
                // with the reconsume flag set.
                iterator.reconsume_token();

                // consume a qualified rule and let rule be the return value.
                if let Some(r) = consume_qualified_rule(&mut iterator) {
                    rule = r;
                } else {
                    // If nothing was returned, return a syntax error.
                    return Err("syntax");
                }
            },

        }

        // While the next input token is a <whitespace-token>, consume the next input token.
        iterator.consume_while(|c| is_whitespace(c));

        // If the next input token is an <EOF-token>, return rule. Otherwise, return a syntax error.
        match iterator.next_token() {
            Token::EOFToken => Ok(rule),
            _ => return Err("syntax")
        }
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, &str>{
        let mut iterator = TokenIterator::new(self.tokens.clone());

        // While the next input token is a <whitespace-token>, consume the next input token.
        iterator.consume_while(|c| is_whitespace(c));

        match iterator.current_token() {
            Token::IdentToken(name) => {
                match consume_declaration(&mut iterator) {
                    // Consume a declaration. If anything was returned, return it.
                    Some(dec) => return Ok(dec),

                    // Otherwise, return a syntax error.
                    None => return Err("syntax2"),
                }
            },
            // If the next input token is not an <ident-token>, return a syntax error.
            _ => return Err("syntax"),
        }
    }

    pub fn parse_declaration_list(&mut self) -> Vec<Declaration> {
        let mut iterator = TokenIterator::new(self.tokens.clone());
        return consume_declarations(&mut iterator);
    }

    pub fn parse_component_value(&mut self) -> Result<ComponentValue, &str> {
        let mut iterator = TokenIterator::new(self.tokens.clone());

        // While the next input token is a <whitespace-token>, consume the next input token.
        iterator.consume_while(|c| is_whitespace(c));

        // If the next input token is an <EOF-token>, return a syntax error.
        if iterator.current_token() == Token::EOFToken {
            return Err("syntax");
        }

        // Consume a component value and let value be the return value.
        let value = consume_component_value(&mut iterator);

        // While the next input token is a <whitespace-token>, consume the next input token.
        iterator.consume_while(|c| is_whitespace(c));

        match iterator.next_token() {
            // If the next input token is an <EOF-token>, return value.
            Token::EOFToken => return Ok(value),

            // Otherwise, return a syntax error.
            _ => return Err("syntax")
        }
    }

    pub fn parse_component_value_list(&mut self) -> Vec<ComponentValue> {
        let mut iterator = TokenIterator::new(self.tokens.clone());

        let mut values = Vec::new();
        // Repeatedly consume a component value until an <EOF-token> is returned,
        while !iterator.eot() {
            // appending the returned values (except the final <EOF-token>) into a list.
            values.push(consume_component_value(&mut iterator));
        }

        // Return the list.
        return values;
    }

    // // https://drafts.csswg.org/css-syntax/#parse-comma-separated-list-of-component-values
    // pub fn parse_csv_component_values(&mut self) {
    // }
}


//
// Consumption
//

// https://drafts.csswg.org/css-syntax/#consume-a-list-of-rules
fn consume_rules(iterator: &mut TokenIterator, top_level: bool) -> Vec<Rule> {
    // Create an initially empty list of rules.
    let mut rules = Vec::new();

    // Repeatedly consume the next input token:
    while !iterator.eot() {
        iterator.consume_token();

        match iterator.current_token() {
            Token::WhitespaceToken => {}, // Do nothing.
            Token::EOFToken => return rules, // Return the list of rules.
            Token::CDOToken | Token::CDCToken => {
                // If the top-level flag is set, do nothing.
                // Otherwise,
                if !top_level {
                    // reconsume the current input token.
                    iterator.reconsume_token();

                    // Consume a qualified rule.
                    if let Some(rule) = consume_qualified_rule(iterator) {
                        // If anything is returned, append it to the list of rules.
                        rules.push(rule);
                    }
                }
            },
            Token::AtKeywordToken(name) => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume an at-rule.
                if let Some(rule) = consume_at_rule(iterator) {
                    // If anything is returned, append it to the list of rules.
                    rules.push(rule);
                }
            }
            _ => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume a qualified rule.
                if let Some(rule) = consume_qualified_rule(iterator) {
                    // If anything is returned, append it to the list of rules.
                    rules.push(rule);
                }
            }
        }
    }
    return rules;
}

// https://drafts.csswg.org/css-syntax/#consume-a-qualified-rule
fn consume_qualified_rule(iterator: &mut TokenIterator) -> Option<Rule> {
    // Create a new qualified rule with its prelude initially set to an empty list,
    // and its value initially set to nothing.
    let mut rule_data = RuleData::new();

    // Repeatedly consume the next input token:
    while !iterator.eot() {
        iterator.consume_token();

        match iterator.current_token() {
            // Don't do anything (parse error) if we've reached the EOF
            Token::EOFToken => return None,
            Token::LeftCurlyBracketToken => {
                // Consume a simple block and assign it to the qualified rule's block.
                rule_data.block = consume_simple_block(iterator);

                // Return the qualified rule.
                return Some(Rule::BasicRule(rule_data));
            }
            // Simple Block? Not sure this part is necessary given above
            _ => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume a component value. Append the returned value to the
                // qualified rule's prelude.
                rule_data.prelude.push(consume_component_value(iterator));
            }
        }
    }

    return Some(Rule::BasicRule(rule_data));
}

fn consume_at_rule(iterator: &mut TokenIterator) -> Option<Rule> {
    // Consume the next input token. // NOTE: Does this just mean to start at the beginning? As
    // if current_token is nil?
    iterator.consume_token();

    // Create a new at-rule with its name set to the value of the current input token,
    // its prelude initially set to an empty list,
    // and its value initially set to nothing.
    let mut rule_data = RuleData::new();
    match iterator.current_token() {
        Token::AtKeywordToken(name) => rule_data.name = name,
        _ => {}
    }

    // Repeatedly consume the next input token:
    while !iterator.eot() {
        iterator.consume_token();
        match iterator.current_token() {
            Token::SemiColonToken |
            Token::EOFToken => return Some(Rule::AtRule(rule_data)),// Return the at-rule.
            Token::LeftCurlyBracketToken => {
                // Consume a simple block and assign it to the at-rule's block.
                rule_data.block = consume_simple_block(iterator);

                // Return the at-rule.
                return Some(Rule::AtRule(rule_data));
            },
            _ => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume a component value.
                // Append the returned value to the at-rule's prelude.
                rule_data.prelude.push(consume_component_value(iterator));
            }
        }
    }

    return Some(Rule::AtRule(rule_data));
}

// https://drafts.csswg.org/css-syntax/#consume-a-simple-block
// NOTE: This algorithm assumes that the current input token has already been consumed and
//       checked to be an <{-token>, <[-token>, or <(-token>.
fn consume_simple_block(iterator: &mut TokenIterator) -> Block {
    // The ending token is the mirror variant of the current input token.
    let end_token = match iterator.current_token() {
        Token::LeftCurlyBracketToken => Token::RightCurlyBracketToken,
        Token::LeftSquareBracketToken => Token::RightSquareBracketToken,
        Token::LeftParenToken => Token::RightParenToken,
        _ => panic!("Missing beginning block token!")
    };

    // Create a simple block with its associated token set to the current input token and
    // with a value with is initially an empty list.
    let mut block = BlockData::new(iterator.current_token());

    // Repeatedly consume the next input token
    while !iterator.eot() {
        iterator.consume_token();
        match iterator.current_token() {
            Token::EOFToken => break, // Return the block.
            ref t if *t == end_token =>  break, // ending token -- Return the block.
            _ => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume a component value and append it to the value of the block.
                block.value.push(consume_component_value(iterator));
            }
        }
    }

    return Block::SimpleBlock(block);
}

// https://drafts.csswg.org/css-syntax/#consume-a-component-value
fn consume_component_value(iterator: &mut TokenIterator) -> ComponentValue {
    // Consume the next input token.
    iterator.consume_token();
    match iterator.current_token() {
        // If the current input token is a <{-token>, <[-token>, or <(-token>,
        // consume a simple block and return it.
        Token::LeftCurlyBracketToken |
        Token::LeftSquareBracketToken |
        Token::LeftParenToken => ComponentValue::Block(consume_simple_block(iterator)),

        // Otherwise, if the current input token is a <function-token>,
        // consume a function and return it.
        Token::FunctionToken(name) => ComponentValue::Block(consume_function(iterator, name)),

        // Otherwise, return the current input token.
        _ => ComponentValue::Token(iterator.current_token()),
    }
}

// https://drafts.csswg.org/css-syntax/#consume-a-function
// NOTE: This algorithm assumes that the current input token has already been checked to be a
// <function-token>.
fn consume_function(iterator: &mut TokenIterator, name: String) -> Block {
    // Create a function with a name equal to the value of the current input token,
    // and with a value which is initially an empty list.
    let mut block_data = BlockData::new(Token::LeftParenToken);
    block_data.name = name;

    // Repeatedly consume the next input token and process it
    while !iterator.eot() {
        iterator.consume_token();
        match iterator.current_token() {
            Token::RightParenToken | Token::EOFToken => break, // Return the function.
            _ => {
                // Reconsume the current input token.
                iterator.reconsume_token();

                // Consume a component value and append the returned value to the function's value.
                block_data.value.push(consume_component_value(iterator));
            }
        }
    }

    // Return the block
    Block::FunctionBlock(block_data)
}

// NOTE: This algorithm assumes that the next input token has already been checked to be an <ident-token>.
fn consume_declaration(iterator: &mut TokenIterator) -> Option<Declaration> {
    // Consume the next input token.
    iterator.consume_token();

    // Create a new declaration with its name set to the value of the current input token and
    // its value initially set to the empty list.
    let mut dec = Declaration::new();
    match iterator.current_token() {
        Token::IdentToken(name) => {
            dec.name = name;
        },
        _ => panic!("This should be an <ident-token>")
    };

    iterator.consume_token();

    // While the next input token is a <whitespace-token>, consume the next input token.
    iterator.consume_while(|c| is_whitespace(c));

    // If the next input token is anything other than a <colon-token>, this is a parse error.
    // Return nothing. Otherwise, consume the next input token.
    match iterator.current_token() {
        Token::ColonToken => iterator.consume_token(),
        _ => return None
    }

    while !iterator.eot() {
        dec.value.push(consume_component_value(iterator));
    }

    let (values, important) = strip_important(dec.value.clone());
    dec.value = values;
    dec.important = important;

    Some(dec)
}

fn consume_declarations(iterator: &mut TokenIterator) -> Vec<Declaration> {
    // Create an initially empty list of declarations.
    let mut decs: Vec<Declaration> = Vec::new();

    // Repeatedly consume the next input token:
    while !iterator.eot() {
        iterator.consume_token();
        match iterator.current_token() {
            Token::WhitespaceToken |
            Token::SemiColonToken => {}, // Do nothing.
            Token::EOFToken => { return decs }, // Return the list of declarations.
            Token::AtKeywordToken(_) => {
                // Reconsume the current input token.
                iterator.reconsume_token();
                let mut dec = Declaration::new();
                dec.rule = consume_at_rule(iterator); // Consume an at-rule.
                decs.push(dec); // Append the returned rule to the list of declarations.
            },
            Token::IdentToken(name) => {
                // Initialize a temporary list initially filled with the current input token.
                let mut values = Vec::new();
                values.push(iterator.current_token());
                while !iterator.eot() {
                    iterator.consume_token();
                    let mut end = false;
                    match iterator.current_token() {
                        // As long as the next input token is anything other than a
                        // <semicolon-token> or <EOF-token>,
                        Token::SemiColonToken |
                        Token::EOFToken => break,
                        _ => {
                            // consume a component value and append it to the temporary list.
                            iterator.reconsume_token();
                            match consume_component_value(iterator) {
                                ComponentValue::Token(token) => {
                                    values.push(token);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // Consume a declaration from the temporary list.
                let mut itr = TokenIterator::new(values);
                if let Some(dec) = consume_declaration(&mut itr) {
                    // If anything was returned, append it to the list of declarations.
                    decs.push(dec);
                }
            },
            _ => {
                // This is a parse error.  Reconsume the current input token.
                iterator.reconsume_token();
                // As long as the next input token is anything other than a <semicolon-token>
                // or <EOF-token>
                while !iterator.eot() {
                    iterator.consume_token();
                    match iterator.current_token() {
                        Token::SemiColonToken |
                        Token::EOFToken => break,
                        _ => {
                            // consume a component value and throw away the returned value.
                            consume_component_value(iterator);
                        }
                    }
                }
            }
        }
    }
    return decs;
}


fn is_whitespace(token: Token) -> bool {
    match token {
        Token::WhitespaceToken => true,
        _ => false
    }
}

fn is_colon(token: Token) -> bool {
    match token {
        Token::ColonToken => true,
        _ => false
    }
}

fn component_value_token(value: &ComponentValue) -> Token {
    match *value {
        ComponentValue::Token(ref token) => token.clone(),
        ComponentValue::Block(ref block) => panic!("Not sure what to do with this yet.")
    }
}

pub fn parse_block_declarations(block: Block) -> Vec<Declaration> {
    match block {
        Block::SimpleBlock(data) => {
            let tokens: Vec<Token> = data.value.iter().map(|v| component_value_token(v)).collect();
            let mut iterator = TokenIterator::new(tokens.clone());
            consume_declarations(&mut iterator)
        },
        _ => Vec::new()
    }
}

// pub fn parse_declaration_data(dec: Declaration) -> (String, Vec<Token>) {
//     let tokens: Vec<Token> = dec.value.iter().map(|v| component_value_token(v)).collect();
//     let mut iterator = TokenIterator::new(tokens.clone());
//     let name = parse_declaration_name(&mut iterator);
//     let val = iterator.remaining_tokens();
//     return (name, val);
// }

// fn parse_declaration_name(iterator: &mut TokenIterator) -> String {
//     iterator.consume_token();
//     match iterator.current_token() {
//         Token::IdentToken(name) => {
//             iterator.consume_while(|c| is_colon(c));
//             name
//         }
//         _ => String::new() // There was a problem if we hit this
//     }
// }

// Starting very basic. We only care to know the id, classlist, and element type.
// Eventually, this will return a Selector object that will determine whether
// or not all, or any of the type#id.class, type .class, etc match.
pub fn parse_prelude(values: Vec<ComponentValue>) -> (String, String, Vec<String>) {
    let tokens: Vec<Token> = values.iter().map(|v| component_value_token(v)).collect();
    let mut iterator = TokenIterator::new(tokens.clone());
    let mut id = String::new();
    let mut element_name = String::new();
    let mut classes = Vec::new();
    while !iterator.eot() {
        iterator.consume_token();
        match iterator.current_token() {
            Token::EOFToken => break,
            Token::DelimToken(c) => {
                match c {
                    '.' => {
                        iterator.consume_token();
                        match iterator.current_token() {
                            Token::IdentToken(name) => {
                                classes.push(name);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Token::HashToken{hash_type: tpe, name: name } => {
                if tpe == "id" {
                    id = name;
                }
            },
            Token::IdentToken(name) => element_name = name,
            _ => {}
        }
    }

    return (element_name, id, classes);
}

// TODO: Refactor this super ugly function
fn strip_important(values: Vec<ComponentValue>) -> (Vec<ComponentValue>, bool) {
    let mut new_values = values.clone();
    new_values.reverse();
    let mut important = false;
    let mut idx = 0;
    for value in new_values.clone() {
        match value {
            ComponentValue::Token(token) => {
                match token {
                    Token::WhitespaceToken => {}
                    Token::DelimToken(c) => {
                        if c == '!' && important {
                            new_values.remove(idx);
                            new_values.reverse(); // Correct reversing
                            return (new_values, true);
                        } else {
                            return (values, important);
                        }
                    }
                    Token::IdentToken(name) => {
                        if name.to_lowercase() == "important" {
                            important = true;
                            new_values.remove(idx);
                        }
                    }
                    _ => {
                        return (values, important)
                    }
                }
            },
            _ => {
                return (values, important)
            }
        }
        idx += 1
    }
    return (values, important);
}

