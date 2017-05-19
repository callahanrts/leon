use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_double_escape_end_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' | '/' | '>' => {
                // If the temporary buffer is the string "script",
                if self.tmp_buffer == String::from("script") {
                    // then switch to the script data escaped state.
                    self.state = State::ScriptDataEscapedState;
                // Otherwise,
                } else {
                    // switch to the script data double escaped state.
                    self.state = State::ScriptDataDoubleEscapedState;
                }
                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(cur))
            },
            x if is_upper_ascii(x) => {
                // Append the lowercase version of the current input character (add
                // 0x0020 to the character’s code point) to the temporary buffer.
                self.tmp_buffer.push(lowercase_char(x));

                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(cur))
            },
            x if is_lower_ascii(x) => {
                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push(x);

                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(x))
            },
            _ => {
                // Reconsume in the script data double escaped state.
                self.reconsume_char();
                self.state = State::ScriptDataDoubleEscapedState;
                Vec::new()
            }
        }
    }
}
