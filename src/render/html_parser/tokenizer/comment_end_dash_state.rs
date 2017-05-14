use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_end_dash_state(&mut self) -> Vec<Token> {
        if self.eof() {
            // Emit the comment. Emit an end-of-file token.
            let mut tokens = Vec::new();
            tokens.push(self.current_token());
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '-' | '\u{0021}' => {
                // Switch to the comment end state
                self.state = State::CommentEndState;
                Vec::new()
            },
            _ => {
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token’s data.
                self.append_to_comment_token('-');

                // Reconsume in the comment state.
                self.reconsume_char();
                self.state = State::CommentState;
                Vec::new()
            }
        }
    }

}
