use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_decimal_char_reference_state(&mut self) -> Vec<Token> {

        match self.consume_char() {
            x if is_hex(x) => {
                // Multiply the character reference code by 16.
                self.char_reference_code *= 16;

                // Add a numeric version of the current input character character reference code.
                self.char_reference_code += i64::from_str_radix(&*x.to_string(), 16).unwrap();
                Vec::new()
            },
            ';' => {
                // Switch to the numeric character reference end state.
                self.state = State::NumericCharReferenceEndState;
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
