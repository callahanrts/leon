use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_escaped_end_tag_open_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            x if is_ascii(x) => {
                // Create a new end tag token.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

                // Reconsume in the script data escaped end tag name state.  (Don’t emit
                // the token yet; further details will be filled in before it is emitted.)
                self.reconsume_char();
                self.state = State::ScriptDataEscapedEndTagNameState;
                Vec::new()
            },
            _ => {
                // Reconsume in the script data escaped state.
                self.reconsume_char();
                self.state = State::ScriptDataEscapedState;

                // Emit a U+003C LESS-THAN SIGN character token
                // and a U+002F SOLIDUS character token.
                vec![Token::CharToken('<'), Token::CharToken('/')]
            }
        }
    }

}
