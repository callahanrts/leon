use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_start_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '-' | '\u{002D}' => {
                // Switch to the comment start dash state.
                self.state = State::CommentStartDashState;
                Vec::new()
            },
            '>' | '\u{002E}' => {
                // Parse error.
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the comment token.
                vec_with_token(self.current_token())
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



