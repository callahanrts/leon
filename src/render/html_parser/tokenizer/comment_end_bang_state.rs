use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_end_bang_state(&mut self) -> Vec<Token> {
        if self.eof() {
            // Emit the comment. Emit an end-of-file token.
            let mut tokens = Vec::new();
            tokens.push(self.current_token());
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '-' | '\u{0021}' => {
                // Append two U+002D HYPHEN-MINUS characters (-) and a U+0021
                // EXCLAMATION MARK character (!) to the comment token’s data.
                self.append_to_comment_token('-');
                self.append_to_comment_token('-');
                self.append_to_comment_token('!');

                // Switch to the comment end dash state.
                self.state = State::CommentEndDashState;
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Parse error.
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the comment token.
                vec_with_token(self.current_token())
            },
            _ => {
                // Append two U+002D HYPHEN-MINUS characters (-) and a U+0021
                // EXCLAMATION MARK character (!) to the comment token’s data.
                self.append_to_comment_token('-');
                self.append_to_comment_token('-');
                self.append_to_comment_token('!');

                // Reconsume in the comment state.
                self.reconsume_char();
                self.state = State::CommentState;
                Vec::new()
            }
        }
    }

}
