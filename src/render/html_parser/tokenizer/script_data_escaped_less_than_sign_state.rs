use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_escaped_less_than_sign_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '/' | '\u{002F}' => {
                // Set the temporary buffer to the empty string.
                self.tmp_buffer = String::new();

                // Switch to the script data escaped end tag open state.
                self.state = State::ScriptDataEscapedEndTagOpenState;
                Vec::new()
            },
            x if is_ascii(x) => {
                // Set the temporary buffer to the empty string.
                self.tmp_buffer = String::new();

                // Reconsume in the script data double escape start state.
                self.reconsume_char();
                self.state = State::ScriptDataDoubleEscapeStartState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            },
            _ => {
                // Reconsume in the script data escaped state.
                self.reconsume_char();
                self.state = State::ScriptDataEscapedState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            }
        }
    }

}
