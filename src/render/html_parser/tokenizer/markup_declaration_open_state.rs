use super::*;
impl<'a> Tokenizer<'a> {

    // http://w3c.github.io/html/syntax.html#markup-declaration-open-state

    pub fn consume_markup_declaration_open_state(&mut self) -> Vec<Token> {
        // If the next two characters are both U+002D HYPHEN-MINUS characters (-),
        if self.starts_with("--") {
            // consume those two characters,
            self.consume_char();
            self.consume_char();

            // create a comment token whose data is the empty string,
            self.current_token = Some(Token::CommentToken(String::new()));

            // and switch to the comment start state.
            self.state = State::CommentStartState;
        }

        // Otherwise, if the next seven characters are an ASCII case-insensitive match
        // for the word "DOCTYPE",
        else if self.starts_with_nocase("doctype") {
            // then consume those characters
            for x in 0..7 {
                self.consume_char();
            }

            // and switch to the DOCTYPE state.
            self.state = State::DOCTYPEState;
        }

        // TODO: Finish this after pieces of the parsing algorithm have been completed
        // Otherwise, if there is an adjusted current node
        // and it is not an element in the HTML namespace
        // and the next seven characters are a case-sensitive match for the string "[CDATA["
        // (the five uppercase letters "CDATA" with a U+005B LEFT SQUARE BRACKET character before and after),
        // else if {
        //     // then consume those characters
        //     // and switch to the CDATA section state.
        //     self.state = CDATASectionState
        // }

        // Otherwise,
        else {
            // this is a parse error.
            // Create a comment token whose data is the empty string.
            self.current_token = Some(Token::CommentToken(String::new()));

            // Switch to the bogus comment state (don’t consume anything in the current state).
            self.state = State::BogusCommentState;
        }
        Vec::new()
    }

}


