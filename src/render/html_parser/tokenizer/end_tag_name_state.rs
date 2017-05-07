use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_end_tag_name_state(&mut self, new_state: State) -> Vec<Token> {
        match self.consume_char() {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the before attribute name state.
                    self.state = State::BeforeAttrNameState;
                    Vec::new()
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            '/' | '\u{002f}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the self-closing start tag state.
                    self.state = State::SelfClosingStartTagState;
                    Vec::new()
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            '>' | '\u{003E}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the data state and emit the current tag token.
                    self.state = State::DataState;
                    vec_with_token(self.current_token())
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            x if is_upper_ascii(x) => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current tag token’s tag name.
                self.append_char_to_tag_name(lowercase_char(x));

                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push(x);
                Vec::new()
            },
            x if is_lower_ascii(x) => {
                // Append the current input character to the current tag token’s tag name.
                self.append_char_to_tag_name(x);

                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push(x);
                Vec::new()
            },
            _ => {
                self.handle_end_tag_name(new_state)
            }
        }
    }

}
