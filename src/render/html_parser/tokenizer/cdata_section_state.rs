use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_cdata_section_state(&mut self) -> Vec<Token> {
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        match self.consume_char() {
            ']' => {
                // Switch to the CDATA section bracket state.
                self.state = State::CDataSectionBracketState;
                Vec::new()
            },
            c => {
                // Emit the current input character as a character token.
                vec_with_token(Token::CharToken(c))
            }
        }
    }
}
