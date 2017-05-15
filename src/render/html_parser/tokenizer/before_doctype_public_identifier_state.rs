use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_before_doctype_public_identifier_state(&mut self) -> Vec<Token> {
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
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Ignore the character
                Vec::new()
            },
            '"' | '\u{0022}' => {
                // Set the DOCTYPE token’s public identifier to the empty string (not missing),
                self.edit_doctype_token(|data| data.public_identifier = Some(String::new()));

                // then switch to the DOCTYPE public identifier (double-quoted) state.
                self.state = State::DOCTYPEPublicIdentifierDoubleQuotedState;
                Vec::new()
            },
            '\'' | '\u{0027}' => {
                // Set the DOCTYPE token’s public identifier to the empty string (not missing),
                self.edit_doctype_token(|data| data.public_identifier = Some(String::new()));

                // then switch to the DOCTYPE public identifier (double-quoted) state.
                self.state = State::DOCTYPEPublicIdentifierSingleQuotedState;
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Set the DOCTYPE token’s force-quirks flag to on.
                self.edit_doctype_token(|data| data.force_quirks = true);

                // Switch to the data state.
                self.state = State::DataState;

                // Emit that DOCTYPE token.
                vec_with_token(self.current_token())
            },
            _ => {
                // Parse error.
                // Set the DOCTYPE token’s force-quirks flag to on.
                self.edit_doctype_token(|data| data.force_quirks = true);

                // Switch to the bogus DOCTYPE state.
                self.state = State::BogusDOCTYPEState;
                Vec::new()
            }
        }
    }
}
