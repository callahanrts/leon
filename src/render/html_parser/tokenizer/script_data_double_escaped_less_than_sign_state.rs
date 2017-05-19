use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_double_escaped_less_than_sign_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            '/' => {
                // Set the temporary buffer to the empty string.
				self.tmp_buffer = String::new();

                // Switch to the script data double escape end state.
				self.state = State::ScriptDataDoubleEscapeEndState;

                // Emit a U+002F SOLIDUS character token.
				vec_with_token(Token::CharToken('/'))
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
