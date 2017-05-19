use super::*;

impl<'a> Tokenizer<'a> {
    pub fn consume_rcdata_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '&' => {
                // Set the return state to the RCDATA state.
                self.return_state = State::RCDataState;

                // Switch to the character reference state.
                self.state = State::CharReferenceState;
                return Vec::new();
            }
            '<' => {
                self.state = State::RCDataLessThanSignState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return vec_with_token(Token::CharToken('\u{FFFD}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }
}
