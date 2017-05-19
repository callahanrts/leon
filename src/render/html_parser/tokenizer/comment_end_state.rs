use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_end_state(&mut self) -> Vec<Token> {
        if self.eof() {
            // Emit the comment. Emit an end-of-file token.
            let mut tokens = Vec::new();
            tokens.push(self.current_token());
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '>' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the comment token.
                vec_with_token(self.current_token())
            },
            '!' => {
                // Switch to the comment end bang state.
                self.state = State::CommentEndBangState;
                Vec::new()
            },
            '-' => {
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token’s data.
                self.append_to_comment_token('-');
                Vec::new()
            },
            _ => {
                // Append two U+002D HYPHEN-MINUS characters (-) to the comment token’s data.
                self.append_to_comment_token('-');
                self.append_to_comment_token('-');

                // Reconsume in the comment state.
                self.reconsume_char();
                self.state = State::CommentState;
                Vec::new()
            }
        }
    }

}
