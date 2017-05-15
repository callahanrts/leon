use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_doctype_state(&mut self) -> Vec<Token> {
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
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the before DOCTYPE name state.
                self.state = State::BeforeDOCTYPENameState;
                Vec::new()
            },
            _ => {
                // Parse error.
                // Reconsume in the before DOCTYPE name state.
                self.reconsume_char();
                self.state = State::BeforeDOCTYPENameState;
                Vec::new()
            }
        }
    }

}
