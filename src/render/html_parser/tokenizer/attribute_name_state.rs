use super::*;
// http://w3c.github.io/html/syntax.html#attribute-name-state
// When the user agent leaves the attribute name state (and before emitting the tag token,
// if appropriate), the complete attribute’s name must be compared to the other attributes
// on the same token; if there is already an attribute on the token with the exact same
// name, then this is a parse error and the new attribute must be removed from the token.
//
// NOTE:
// If an attribute is so removed from a token, it, and the value that gets associated
// with it, if any, are never subsequently used by the parser, and are therefore
// effectively discarded. Removing the attribute in this way does not change its status
// as the "current attribute" for the purposes of the tokenizer, however.
impl<'a> Tokenizer<'a> {

    pub fn consume_attr_name_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            '\t' | '\u{000A}' | '\u{000C}' | ' ' | '/' | '>' => {
                // Reconsume in the after attribute name state.
                self.reconsume_char();
                self.state = State::AfterAttrNameState;
                Vec::new()
            },
            '=' => {
                // Switch to the before attribute value state.
                self.state = State::BeforeAttrNameState;
                Vec::new()
            },
            x if is_upper_ascii(x) => {
                // Append the lowercase version of the current input character (add
                // 0x0020 to the character’s code point) to the current attribute’s name.
                self.edit_current_tag(|tag| tag.append_attr_name(lowercase_char(x)));
                Vec::new()
            },
            '\u{0000}' => {
                // Parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute’s name.
                self.edit_current_tag(|tag| tag.append_attr_name('\u{FFFD}'));
                Vec::new()
            },
            '"' | '\'' | '<' => {
                // Parse error. Treat it as per the "anything else" entry below.
                self.edit_current_tag(|tag| tag.append_attr_name('"'));
                Vec::new()
            },
            x => {
                // Append the current input character to the current attribute’s name.
                self.edit_current_tag(|tag| tag.append_attr_name(x));
                Vec::new()
            }
        }
    }

}


