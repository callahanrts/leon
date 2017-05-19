use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_before_doctype_name_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();
            // Parse error.
            // Create a new DOCTYPE token.
            let mut data = DoctypeData::new(String::new());

            // Set its force-quirks flag to on.
            data.force_quirks = true;

            // Emit the token.
            tokens.push(Token::DoctypeToken(data));

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                // Ignore the character
                Vec::new()
            },
            x if is_upper_ascii(x) => {
                // Create a new DOCTYPE token.
                // Set the token’s name to the lowercase version of the current input
                // character (add 0x0020 to the character’s code point).
                let data = DoctypeData::new(String::from(lowercase_char(x).to_string()));
                self.current_token = Some(Token::DoctypeToken(data));

                // Switch to the DOCTYPE name state.
                self.state = State::DOCTYPENameState;
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error.
                // Create a new DOCTYPE token.
                // Set the token’s name to a U+FFFD REPLACEMENT CHARACTER character.
                let data = DoctypeData::new(String::from('\u{FFFD}'.to_string()));
                self.current_token = Some(Token::DoctypeToken(data));

                // Switch to the DOCTYPE name state.
                self.state = State::DOCTYPENameState;
                Vec::new()
            },
            '>' => {
                // Parse error.
                // Create a new DOCTYPE token.
                let mut data = DoctypeData::new(String::new());

                // Set its force-quirks flag to on.
                data.force_quirks = true;

                // Switch to the data state.
                self.state = State::DataState;

                // Emit the token.
                vec_with_token(Token::DoctypeToken(data))
            },
            c => {
                // Create a new DOCTYPE token.
                // Set the token’s name to the current input character.
                let data = DoctypeData::new(String::from(c.to_string()));
                self.current_token = Some(Token::DoctypeToken(data));

                // Switch to the DOCTYPE name state.
                self.state = State::DOCTYPENameState;
                Vec::new()
            }
        }
    }
}
