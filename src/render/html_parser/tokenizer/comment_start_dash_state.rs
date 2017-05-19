use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_start_dash_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();
            // Parse error.
            // Emit the comment token.
            tokens.push(self.current_token());

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '-' => {
                // Switch to the comment end state
                self.state = State::CommentEndState;
                Vec::new()
            },
            '>' => {
                // Parse error.
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the comment token.
                vec_with_token(self.current_token())
            },
            c => {
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token’s data.
                self.append_to_comment_token(c);

                // Reconsume in the comment state.
                self.reconsume_char();
                self.state = State::CommentState;
                Vec::new()
            }
        }
    }

}

