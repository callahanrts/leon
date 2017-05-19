use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_rawtext_less_than_sign_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '/' => {
                // Set the temporary buffer to the empty string.
                self.tmp_buffer = String::new();

                // Switch to the RAWTEXT end tag open state.
                self.state = State::RawtextEndTagOpenState;
                Vec::new()
            },
            _ => {
                // Reconsume in the RAWTEXT state.
                self.reconsume_char();
                self.state = State::RawtextState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            }
        }
    }

}
