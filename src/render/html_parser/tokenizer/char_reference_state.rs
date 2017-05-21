use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_char_reference_state(&mut self) -> Vec<Token> {
        // Set the temporary buffer to the empty string.
        // Append a U+0026 AMPERSAND (&) character to the temporary buffer.
        self.tmp_buffer = String::from("#");

        if self.eof() {
            self.reconsume_char();
            self.state = State::CharReferenceEndState;
            return Vec::new()
        }

        match self.consume_char() {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' | '<' | '&' => {
                // Reconsume in the character reference end state
                self.reconsume_char();
                self.state = State::CharReferenceEndState;
                Vec::new()
            }
            '#' => {
                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push('#');

                // Switch to the numeric character reference state.
                self.state = State::NumericCharReferenceState;
                Vec::new()
            },
            _ => {
                // TODO:
                // Consume the maximum number of characters possible, with the consumed characters
                // matching one of the identifiers in the first column of the §8.5 Named character
                // references table (in a case-sensitive manner). Append each character to the
                // temporary buffer when it’s consumed.

                // If no match can be made and the temporary buffer consists of a U+0026 AMPERSAND
                // character (&) followed by a sequence of one or more alphanumeric ASCII characters
                // and a U+003B SEMICOLON character (;), then this is a parse error.

                // If no match can be made, switch to the character reference end state.

                // If the character reference was consumed as part of an attribute (return state
                // is either attribute value (double-quoted) state, attribute value (single-quoted)
                // state or attribute value (unquoted) state), and the last character matched is
                // not a U+003B SEMICOLON character (;), and the next input character is either a
                // U+003D EQUALS SIGN character (=) or an alphanumeric ASCII character, then, for
                // historical reasons, switch to the character reference end state.

                // If the last character matched is not a U+003B SEMICOLON character (;), this is
                // a parse error.

                // Set the temporary buffer to the empty string. Append one or two characters
                // corresponding to the character reference name (as given by the second column of
                // the §8.5 Named character references table) to the temporary buffer.

                // Switch to the character reference end state.
                Vec::new()
            }
        }
    }
}
