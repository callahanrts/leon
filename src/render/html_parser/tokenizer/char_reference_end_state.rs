use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_char_reference_end_state(&mut self) -> Vec<Token> {

        let next = self.next_char();
        let ref buf = self.tmp_buffer;

        match self.return_state {
            State::AttrValueDoubleQuotedState |
            State::AttrValueSingleQuotedState |
            State::AttrValueUnquotedState => {
                // Append each character in the temporary buffer (in the order they
                // were added to the buffer) to the current attribute’s value.
                self.state = self.return_state.clone();
                Vec::new()
            }
            _ => {
                let mut tokens = Vec::new();

                // For each of the characters in the temporary buffer
                // (in the order they were added to the buffer),
                // emit the character as a character token.
                for c in buf.chars() {
                    tokens.push(Token::CharToken(c));
                }

                self.state = self.return_state.clone();

                tokens
            }
        }

    }

}
