use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_attr_value_double_quoted_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            '"' => {
                // Switch to the after attribute value (quoted) state.
                self.state = State::AfterAttrValueQuotedState;
                Vec::new()
            },
            '&' => {
                // Set the return state to the attribute value (double-quoted) state.
                self.return_state = State::AttrValueDoubleQuotedState;

                // Switch to the character reference state.
                self.state = State::CharReferenceState;
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the
                // current attribute’s value.
                self.edit_current_tag(|tag| tag.append_attr_value('\u{FFFD}'));
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



