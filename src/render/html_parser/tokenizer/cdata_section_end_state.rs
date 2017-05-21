use super::*;
impl<'a> Tokenizer<'a> {

    pub fn consume_cdata_section_end_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            ']' => {
                // Emit a U+005D RIGHT SQUARE BRACKET character token.
                vec_with_token(Token::CharToken(']'))
            },
            '>' => {
                // Switch to the data state.
                self.state = State::DataState;
                Vec::new()
            }
            _ => {
                // Reconsume in the CDATA section state
                self.reconsume_char();
                self.state = State::CDataSectionState;

                // Emit two U+005D RIGHT SQUARE BRACKET character tokens.
                let mut tokens = Vec::new();
                tokens.push(Token::CharToken(']'));
                tokens.push(Token::CharToken(']'));
                return tokens
            }
        }
    }
}
