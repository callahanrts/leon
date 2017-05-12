use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_before_attr_name_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Ignore the character
                Vec::new()
            },
            '/' | '\u{002F}' | '>' | '\u{003E}' => {
                // Reconsume in the after attribute name state.
                self.reconsume_char();
                self.state = State::AfterAttrNameState;
                Vec::new()
            },
            '=' | '\u{003E}' => {
                // Parse error.
                // Start a new attribute in the current tag token.
                // Set that attribute’s name to the current input character,
                // and its value to the empty string.
                self.edit_current_tag(|tag| tag.attributes.push(Attribute{
                    name: format!("{}", cur),
                    value: String::new()
                }));

                // Switch to the attribute name state.
                self.state = State::AttrNameState;
                Vec::new()
            },
            _ => {
                // Start a new attribute in the current tag token.
                // Set that attribute’s name and value to the empty string.
                self.edit_current_tag(|tag| tag.append_attribute(Attribute{
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

