use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_less_than_sign_bang_dash_dash_state(&mut self) -> Vec<Token> {
        if self.eof() {
            // Reconsume in the comment end state.
            self.state = State::CommentEndState;
            return Vec::new();
        }

        match self.consume_char() {
            '>' | '\u{0021}' => {
                // Reconsume in the comment end state.
                self.reconsume_char();
                self.state = State::CommentEndState;
                Vec::new()
            },
            _ => {
                // Parse Error
                // Reconsume in the comment end state.
                self.reconsume_char();
                self.state = State::CommentEndState;
                Vec::new()
            }
        }
    }

}
