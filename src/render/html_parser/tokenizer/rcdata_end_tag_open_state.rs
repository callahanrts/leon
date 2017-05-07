use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_rcdata_end_tag_open_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            // ASCII Letter
            x if is_ascii(x) => {
                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

                // Reconsume in RCDATA end tag name state.
                self.reconsume_char();
                self.state = State::RCDataEndTagNameState;
                return Vec::new();
            }
            _ => {
                // Reconsume in the RCDATA state.
                self.reconsume_char();
                self.state = State::RCDataState;

                let mut tokens = Vec::new();
                // Emit a U+003C LESS-THAN SIGN character token
                tokens.push(Token::CharToken('<'));
                // and a U+002F SOLIDUS character token.
                tokens.push(Token::CharToken('/'));
                return tokens;
            }
        };
    }

}
