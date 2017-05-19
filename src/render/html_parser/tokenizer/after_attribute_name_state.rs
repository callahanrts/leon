use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_after_attr_name_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                // Ignore the character
                Vec::new()
            },
            '/' => {
                // Switch to the self-closing start tag state.
                self.state = State::SelfClosingStartTagState;
                Vec::new()
            },
            '=' => {
                // Switch to the before attribute value state.
                self.state = State::BeforeAttrValueState;
                Vec::new()
            },
            '>' => {
                // Switch to the data state. Emit the current tag token.
                self.state = State::DataState;
                vec_with_token(self.current_token())
            },
            _ => {
                // Start a new attribute in the current tag token.
                // Set that attribute’s name and value to the empty string.
                self.edit_current_tag(|tag| tag.append_attribute(Attribute {
                    name: String::new(),
                    value: String::new()
                }));

                // Reconsume in the attribute name state.
                self.reconsume_char();
                self.state = State::AttrNameState;
                Vec::new()
            }
        }
    }

}


