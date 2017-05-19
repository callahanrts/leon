use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_escaped_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            '-' => {
                // Switch to the script data escaped dash state.
                self.state = State::ScriptDataEscapedDashState;

                // Emit a U+002D HYPHEN-MINUS character token.
                vec_with_token(Token::CharToken('-'))
            },
            '<' => {
                // Switch to the script data escaped less-than sign state.
                self.state = State::ScriptDataEscapedLessThanSignState;
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error.
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                vec_with_token(Token::CharToken('\u{FFFD}'))
            },
            c => {
                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(c))
            }
        }
    }

}
