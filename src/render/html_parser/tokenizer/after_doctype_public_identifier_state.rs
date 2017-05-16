use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_after_doctype_public_identifier_state(&mut self) -> Vec<Token> {
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
                // Switch to the between DOCTYPE public and system identifiers state.
                self.state = State::BetweenDOCTYPEPublicAndSystemIdentifiersState;
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current DOCTYPE token.
                vec_with_token(self.current_token())
            },

            '"' | '\u{0022}' => {
                // Parse error.
                // Set the DOCTYPE token’s system identifier to the empty string (not missing),
                self.edit_doctype_token(|data| data.system_identifier = Some(String::new()));

                // then switch to the DOCTYPE system identifier (double-quoted) state.
                self.state = State::DOCTYPESystemIdentifierDoubleQuotedState;
                Vec::new()
            },
            '\'' | '\u{0027}' => {
                // Parse error.
                // Set the DOCTYPE token’s system identifier to the empty string (not missing),
                self.edit_doctype_token(|data| data.system_identifier = Some(String::new()));

                // then switch to the DOCTYPE system identifier (single-quoted) state.
                self.state = State::DOCTYPESystemIdentifierSingleQuotedState;
                Vec::new()
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
