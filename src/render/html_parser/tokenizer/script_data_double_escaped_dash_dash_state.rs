use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_double_escaped_dash_dash_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            '-' | '\u{002D}' => {
                // Emit a U+002D HYPHEN-MINUS character token.
                vec_with_token(Token::CharToken('-'))
            },
            '<' | '\u{003C}' => {
                // Switch to the script data double escaped less-than sign state.
                self.state = State::ScriptDataDoubleEscapedLessThanSignState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            },
            '\u{0000}' => {
                // Parse error. Switch to the script data double escaped state.
                self.state = State::ScriptDataDoubleEscapedState;

                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                vec_with_token(Token::CharToken('\u{FFFD}'))
            },
            c => {
                // Switch to the script data double escaped state.
                self.state = State::ScriptDataDoubleEscapedState;

                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(c))
            }
        }
    }
}