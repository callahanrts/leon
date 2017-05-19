use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_less_than_sign_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '!' => {
                // Append the current input character to the comment token’s data.
                self.append_to_comment_token('!');

                // Switch to the comment less-than sign bang state.
                self.state = State::CommentLessThanSignBangState;
                Vec::new()
            },
            '<' => {
                // Append the current input character to the comment token’s data.
                self.append_to_comment_token('<');
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
