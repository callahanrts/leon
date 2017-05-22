use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_hex_char_reference_state(&mut self) -> Vec<Token> {

        match self.consume_char() {
            x if is_hex(x) => {
                // Multiply the character reference code by 16.
                self.char_reference_code *= 16;

                // Add a numeric version of the current input character as a hexademical
                // digit (subtract 0x0057 from the character’s code point) to the
                // character reference code.
                self.char_reference_code += i64::from_str_radix(&*x.to_string(), 16).unwrap();
                Vec::new()
            },
            ';' => {
                // Switch to the numeric character reference end state.
                self.state = State::CharReferenceEndState;
                Vec::new()
            }
            _ => {
                // Parse error.
                // Reconsume in the numeric character reference end state.
                self.reconsume_char();
                self.state = State::NumericCharReferenceEndState;
                Vec::new()
            }
        }
    }
}
