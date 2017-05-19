use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_rcdata_less_than_sign_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            '/' => {
                // Set the temporary buffer to the empty string.
                // Switch to the RCDATA end tag open state.
                self.state = State::RCDataEndTagOpenState;
                return Vec::new();
            }
            _ => {
                let mut tokens = Vec::new();
                // Reconsume in the RCDATA state.
                self.reconsume_char();
                self.state = State::RCDataState;

                // Emit a U+003C LESS-THAN SIGN character token.
                tokens.push(Token::CharToken('<'));
                return tokens;
            }
        }
    }

}
