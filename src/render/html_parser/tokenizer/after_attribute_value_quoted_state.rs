use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_after_attr_value_quoted_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                // Switch to the before attribute name state.
                self.state = State::BeforeAttrNameState;
                Vec::new()
            },
            '/' => {
                // Switch to the self-closing start tag state.
                self.state = State::SelfClosingStartTagState;
                Vec::new()
            },
            '>' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current tag token.
                vec_with_token(self.current_token())
            },
            _ => {
                // Parse error. Reconsume in the before attribute name state.
                self.reconsume_char();
                self.state = State::BeforeAttrNameState;
                Vec::new()
            }
        }
    }

}




