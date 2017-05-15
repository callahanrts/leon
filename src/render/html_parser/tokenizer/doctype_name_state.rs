use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_doctype_name_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();
            // Parse error.
            // Set the DOCTYPE token’s force-quirks flag to on.
            self.edit_doctype_token(|data| data.force_quirks = true);

            // Emit that DOCTYPE token.
            tokens.push(self.current_token());

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the after DOCTYPE name state.
                self.state = State::DOCTYPENameState;
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current DOCTYPE token.
                vec_with_token(self.current_token())
            },
            x if is_upper_ascii(x) => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current DOCTYPE token’s name.
                self.edit_doctype_token(|data| data.name.push(lowercase_char(x)));
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token’s name.
                self.edit_doctype_token(|data| data.name.push('\u{FFFD}'));
                Vec::new()
            },
            c => {
                // Append the current input character to the current DOCTYPE token’s name.
                self.edit_doctype_token(|data| data.name.push(c));
                Vec::new()
            }
        }
    }
}
