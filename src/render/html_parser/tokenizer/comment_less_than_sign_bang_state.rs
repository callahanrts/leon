use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_less_than_sign_bang_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '!' => {
                // Switch to the comment less-than sign bang dash state.
                self.state = State::CommentLessThanSignBangDashState;
                Vec::new()
            },
            _ => {
                // Reconsume in the comment state.
                self.reconsume_char();
                self.state = State::CommentState;
                Vec::new()
            }
        }
    }

}

