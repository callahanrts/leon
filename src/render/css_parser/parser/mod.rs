use render::css_parser::tokenizer::{Tokenizer,Token};

pub struct StyleSheet{
    rules: Vec<Rule>,
}

pub struct Declaration{
}

pub enum Rule{
    Empty,
    BasicRule(RuleData),
    AtRule(RuleData),
}

pub struct RuleData {
    name: String,
    prelude: Vec<ComponentValue>,
    value: String,
    block: Block
}

impl RuleData {
    pub fn new() -> RuleData {
        RuleData {
            name: String::new(),
            prelude: Vec::new(),
            value: String::new(),
            block: Block::Empty,
        }
    }
}

enum Block {
    Empty,
    SimpleBlock(BlockData),
    FunctionBlock(BlockData),
}

struct BlockData {
    name: String,
    value: Vec<ComponentValue>,
    token: Token,
}

enum ComponentValue {
    Block(Block),
    Token(Token),
}

impl BlockData {
    pub fn new(token: Token) -> BlockData {
        BlockData {
            name: String::new(),
            value: Vec::new(),
            token: token,
        }
    }
}

struct Parser {
    pos: usize,
    top_level: bool,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(css: &str) -> Parser {
        let mut t = Tokenizer::new(css);
        Parser {
            pos: 0,
            top_level: false,
            tokens: t.consume_tokens(),
        }
    }

    //
    // Parsing
    //
    // https://drafts.csswg.org/css-syntax/#parse-stylesheet
    pub fn parse_stylesheet(&mut self) -> StyleSheet {
        let mut sheet = StyleSheet{rules: Vec::new()};

        // Consume with 'top-level' flag set
        self.top_level = true; // Set the 'top-level' flag
        sheet.rules = self.consume_rules();

        // If the first rule in rules is an at-rule with a name that is an ASCII
        // case-insensitive match for "charset", remove it from rules.
        return sheet;
    }

    pub fn parse_rules_list(&mut self) -> Vec<Rule> {
        return self.consume_rules();
    }

    pub fn parse_rule(&mut self) -> Result<Rule, &str> {
        let mut rule = Rule::Empty;

        self.consume_while(|c| c == Token::WhitespaceToken);
        if self.eot() {
            // Return a syntax error
            return Err("syntax");
        } else {
            match self.next_token() {
                Token::AtKeywordToken(name) => return Ok(self.consume_at_rule()),
                _ => rule = self.consume_qualified_rule(),
            }
            // Return Syntax error if rule empty
        }
        self.consume_while(|c| c == Token::WhitespaceToken);
        if self.next_token() == Token::EOFToken {
            return Ok(rule);
        }
        return Err("syntax");
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, &str>{
        self.consume_while(|c| c == Token::WhitespaceToken);
        let token = self.consume_token();
        match self.next_token() {
            Token::IdentToken(name) => {
                match self.consume_declaration() {
                    Some(dec) => return Ok(dec),
                    None => return Err("syntax"),
                }
            },
            _ => return Err("syntax"),
        }
    }

    pub fn parse_declaration_list(&mut self) -> Vec<Declaration> {
        return self.consume_declarations();
    }

    pub fn parse_component_value(&mut self) -> Result<ComponentValue, &str> {
        self.consume_while(|c| c == Token::WhitespaceToken);
        if self.next_token() == Token::EOFToken {
            return Err("syntax");
        }
        let value = self.consume_component_value();
        self.consume_while(|c| c == Token::WhitespaceToken);
        if self.eot() {
            return Ok(value);
        } else {
            return Err("syntax");
        }
    }

    pub fn parse_component_value_list(&mut self) -> Vec<ComponentValue> {
        let mut values = Vec::new();
        while !self.eot() {
            values.push(self.consume_component_value());
        }
        return values;
    }

    // https://drafts.csswg.org/css-syntax/#parse-comma-separated-list-of-component-values
    pub fn parse_csv_component_values(&mut self) {
    }


    //
    // Consumption
    //

    // Read the current character without consuming it
    fn next_token(&self) -> Token {
        return self.tokens[self.pos + 1].clone();
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_token(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        return token;
    }

    fn reconsume_token(&mut self) {
        self.pos -= 1;
    }

    // End of tokens
    fn eot(&self) -> bool {
        return self.pos >= self.tokens.len();
    }

    // Consume characters until test returns false
    fn consume_while<F>(&mut self, test: F) -> Vec<Token> where F: Fn(Token) -> bool {
        let mut tokens = Vec::new();
        while !self.eot() && test(self.next_token()) {
            tokens.push(self.consume_token().clone());
        }
        return tokens;
    }

    // https://drafts.csswg.org/css-syntax/#consume-a-list-of-rules
    fn consume_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        while !self.eot() {
            match self.consume_token() {
                Token::WhitespaceToken => {},
                Token::CDOToken | Token::CDCToken => {
                    if !self.top_level {
                        self.reconsume_token();
                        rules.push(self.consume_qualified_rule());
                    }
                },
                Token::AtKeywordToken(name) => {
                    self.reconsume_token();
                    rules.push(self.consume_at_rule());
                }
                _ => {
                    self.reconsume_token();
                    rules.push(self.consume_qualified_rule());
                }
            }
        }
        return rules;
    }

    // https://drafts.csswg.org/css-syntax/#consume-a-qualified-rule
    fn consume_qualified_rule(&mut self) -> Rule {
        let mut rule_data = RuleData::new();

        while !self.eot() {
            let token = self.consume_token();
            match token {
                // Don't do anything (parse error) if we've reached the EOF
                // TODO: Return Option<Rule> instead so we can do Some(Rule) or None instead of
                // using empty for the return type
                Token::EOFToken => return Rule::Empty,
                Token::LeftCurlyBracketToken => {
                    rule_data.block = self.consume_simple_block(token);
                    return Rule::BasicRule(rule_data);
                }
                // Simple Block? Not sure this part is necessary given above
                _ => {
                    self.reconsume_token();
                    rule_data.prelude.push(self.consume_component_value());
                }
            }
        }

        return Rule::BasicRule(rule_data);
    }

    // https://drafts.csswg.org/css-syntax/#consume-at-rule
    fn consume_at_rule(&mut self) -> Rule {
        let mut rule_data = RuleData::new();
        match self.consume_token() {
            Token::AtKeywordToken(name) => rule_data.name = name,
            _ => {}
        }

        while !self.eot() {
            let token = self.consume_token();
            match token {
                Token::SemiColonToken => return Rule::AtRule(rule_data),
                Token::EOFToken => return Rule::AtRule(rule_data),
                Token::LeftCurlyBracketToken => {
                    rule_data.block = self.consume_simple_block(token);
                    return Rule::AtRule(rule_data);
                }
                _ => {
                    self.reconsume_token();
                    rule_data.prelude.push(self.consume_component_value());
                }
            }
        }

        return Rule::AtRule(rule_data);
    }

    // https://drafts.csswg.org/css-syntax/#consume-a-simple-block
    // NOTE: This algorithm assumes that the current input token has already been consumed and
    //       checked to be an <{-token>, <[-token>, or <(-token>.
    fn consume_simple_block(&mut self, block_token: Token) -> Block {
        // TODO: Fix the NOTE: the compiler gives regarding using end_token in match
        let end_token = match block_token {
            Token::LeftCurlyBracketToken => Token::RightCurlyBracketToken,
            Token::LeftSquareBracketToken => Token::RightSquareBracketToken,
            Token::LeftParenToken => Token::RightParenToken,
            // Default to } because that's probably what was meant anyway
            _ => Token::RightSquareBracketToken,
        };

        let mut block = BlockData::new(block_token);

        while !self.eot() {
            let token = self.consume_token();
            match token {
                end_token => break,
                Token::EOFToken => break,
                _ => {
                    self.reconsume_token();
                    block.value.push(self.consume_component_value());
                }
            }
        }

        return Block::SimpleBlock(block);
    }

    // https://drafts.csswg.org/css-syntax/#consume-a-component-value
    fn consume_component_value(&mut self) -> ComponentValue {
        let token = self.consume_token();
        match token {
            Token::LeftCurlyBracketToken |
            Token::LeftSquareBracketToken |
            Token::LeftParenToken => ComponentValue::Block(self.consume_simple_block(token)),
            // Token::FunctionToken => ComponentValue::Block(self.consume_function()),
            _ => ComponentValue::Token(token),
        }
    }

    // https://drafts.csswg.org/css-syntax/#consume-a-function
    fn consume_function(&mut self) -> Block {
        let mut block_data = BlockData::new(Token::LeftParenToken);
        while !self.eot() {
            let token = self.consume_token();
            match token {
                Token::RightParenToken => break,
                _ => {
                    self.reconsume_token();
                    block_data.value.push(self.consume_component_value());
                }
            }
        }
        Block::FunctionBlock(block_data)
    }

    fn consume_declaration(&mut self) -> Option<Declaration> {
        return Some(Declaration{});
    }

    fn consume_declarations(&mut self) -> Vec<Declaration> {
        return Vec::new();
    }

}
