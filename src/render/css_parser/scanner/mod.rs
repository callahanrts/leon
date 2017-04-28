#[cfg(test)]
mod test;
use std::ascii::AsciiExt;

// # Scanner
// A struct to store an input string and the position while parsing
struct Scanner {
    pos: usize,
    input: String,
}

enum Token {
    // General Tokens
    WhitespaceToken,
    EOFToken,    // End of file
    DelimToken(char), // Anything else

    // Char Tokens
    LeftParenToken,
    RightParenToken,
    CommaToken,
    ColonToken,
    SemiColonToken,
    LeftSquareBracketToken,
    LeftCurlyBracketToken,
    RightCurlyBracketToken,

    // String Tokens
    StringToken(String),
    BadStringToken,

    // Number Tokens
    PercentageToken(f32),
    DimensionToken{value: f32, num_type: String, unit: String},
    NumberToken{value: f32, num_type: String},

    // URLs
    UrlToken(String),
    BadUrlToken,

    // CD Tokens
    CDOToken,
    CDCToken,

    HashToken { hash_type: String, name: String },
    FunctionToken(String),
    IdentToken(String),
    AtKeywordToken(String),
}

// # Scanner
// Implementation of a scanner. Consume characters and return tokens
impl Scanner {
    //
    // Basic String scanning methods
    //

    // Read the current character without consuming it
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn nth_char(&self, offset: usize) -> char{
        return self.input[(self.pos + offset - 1)..].chars().next().unwrap();
    }

    // Do the next characters start with the given string?
    // NOTE: Starts with compares as lower case
    fn starts_with(&self, s: &str) -> bool {
        // return self.input[self.pos..].to_lowercase().starts_with(s);
        return self.input[self.pos..].starts_with(s);
    }

    // Return true if all input is consumed
    fn eof(&self) -> bool {
        return self.pos >= self.input.len();
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // Consume characters until test returns false
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    // https://drafts.csswg.org/css-syntax/#consume-token
    fn consume_token(&mut self) -> Token {
        // Consume comments
        self.consume_comment();

        // Retrieve the next character in the input, but don't consume it yet. Consumption
        // will be handled by pattern matching below
        let code_point = self.next_char();

        // Return EOF Token if we're at the end of the stream
        if self.eof() {
            return Token::EOFToken;
        }

        // whitespace
        if char::is_whitespace(code_point) {
            return self.consume_whitespace();
        }

        match code_point {
            // Quotation mark "
            '"' => self.consume_string_token(),
            // Number sign #
            '#' => self.consume_hash_token(),
            // apostrophe '
            '\'' => self.consume_string_token(),
            // Left paren (
            '(' => self.consume_then_token(Token::LeftParenToken),
            // Right paren )
            ')' => self.consume_then_token(Token::RightParenToken),
            // Plus Sign +
            '+' => self.consume_plus(),
            // Comma ,
            ',' => self.consume_then_token(Token::CommaToken),
            // Hyphen-minus -
            '-' => self.consume_minus(),
            // Full Stop .
            '.' => self.consume_full_stop(),
            // Colon :
            ':' => self.consume_then_token(Token::ColonToken),
            // Semi-colon ;
            ';' => self.consume_then_token(Token::SemiColonToken),
            // Less than Sign <
            '<' => self.consume_less_than(),
            // Commercial at @
            '@' => self.consume_at_keyword_token(),
            // Left square bracket [
            '[' => self.consume_then_token(Token::LeftSquareBracketToken),
            // Left curly bracket {
            '{' => self.consume_then_token(Token::LeftCurlyBracketToken),
            // Right curly bracket }
            '}' => self.consume_then_token(Token::RightCurlyBracketToken),
            // digit
            '0' ... '9' => self.consume_number_token(),
            // Anything else
            _ => {
                if self.eof() {
                    return Token::EOFToken;
                } else if name_start_code_point(code_point) {
                    return self.consume_ident_token();
                }
                return self.consume_then_token(Token::DelimToken(code_point));
            }
        }
    }

    // Consume a character and then return the token
    fn consume_then_token(&mut self, token: Token) -> Token {
        self.consume_char();
        return token;
    }

    //
    // Scan tokens
    //

    // comment
    fn consume_comment(&mut self) {
        if self.starts_with("/*") {
            // Consume the beginning of a comment
            self.consume_char();
            self.consume_char();

            // Consume everything within the comment, or up to eof
            while !self.eof() && !self.starts_with("*/") {
                self.consume_char();
            }

            // Consume the end comment unless we're at the end of the file
            if self.starts_with("*/") {
                self.consume_char();
                self.consume_char();
            }
        }
    }

    // newline, whitespace, <whitespace-token>, ws*
    // Consume and discard 0 or more whitespace characters and return a whitespace token
    fn consume_whitespace(&mut self) -> Token {
        self.consume_while(char::is_whitespace);
        return Token::WhitespaceToken;
    }

    // <hash-token>
    fn consume_hash_token(&mut self) -> Token {
        self.consume_char(); // Consume the # character

        // If the next input code point is a name code point
        if name_code_point(self.next_char()) {
            // Create a <hash-token>.
            // If the next 3 input code points would start an identifier, set the <hash-token>’s type flag to "id".
            let mut hash_type = String::new();
            if would_be_identifier(self) {
                hash_type.push_str("id");
            }
            // Consume a name, and set the <hash-token>’s value to the returned string.
            let name = self.consume_name();

            // Return the <hash-token>.
            Token::HashToken{hash_type: hash_type, name: name}
        } else {
            // Otherwise, return a <delim-token> with its value set to the current input code point.
            Token::DelimToken(self.consume_char())
        }
    }

    fn consume_name(&mut self) -> String {
        self.consume_while(|c| name_code_point(c))
    }

    // <string-token>
    // https://drafts.csswg.org/css-syntax/#consume-a-string-token
    fn consume_string_token(&mut self) -> Token {
        let code_point = self.consume_char();
        let result = self.consume_string(code_point);

        if !self.eof() {
            // Consume the end string character
            if self.next_char() == code_point {
                self.consume_char();
            }

            // Newlines in strings are parse errors, return a <bad-string-token>
            if newline(self.next_char()) {
                return Token::BadStringToken;
            }
        }

        return Token::StringToken(result);
    }

    fn consume_string(&mut self, code_point: char) -> String {
        let mut result = String::new();
        while !self.eof() && self.next_char() != code_point && !newline(self.next_char()) {
            // If the next character starts with a \
            // TODO: Fix the solidus thing
            if self.next_char() == '\\' {
                self.consume_char(); // Consume the \

                // In the case of an escaped newline, consume the newline
                if newline(self.next_char()) {
                    self.consume_char();
                }

                // EOF is a parse error, with which we should do nothing. Otherwise,
                // consume the character and append to the result
                if !self.eof() {
                    result.push(self.consume_char());
                }
            }

            // Append character to string
            result.push(self.consume_char());
        }
        return result;
    }

    // <number-token>, <dimension-token>, <percentage-token>
    // https://drafts.csswg.org/css-syntax/#consume-a-numeric-token
    fn consume_number_token(&mut self) -> Token {
        let (val, num_type) = self.consume_number();
        if would_be_identifier(self) {
            let unit = self.consume_name();
            Token::DimensionToken{value: val, num_type: num_type, unit: unit}
        } else if self.next_char() == '%' {
            self.consume_char(); // Consume the %
            Token::PercentageToken(val)
        } else {
            Token::NumberToken{value: val, num_type: num_type}
        }
    }

    fn consume_number(&mut self) -> (f32, String) {
        let mut num_type = String::from("integer");
        let mut repr = String::new(); // Not sure what repr stands for?

        // Consume + / -
        if self.next_char() == '+' || self.next_char() == '-' {
            repr.push(self.consume_char());
        }

        // Consume all digits
        while is_number(self.next_char()) {
            repr.push(self.consume_char());
        }

        // If the number has a decimal point, consume the point and the following digits
        if self.next_char() == '.' && is_number(self.nth_char(2)) {
            repr.push(self.consume_char());
            num_type = String::from("number");
            while is_number(self.next_char())  {
                repr.push(self.consume_char());
            }
        }

        // If the number has e+10 etc
        if (self.next_char() == 'E' || self.next_char() == 'e') && (self.nth_char(2) == '+' || self.nth_char(2) == '-') {
            repr.push(self.consume_char()); // consume e/E
            repr.push(self.consume_char()); // consume +/-
            num_type = String::from("number");
            while is_number(self.next_char())  {
                repr.push(self.consume_char());
            }
        }
        let f = repr.parse::<f32>().unwrap();

        return (f, num_type);
    }

    fn consume_plus(&mut self) -> Token {
        // Consume the +
        let code_point = self.consume_char();
        if is_number(self.next_char()) {
            self.consume_number_token()
        } else {
            Token::DelimToken(code_point)
        }
    }

    fn consume_minus(&mut self) -> Token {
        // If the second character is a number (eg. -2)
        if is_number(self.nth_char(2)) {
            return self.consume_number_token();
        } else {
            self.consume_char();
            if self.nth_char(2) == '>' {
                // Consume both tokens and return <CDC-token>
                return self.consume_cdc();
            } else if would_be_identifier(self) {
                // Consume and return ident token
                return self.consume_ident_token();
            }

            // Return delim token with '-'
            return Token::DelimToken(self.consume_char());
        }
    }

    // <CDC-token>
    fn consume_cdc(&mut self) -> Token {
        self.consume_char(); // -
        self.consume_char(); // -
        self.consume_char(); // >
        Token::CDCToken
    }

    // <ident-token>
    // <function-token>
    fn consume_ident_token(&mut self) -> Token {
        let ident = self.consume_name();
        // If the returned string’s value is an ASCII case-insensitive match for "url"
        // and the next input code point is U+0028 LEFT PARENTHESIS ((),
        if ident.to_lowercase() == "url" && self.next_char() == '(' {
            // consume it
            self.consume_char();

            //   While the next two input code points are whitespace,
            //     consume the next input code point.
            self.consume_whitespace();

            //   If the next one or two input code points are U+0022 QUOTATION MARK ("), U+0027
            //   APOSTROPHE ('), or whitespace followed by U+0022 QUOTATION MARK (") orU+0027
            //   APOSTROPHE ('),
            if self.next_char() == '\'' || self.next_char() == '"' {
                //     then create a <function-token> with its value set to the returned string
                //     and return it.
                return Token::FunctionToken(ident);
            } else {
                //  Otherwise, consume a url token, and return it.
                return self.consume_url_token();
            }
        } else if self.next_char() == '(' {
            return Token::FunctionToken(ident);
        }
        return Token::IdentToken(ident);
    }

    // <url-token>
    // NOTE: This algorithm assumes that the initial "url(" has already been consumed.
    // https://drafts.csswg.org/css-syntax/#consume-a-url-token
    fn consume_url_token(&mut self) -> Token {
        let mut result = String::new();
        self.consume_whitespace();
        if !self.eof() {
            loop {
                let next_char = self.consume_char();
                self.consume_whitespace();
                if next_char == ')' || self.eof() {
                    break;
                }

                match next_char {
                    '"' | '\'' | '(' => {
                        self.consume_bad_url_remnants();
                        return Token::BadUrlToken;
                    },
                    _ => result.push(next_char),
                }
            }
        }
        return Token::UrlToken(result);
    }

    fn consume_bad_url_remnants(&mut self) {
        self.consume_while(|c| c != ')');
        if !self.eof() && self.next_char() == ')' {
            self.consume_char();
        }
    }

    fn consume_full_stop(&mut self) -> Token {
        if is_number(self.nth_char(2)) {
            self.consume_number_token()
        } else {
            Token::DelimToken(self.consume_char())
        }
    }

    // <CDO-token>
    fn consume_less_than(&mut self) -> Token {
        let lt = self.consume_char(); // less than '<'
        if self.starts_with("!--") {
            self.consume_char();
            self.consume_char();
            self.consume_char();
            Token::CDOToken
        } else {
            Token::DelimToken(lt)
        }
    }

    fn consume_at_keyword_token(&mut self) -> Token {
        let at = self.consume_char(); // consume '@'
        if would_be_identifier(self) {
            Token::AtKeywordToken(self.consume_name())
        } else {
            Token::DelimToken(at)
        }
    }

}

//
// Code Points
//

fn is_number(c: char) -> bool {
    match c {
        '0' ... '9' => true,
        _ => false,
    }
}

// Do the next 3 characters start a number
fn start_of_number(s: &Scanner) -> bool {
    match s.next_char() {
        '+' | '-' => is_number(s.nth_char(2)) || (s.nth_char(2) == '.' && is_number(s.nth_char(3))),
        '.' => is_number(s.nth_char(2)),
        _ => is_number(s.next_char()),
    }
}

// Is the next character a name-start-code-point
fn name_start_code_point(c: char) -> bool {
    match c {
        'A' ... 'Z' => true,
        'a' ... 'z' => true,
        '_' => true,
        _ => false,
    }
}

// Is the next character a name-code-point
fn name_code_point(c: char) -> bool {
    name_start_code_point(c) || match c {
        '0' ... '9' => true,
        '-' => true,
        _ => false,
    }
}

fn would_be_identifier(s: &Scanner) -> bool {
    name_start_code_point(s.next_char()) && name_code_point(s.nth_char(2))
}

fn newline(c: char) -> bool{
    match c {
        '\n' | '\r' => true,
        _ => false,
    }
}
