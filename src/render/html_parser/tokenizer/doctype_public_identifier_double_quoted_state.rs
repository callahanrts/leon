use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_doctype_public_identifier_double_quoted_state(&mut self) -> Vec<Token> {
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
            '"' => {
                // Switch to the after DOCTYPE public identifier state.
                self.state = State::AfterDOCTYPEPublicIdentifierState;
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the current
                // DOCTYPE token’s public identifier.
                self.edit_doctype_token(|data| data.append_public_identifier('\u{FFFD}'));
                Vec::new()
            },
            '>' => {
                // Parse error.
                // Set the DOCTYPE token’s force-quirks flag to on.
                self.edit_doctype_token(|data| data.force_quirks = true);

                // Switch to the data state.
                self.state = State::DataState;

                // Emit that DOCTYPE token.
                vec_with_token(self.current_token())
            },
            c => {
                // Append the current input character to the current DOCTYPE token’s public identifier.
                self.edit_doctype_token(|data| data.append_public_identifier(c));
                Vec::new()
            }
        }
    }
}
