#[cfg(test)]
mod test;
use tokenizer::{Token};

pub struct TokenIterator {
    pos: usize,
    tokens: Vec<Token>,
    reconsume: bool,
}

impl TokenIterator {
    pub fn new(tokens: Vec<Token>) -> TokenIterator {
        TokenIterator {
            pos: 0,
            tokens: tokens,
            reconsume: true, // Treat the first token as nil on create
        }
    }

    // The token or component value following the current input token in the list of tokens
    // produced by the tokenizer. If there isn't a token following the current input token,
    // the next input token is an <EOF-token>.
    pub fn next_token(&self) -> Token {
        if self.eot() {
            // A conceptual token representing the end of the list of tokens. Whenever the list
            // of tokens is empty, the next input token is always an <EOF-token>.
            Token::EOFToken
        } else {
            self.tokens[self.pos + 1].clone()
        }
    }

    pub fn nth_token(&self, n: usize) -> Token {
        self.tokens[self.pos + n].clone()
    }

    // The token or component value currently being operated on, from the list of tokens
    // produced by the tokenizer.
    pub fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    // Let the current input token be the current next input token,
    // adjusting the next input token accordingly.
    pub fn consume_token(&mut self) {
        // Don't consume another token if we're reconsuming.
        if !self.reconsume {
            self.pos += 1;
        }
        self.reconsume = false; // Reset the reconsume flag
    }

    // The next time an algorithm instructs you to consume the next input token,
    // instead do nothing (retain the current input token unchanged).
    pub fn reconsume_token(&mut self) {
        self.reconsume = true;
    }

    // End of tokens
    pub fn eot(&self) -> bool {
        self.pos >= self.tokens.len() - 1
    }

    // Consume characters until test returns false
    pub fn consume_while<F>(&mut self, test: F) -> Vec<Token> where F: Fn(Token) -> bool {
        let mut tokens = Vec::new();
        while !self.eot() && test(self.current_token()) {
            self.consume_token();
            tokens.push(self.current_token().clone());
        }
        return tokens;
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    // For testing
    pub fn print_remaining(&mut self) {
        // let tokens = self.tokens.drain(self.pos..).collect();
        print_tokens(self.tokens.clone());
    }

}

fn print_tokens(tokens: Vec<Token>) {
    let mut output = String::new();
    for token in tokens {
        let st = match token {
            Token::WhitespaceToken => "whitespace",
            Token::EOFToken => "EOF",
            Token::DelimToken(_) => "DELIM",
            Token::StringToken(_) => "DELIM",
            Token::BadStringToken => "bad string",
            Token::NumberToken{value: v, num_type: _} => "DELIM",
            Token::DimensionToken{value: v, num_type: _, unit: _} => "DELIM",
            Token::HashToken{ hash_type: _, name: _ } => "HASH",
            Token::FunctionToken(_) => "function",
            Token::IdentToken(_) => "ident",
            Token::CDOToken => "cdo",
            Token::CDCToken => "cdc",
            Token::ColonToken => "colon",
            Token::UrlToken(_) => "url",
            Token::BadUrlToken => "bad-url",
            Token::PercentageToken(_) => "percentage",
            Token::SemiColonToken => "semi-colon",
            _ => "Other Token",
        };
        output.push_str(st);
        output.push_str("\n");
    }
    println!("================\n{}\n====================", output);
}

