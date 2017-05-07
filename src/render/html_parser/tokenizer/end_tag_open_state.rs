use super::*;

impl<'a> Tokenizer<'a> {

    // This generic-ish method is used for both rcdata end tag name state and rawtext end tag name state
    pub fn consume_end_tag_open_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            let mut tokens = Vec::new();
            // TODO: Parse error.
            // Emit a U+003C LESS-THAN SIGN character token,
            tokens.push(Token::CharToken('<'));
            // a U+002F SOLIDUS character token
            tokens.push(Token::CharToken('/'));
            // and an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            x if is_ascii(x) => {
                // Reconsume in the tag name state.
                self.reconsume_char();
                self.state = State::TagNameState;

                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));
                return Vec::new();
            },
            '>' | '\u{003E}' => {
                // TODO: Parse error.
                // Switch to the data state.
                self.state = State::DataState;
                return Vec::new();
            },
            _ => {
                // TODO: Parse error.
                // Reconsume in the bogus comment state.
                self.reconsume_char();
                self.state = State::BogusCommentState;

                // Create a comment token whose data is the empty string.
                self.current_token = Some(Token::CommentToken(String::from("")));
                return Vec::new();
            }
        }
    }

}
