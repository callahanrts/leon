use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_after_doctype_system_identifier_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();
            // Parse error.
            // Set the DOCTYPE token’s force-quirks flag to on.
            self.edit_doctype_token(|data| data.force_quirks = true);

            // Emit that DOCTYPE token.
            tokens.push(self.current_token());

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                // Ignore the character
                Vec::new()
            },
            '>' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current DOCTYPE token.
                vec_with_token(self.current_token())
            },
            _ => {
                // Parse error.
                // Switch to the bogus DOCTYPE state. (This does not set the DOCTYPE token’s
                // force-quirks flag to on.)
                self.state = State::BogusDOCTYPEState;
                Vec::new()
            }
        }
    }
}
