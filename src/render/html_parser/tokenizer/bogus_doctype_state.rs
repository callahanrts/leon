use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_bogus_doctype_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();

            // Emit that DOCTYPE token.
            tokens.push(self.current_token());

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '>' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current DOCTYPE token.
                vec_with_token(self.current_token())
            },
            _ => {
                // Ignore the character
                Vec::new()
            }
        }
    }
}
