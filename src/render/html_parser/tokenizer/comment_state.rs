use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_comment_state(&mut self) -> Vec<Token> {
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
            '<' | '\u{003C}' => {
                // Append the current input character to the comment token’s data.
                self.append_to_comment_token('<');

                // Switch to the comment less-than sign state.
                self.state = State::CommentLessThanSignState;
                Vec::new()
            },
            '-' | '\u{002D}' => {
                // Switch to the comment end dash state
                self.state = State::CommentEndDashState;
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the comment
                // token’s data.
                self.append_to_comment_token('\u{FFFD}');

                Vec::new()
            },
            c => {
                // Append the current input character to the comment token’s data.
                self.append_to_comment_token(c);
                Vec::new()
            }
        }
    }

}


