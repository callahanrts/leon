use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_cdata_section_bracket_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            ']' => {
                // Switch to the CDATA section end state.
                self.state = State::CDataSectionEndState;
                Vec::new()
            },
            _ => {
                // Reconsume in the CDATA section state
                self.reconsume_char();
                self.state = State::CDataSectionState;

                // Emit a U+005D RIGHT SQUARE BRACKET character token.
                vec_with_token(Token::CharToken(']'))
            }
        }
    }
}
