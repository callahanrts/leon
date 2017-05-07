use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_tag_name_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the before attribute name state.
                self.state = State::BeforeAttrNameState;
                return Vec::new();
            },
            '/' | '\u{002F}' => {
                // Switch to the self-closing start tag state.
                self.state = State::SelfClosingStartTagState;
                return Vec::new();
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current tag token.
                return vec_with_token(self.current_token());
            },
            'A' ... 'Z' | '\u{0041}' ... '\u{005A}' => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current tag token’s tag name.
                self.append_char_to_tag_name(lowercase_char(cur));
                return Vec::new();
            },
            '\u{0000}' => {
                // TODO: Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the current tag token’s tag name.
                self.append_char_to_tag_name('\u{FFFD}');
                return Vec::new()
            },
            _ => {
                // Append the current input character to the current tag token’s tag name.
                self.append_char_to_tag_name(cur);
                return Vec::new()
            }

        }
    }

}
