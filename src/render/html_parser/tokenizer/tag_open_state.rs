use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_tag_open_state(&mut self) -> Vec<Token> {
        // Consume the next input Char
        match self.consume_char() {
            '!' | '\u{0021}' => {
                // Switch to MarkupDeclarationOpenState
                self.state = State::MarkupDeclarationOpenState;
                return Vec::new();
            },
            '/' | '\u{002F}' => {
                self.state = State::EndTagOpenState;
                return Vec::new();
            },
            // ASCII Letter
            x if is_ascii(x) => {
                // Reconsume the character and move to the TagNameState
                self.reconsume_char();
                self.state = State::TagNameState;
                self.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
                return Vec::new();
            },
            '?' | '\u{003F}' => {
                // TODO: Parse Error
                // Reconsume character and enter the BogusCommentState
                self.reconsume_char();
                self.state = State::BogusCommentState;

                // Create a comment token who's data is an emtpy string
                self.current_token = Some(Token::CommentToken(String::new()));
                return Vec::new();
            },
            _ => {
                // TODO: Parse Error
                // Reconsume character and enter the DataState
                self.reconsume_char();
                self.state = State::DataState;

                // For everything else, return a '<' in a CharToken
                return vec_with_token(Token::CharToken('<'));
            },
        }
    }

}
