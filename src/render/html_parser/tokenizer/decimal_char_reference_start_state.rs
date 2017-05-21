use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_decimal_char_reference_start_state(&mut self) -> Vec<Token> {

        match self.consume_char() {
            '0' ... '9' => {
                // Reconsume in the decimal character reference state.
                self.reconsume_char();
                self.state = State::DecimalCharReferenceStartState;
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
