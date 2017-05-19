use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_self_closing_start_tag_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '>' => {
                // Set the self-closing flag of the current tag token.
                self.edit_current_tag(|tag| tag.set_self_closing(true));

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




