use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_rawtext_end_tag_name_state(&mut self) -> Vec<Token> {
        self.consume_end_tag_name_state(State::RawtextState)
    }

}
