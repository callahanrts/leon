use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_script_data_end_tag_open_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            x if is_ascii(x) => {
                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

                // Reconsume in the script data end tag name state.
                self.reconsume_char();
                self.state = State::ScriptDataEndTagNameState;
                Vec::new()
            },
            _ => {
                // Reconsume in the script data state.
                self.reconsume_char();
                self.state = State::ScriptDataState;

                // Emit a U+003C LESS-THAN SIGN character token
                // and a U+002F SOLIDUS character token.
                let mut tokens = Vec::new();
                tokens.push(Token::CharToken('<'));
                tokens.push(Token::CharToken('/'));
                tokens
            }
        }
    }

}
