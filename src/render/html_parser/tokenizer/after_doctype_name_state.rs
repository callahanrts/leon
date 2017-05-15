use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_after_doctype_name_state(&mut self) -> Vec<Token> {
        if self.eof() {
            let mut tokens = Vec::new();
            // Parse error.
            // Set the DOCTYPE token’s force-quirks flag to on.
            self.edit_doctype_token(|data| data.force_quirks = true);

            // Emit that DOCTYPE token.
            tokens.push(self.current_token());

            // Emit an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Ignore the character
                Vec::new()
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current DOCTYPE token.
                vec_with_token(self.current_token())
            },
            c => {
                self.reconsume_char();

                // If the six characters starting from the current input character are an
                // ASCII case-insensitive match for the word "PUBLIC",
                if self.starts_with_nocase("public") {
                    println!("PUBLIC");
                    // then consume those characters
                    for i in 0..6 {
                        self.consume_char();
                    }
                    // and switch to the after DOCTYPE public keyword state.
                    self.state = State::DOCTYPEPublicKeywordState;
                }
                // Otherwise, if the six characters starting from the current input
                // character are an ASCII case-insensitive match for the word "SYSTEM",
                else if self.starts_with_nocase("system") {
                    // then consume those characters
                    for i in 0..6 {
                        self.consume_char();
                    }
                    // and switch to the after DOCTYPE system keyword state.
                    self.state = State::DOCTYPESystemKeywordState;
                }

                // Otherwise,
                else {
                    self.consume_char();

                    // this is a parse error.
                    // Set the DOCTYPE token’s force-quirks flag to on.
                    self.edit_doctype_token(|data| data.force_quirks = true);

                    // Switch to the bogus DOCTYPE state.
                    self.state = State::BogusDOCTYPEState;
                }
                Vec::new()
            }
        }
    }
}
