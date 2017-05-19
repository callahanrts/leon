use super::*;

impl<'a> Tokenizer<'a> {
    pub fn consume_data_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '&' => {
                // Set the return state to the data state
                self.return_state = State::DataState;
                // Switch to the character reference state
                self.state = State::CharReferenceState;
                return Vec::new();
            }
            '<' => {
                // Switch to the tag open state. We're reading an open tag
                self.state = State::TagOpenState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Return character in a CharToken
                return vec_with_token(Token::CharToken('\u{0000}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }
}
