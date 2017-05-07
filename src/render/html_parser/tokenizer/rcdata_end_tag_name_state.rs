use super::*;

impl<'a> Tokenizer<'a> {

    pub fn consume_rcdata_end_tag_name_state(&mut self) -> Vec<Token> {
        self.consume_end_tag_name_state(State::RCDataState)
    }
}
