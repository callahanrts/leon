use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_attr_value_unquoted_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the before attribute name state.
                self.state = State::BeforeAttrNameState;
                Vec::new()
            },
            '&' | '\u{0026}' => {
                // Set the return state to the attribute value (unquoted) state.
                self.return_state = State::AttrValueUnquotedState;

                // Switch to the character reference state.
                self.state = State::CharReferenceState;
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current tag token.
                vec_with_token(self.current_token())
            },
            '\u{0000}' => {
                // Parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the
                // current attribute’s value.
                self.edit_current_tag(|tag| tag.append_attr_value('\u{FFFD}'));
                Vec::new()
            },
            '"' | '\u{0022}' |  '\'' | '\u{0027}' |  '<' | '\u{003C}' |  '=' | '\u{003D}' |  '`' | '\u{0060}'  => {
                // Parse error. Treat it as per the "anything else" entry below.
                self.edit_current_tag(|tag| tag.append_attr_value(cur));
                Vec::new()
            },
            x => {
                // Append the current input character to the current attribute’s value.
                self.edit_current_tag(|tag| tag.append_attr_value(x));
                Vec::new()
            }
        }
    }

}



