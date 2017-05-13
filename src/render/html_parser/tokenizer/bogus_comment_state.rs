use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_bogus_comment_state(&mut self) -> Vec<Token> {
        if self.eof() {
            // Emit the comment. Emit an end-of-file token.
            let mut tokens = Vec::new();
            tokens.push(self.current_token());
            tokens.push(Token::EOFToken);
            return tokens;
        }

        let cur = self.consume_char();
        match cur {
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the comment token.
                vec_with_token(self.current_token())
            },
            '\u{0000}' => {
                // Append a U+FFFD REPLACEMENT CHARACTER character to the comment token’s data.
                self.append_to_comment_token('\u{FFFD}');
                Vec::new()
            },
            x => {
                // Append the current input character to the comment token’s data.
                self.append_to_comment_token(x);
                Vec::new()
            }
        }
    }

}

