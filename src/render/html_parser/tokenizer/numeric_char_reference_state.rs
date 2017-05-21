use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_numeric_char_reference_state(&mut self) -> Vec<Token> {

        match self.consume_char() {
            'x' | 'X' => {
                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push('x');

                // Switch to the hexadecimal character reference start state.
                self.state = State::HexCharReferenceStartState;
                Vec::new()
            },
            _ => {
                // Reconsume in the decimal character reference start state.
                self.reconsume_char();
                self.state = State::DecimalCharReferenceStartState;
                Vec::new()
            }
        }
    }
}

