use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_escape_start_dash_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '-' | '\u{002D}' => {
                // Switch to the script data escape start dash state.
                self.state = State::ScriptDataEscapedDashDashState;

                // Emit a U+002D HYPHEN-MINUS character token.
                vec_with_token(Token::CharToken('-'))
            },
            _ => {
                // Reconsume in the script data state.
                self.state = State::ScriptDataState;
                self.reconsume_char();
                Vec::new()
            }
        }
    }

}
