// http://w3c.github.io/html/syntax.html#tokenization
#[cfg(test)]
mod test;

enum Token {
    Empty,
    DoctypeToken(DoctypeData),
    StartTagToken(Tag),
    EndTagToken(Tag),
    CommentToken(String),
    EOFToken,
    CharToken(char),
}

struct DoctypeData {
    name: String,
    public_identifier: String,
    system_identifier: String,
    force_quirks: bool,
}

impl DoctypeData {
    pub fn new(name: String) -> DoctypeData {
        DoctypeData{
            name: name,
            public_identifier: String::from(""),
            system_identifier: String::from(""),
            force_quirks: false,
        }
    }
}

#[derive(Clone)]
struct Tag {
    name: String,
    self_closing: bool,
    attributes: Vec<Attribute>,
}

impl Tag {
    pub fn new(name: String) -> Tag {
        Tag {
            name: name,
            self_closing: false,
            attributes: Vec::new(),
        }
    }
    pub fn append_name(&mut self, letter: char) {
        self.name.push(letter);
    }

    pub fn name(&mut self) -> String {
        self.name.clone()
    }

    pub fn self_closing(&mut self) -> bool {
        self.self_closing.clone()
    }

    pub fn attributes(&mut self) -> Vec<Attribute> {
        self.attributes.clone()
    }
}

#[derive(Clone)]
struct Attribute {
    name: String,
    value: String,
}

enum State {
    DataState,
    CharReferenceState,
    TagOpenState,
    EndTagOpenState,
    TagNameState,
    BogusCommentState,
    BeforeAttrNameState,
    SelfClosingStartTagState,

    // RC States
    RCDataState,
    RCDataLessThanSignState,

    RawtextState,
    RawtextLessThanSignState,

    ScriptDataState,
    ScriptDataLessThanSignState,

    PlaintextState,
    MarkupDeclarationOpenState,
}

struct Tokenizer<'a> {
    pos: usize,
    input: &'a str,
    state: State,
    return_state: State,
    current_token: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer {
        Tokenizer {
            pos: 0,
            input: input,
            state: State::DataState,
            return_state: State::DataState,
            current_token: None,
        }
    }

    //
    // Basic String scanning methods
    //

    // Read the current character without consuming it
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn nth_char(&self, offset: usize) -> char{
        return self.input[(self.pos + offset - 1)..].chars().next().unwrap();
    }

    // Do the next characters start with the given string?
    // NOTE: Starts with compares as lower case
    fn starts_with(&self, s: &str) -> bool {
        // return self.input[self.pos..].to_lowercase().starts_with(s);
        return self.input[self.pos..].starts_with(s);
    }

    // Return true if all input is consumed
    fn eof(&self) -> bool {
        return self.pos >= self.input.len();
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn reconsume_char(&mut self) {
        self.pos -= 1;
    }

    // Consume characters until test returns false
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    pub fn consume_token(&mut self) -> Vec<Token> {
        // Some states can emit more than one token
        let mut tokens = Vec::new();
        match self.state {
            State::DataState => {
                match self.consume_data_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::RCDataState => {
                match self.consume_rcdata_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::RawtextState => {
                match self.consume_rawtext_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::ScriptDataState => {
                match self.consume_script_data_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::PlaintextState => {
                match self.consume_plaintext_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::TagOpenState => {
                match self.consume_tag_open_state() {
                    Some(token) => tokens.push(token),
                    None => {},
                }
            },
            State::EndTagOpenState => {
                match self.consume_end_tag_open_state() {
                    Some(ts) => tokens.extend(ts),
                    None => {},
                }
            },
            State::TagNameState => {
                match self.consume_tag_name_state() {
                    Some(token) => {
                        // tokens.push(token.clone());
                    },
                    None => {},
                }
            }

            // TODO: Cover all states instead of using a catchall
            _ => {}
        }

        return tokens;
    }

    //
    // Tokenizer States
    //

    fn consume_data_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            '&' | '\u{0026}' => {
                // Set the return state to the data state
                self.return_state = State::DataState;
                // Switch to the character reference state
                self.state = State::CharReferenceState;
            }
            '<' | '\u{003C}' => {
                // Switch to the tag open state. We're reading an open tag
                self.state = State::TagOpenState;
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Return character in a CharToken
                return Some(Token::CharToken(cur));
            }
            _ => {
                // For everything else, return the character in a CharToken
                return Some(Token::CharToken(cur));
            }
        }
        return None;
    }

    fn consume_rcdata_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            '&' | '\u{0026}' => {
                // Set the return state to the RCDATA state.
                self.return_state = State::RCDataState;

                // Switch to the character reference state.
                self.state = State::CharReferenceState;
            }
            '<' | '\u{003C}' => {
                self.state = State::RCDataLessThanSignState;
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return Some(Token::CharToken('\u{FFFD}'));
            }
            _ => {
                // For everything else, return the character in a CharToken
                return Some(Token::CharToken(cur));
            }
        }

        return None;
    }

    fn consume_rawtext_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            '<' | '\u{003C}' => {
                // Switch to RawtextLessThanSignState
                self.state = State::RawtextLessThanSignState;
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return Some(Token::CharToken('\u{FFFD}'));
            }
            _ => {
                // For everything else, return the character in a CharToken
                return Some(Token::CharToken(cur));
            }
        }
        return None;
    }

    fn consume_script_data_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            '<' | '\u{003C}' => {
                // Switch to RawtextLessThanSignState
                self.state = State::ScriptDataLessThanSignState;
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return Some(Token::CharToken('\u{FFFD}'));
            }
            _ => {
                // For everything else, return the character in a CharToken
                return Some(Token::CharToken(cur));
            }
        }
        return None;
    }

    fn consume_plaintext_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return Some(Token::CharToken('\u{FFFD}'));
            }
            _ => {
                // For everything else, return the character in a CharToken
                return Some(Token::CharToken(cur));
            }
        }
        return None;
    }

    fn consume_tag_open_state(&mut self) -> Option<Token> {
        // Consume the next input Char
        let cur = self.consume_char();
        match cur {
            '!' | '\u{0021}' => {
                // Switch to MarkupDeclarationOpenState
                self.state = State::MarkupDeclarationOpenState;
            },
            '/' | '\u{002F}' => {
                self.state = State::EndTagOpenState;
            },
            // ASCII Letter
            x if is_ascii(x) => {
                // Reconsume the character and move to the TagNameState
                self.reconsume_char();
                self.state = State::TagNameState;
                self.current_token = Some(Token::StartTagToken(Tag::new(String::new())));
            },
            '?' | '\u{003F}' => {
                // TODO: Parse Error
                // Reconsume character and enter the BogusCommentState
                self.reconsume_char();
                self.state = State::BogusCommentState;

                // Create a comment token who's data is an emtpy string
                self.current_token = Some(Token::CommentToken(String::new()));
            },
            _ => {
                // TODO: Parse Error
                // Reconsume character and enter the DataState
                self.reconsume_char();
                self.state = State::DataState;

                // For everything else, return a '<' in a CharToken
                return Some(Token::CharToken('<'));
            },
        }

        return None;
    }

    fn consume_end_tag_open_state(&mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            // TODO: Parse error.
            // Emit a U+003C LESS-THAN SIGN character token,
            tokens.push(Token::CharToken('<'));
            // a U+002F SOLIDUS character token
            tokens.push(Token::CharToken('/'));
            // and an end-of-file token.
            tokens.push(Token::EOFToken);
            return Some(tokens);
        }

        let cur = self.consume_char();
        match cur {
            x if is_ascii(x) => {
                // Reconsume in the tag name state.
                self.reconsume_char();
                self.state = State::TagNameState;

                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));
            },
            '>' | '\u{003E}' => {
                // TODO: Parse error.
                // Switch to the data state.
                self.state = State::DataState;
            },
            _ => {
                // TODO: Parse error.
                // Reconsume in the bogus comment state.
                self.reconsume_char();
                self.state = State::BogusCommentState;

                // Create a comment token whose data is the empty string.
                self.current_token = Some(Token::CommentToken(String::from("")));
            }
        }
        return None;
    }

    fn consume_tag_name_state(&mut self) -> Option<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return Some(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the before attribute name state.
                self.state = State::BeforeAttrNameState;
            },
            '/' | '\u{002F}' => {
                // Switch to the self-closing start tag state.
                self.state = State::SelfClosingStartTagState;
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current tag token.
                return Some(self.current_token());
            },
            'A' ... 'Z' | '\u{0041}' ... '\u{005A}' => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current tag token’s tag name.
                self.append_char_to_tag_name(cur.to_lowercase().collect::<Vec<_>>()[0]);
            },
            '\u{0000}' => {
                // TODO: Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the current tag token’s tag name.
                self.append_char_to_tag_name('\u{FFFD}');
            },
            _ => {
                // Append the current input character to the current tag token’s tag name.
                self.append_char_to_tag_name(cur);
            }

        }

        return None;
    }

    //
    // Helpers
    //
    fn append_char_to_tag_name(&mut self, letter: char) {
        match self.current_token() {
            Token::StartTagToken(mut tag) => {
                tag.append_name(letter);
                self.current_token = Some(Token::StartTagToken(tag))
            },
            _ => {
                panic!("Unimplemented token");
            }
        };
    }

    // I fought the compiler a lot with this one and append_chart_to_tag_name.
    // It's likely I'm missing something
    // TODO: Come back to this function in the future and make it better.
    fn current_token(&mut self) -> Token {
        match self.current_token {
            Some(ref mut t) => {
                match *t {
                    Token::StartTagToken(ref mut tag) => {
                        return Token::StartTagToken(tag.clone());
                    }
                    _ => panic!("Unimplemented token")
                }
            }
            None => panic!("No token found")
        }
    }

}


fn is_ascii(c: char) -> bool {
    match c {
        'A' ... 'Z' | 'a' ... 'z' | '\u{0041}' ... '\u{005A}' | '\u{0061}' ... '\u{007A}' => true,
        _ => false,
    }
}
