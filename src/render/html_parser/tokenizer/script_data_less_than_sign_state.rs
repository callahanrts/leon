use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_less_than_sign_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '/' | '\u{002F}' => {
                // Set the temporary buffer to the empty string.
                self.tmp_buffer = String::new();

                // Switch to the script data end tag open state.
                self.state = State::ScriptDataEndTagOpenState;
                Vec::new()
            },
            '!' | '\u{0021}' => {
                // Switch to the script data escape start state.
                self.state = State::ScriptDataEscapeStartState;

                // Emit a U+003C LESS-THAN SIGN character token
                // and a U+0021 EXCLAMATION MARK character token.
                let mut tokens = Vec::new();
                tokens.push(Token::CharToken('<'));
                tokens.push(Token::CharToken('!'));
                tokens
            },
            _ => {
                // Reconsume in the script data state.
                self.reconsume_char();
                self.state = State::ScriptDataState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            }
        }
    }

}
