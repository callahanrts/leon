use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_hex_char_reference_start_state(&mut self) -> Vec<Token> {

        match self.consume_char() {
            c if is_hex(c) => {
                // Reconsume in the hexadecimal character reference state.
                self.reconsume_char();
                self.state = State::HexCharReferenceState;
                Vec::new()
            },
            _ => {
                // Parse error.
                // Reconsume in the character reference end state.
                self.reconsume_char();
                self.state = State::CharReferenceEndState;
                Vec::new()
            }
        }
    }
}
