use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_before_attr_value_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                // Ignore the character
                Vec::new()
            },
            '"' => {
                // Switch to the attribute value (double-quoted) state.
                self.state = State::AttrValueDoubleQuotedState;
                Vec::new()
            },
            '\'' => {
				// Switch to the attribute value (single-quoted) state.
                self.state = State::AttrValueSingleQuotedState;
                Vec::new()
            },
            '>' => {
				// Parse error. Treat it as per the "anything else" entry below.
                self.reconsume_char();
                self.state = State::AttrValueUnquotedState;
                Vec::new()
            },
            _ => {
 				// Reconsume in the attribute value (unquoted) state.
                self.reconsume_char();
                self.state = State::AttrValueUnquotedState;
                Vec::new()
            }
        }
    }

}


