// http://w3c.github.io/html/syntax.html#tokenization
#[cfg(test)]
mod test;

#[derive(Clone)]
enum Token {
    Empty,
    DoctypeToken(DoctypeData),
    StartTagToken(Tag),
    EndTagToken(Tag),
    CommentToken(String),
    EOFToken,
    CharToken(char),
}

#[derive(Clone)]
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
    RCDataEndTagOpenState,
    RCDataEndTagNameState,

    RawtextState,
    RawtextLessThanSignState,
    RawtextEndTagOpenState,
    RawtextEndTagNameState,

    ScriptDataState,
    ScriptDataLessThanSignState,
    // ScriptDataEscapedDashDashState,
    // ScriptDataEscapeStartSlashState,

    PlaintextState,
    MarkupDeclarationOpenState,
}

struct Tokenizer<'a> {
    pos: usize,
    input: &'a str,
    state: State,
    return_state: State,
    current_token: Option<Token>,
    tokens: Vec<Token>,
    tmp_buffer: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer {
        Tokenizer {
            pos: 0,
            input: input,
            state: State::DataState,
            return_state: State::DataState,
            current_token: None,
            tokens: Vec::new(),
            tmp_buffer: String::new(),
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
        match self.state {
            State::DataState => self.consume_data_state(),
            State::RCDataState => self.consume_rcdata_state(),
            State::RawtextState => self.consume_rawtext_state(),
            State::ScriptDataState => self.consume_script_data_state(),
            State::PlaintextState => self.consume_plaintext_state(),
            State::TagOpenState => self.consume_tag_open_state(),
            State::EndTagOpenState => self.consume_end_tag_open_state(),
            State::TagNameState => self.consume_tag_name_state(),
            State::RCDataLessThanSignState => self.consume_rcdata_less_than_sign_state(),
            State::RCDataEndTagOpenState => self.consume_rcdata_end_tag_open_state(),
            State::RCDataEndTagNameState => self.consume_rcdata_end_tag_name_state(),
            State::RawtextLessThanSignState => self.consume_rawtext_less_than_sign_state(),
            State::RawtextEndTagOpenState => self.consume_rawtext_end_tag_open_state(),
            State::RawtextEndTagNameState => self.consume_rawtext_end_tag_name_state(),

            // TODO: Cover all states instead of using a catchall
            _ => Vec::new()
        }
    }

    //
    // Tokenizer States
    //

    fn consume_data_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '&' | '\u{0026}' => {
                // Set the return state to the data state
                self.return_state = State::DataState;
                // Switch to the character reference state
                self.state = State::CharReferenceState;
                return Vec::new();
            }
            '<' | '\u{003C}' => {
                // Switch to the tag open state. We're reading an open tag
                self.state = State::TagOpenState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Return character in a CharToken
                return vec_with_token(Token::CharToken('\u{0000}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }

    fn consume_rcdata_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '&' | '\u{0026}' => {
                // Set the return state to the RCDATA state.
                self.return_state = State::RCDataState;

                // Switch to the character reference state.
                self.state = State::CharReferenceState;
                return Vec::new();
            }
            '<' | '\u{003C}' => {
                self.state = State::RCDataLessThanSignState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return vec_with_token(Token::CharToken('\u{FFFD}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }

    fn consume_rawtext_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '<' | '\u{003C}' => {
                // Switch to RawtextLessThanSignState
                self.state = State::RawtextLessThanSignState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return vec_with_token(Token::CharToken('\u{FFFD}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }

    fn consume_script_data_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            '<' | '\u{003C}' => {
                // Switch to RawtextLessThanSignState
                self.state = State::ScriptDataLessThanSignState;
                return Vec::new();
            }
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return vec_with_token(Token::CharToken('\u{FFFD}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        };
    }

    fn consume_plaintext_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        // Consume the next input Char
        match self.consume_char() {
            // Null character
            '\u{0000}' => {
                // TODO: Parse error
                // Emit a U+FFFD REPLACEMENT CHARACTER character token.
                return vec_with_token(Token::CharToken('\u{FFFD}'));
            }
            cur => {
                // For everything else, return the character in a CharToken
                return vec_with_token(Token::CharToken(cur));
            }
        }
    }

    fn consume_tag_open_state(&mut self) -> Vec<Token> {
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

    fn consume_end_tag_open_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            let mut tokens = Vec::new();
            // TODO: Parse error.
            // Emit a U+003C LESS-THAN SIGN character token,
            tokens.push(Token::CharToken('<'));
            // a U+002F SOLIDUS character token
            tokens.push(Token::CharToken('/'));
            // and an end-of-file token.
            tokens.push(Token::EOFToken);
            return tokens;
        }

        match self.consume_char() {
            x if is_ascii(x) => {
                // Reconsume in the tag name state.
                self.reconsume_char();
                self.state = State::TagNameState;

                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));
                return Vec::new();
            },
            '>' | '\u{003E}' => {
                // TODO: Parse error.
                // Switch to the data state.
                self.state = State::DataState;
                return Vec::new();
            },
            _ => {
                // TODO: Parse error.
                // Reconsume in the bogus comment state.
                self.reconsume_char();
                self.state = State::BogusCommentState;

                // Create a comment token whose data is the empty string.
                self.current_token = Some(Token::CommentToken(String::from("")));
                return Vec::new();
            }
        }
    }

    fn consume_tag_name_state(&mut self) -> Vec<Token> {
        // Return an EOF token if there are no more characters. Do this before we try to
        // consume another character.
        if self.eof() {
            return vec_with_token(Token::EOFToken);
        }

        let cur = self.consume_char();
        match cur {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // Switch to the before attribute name state.
                self.state = State::BeforeAttrNameState;
                return Vec::new();
            },
            '/' | '\u{002F}' => {
                // Switch to the self-closing start tag state.
                self.state = State::SelfClosingStartTagState;
                return Vec::new();
            },
            '>' | '\u{003E}' => {
                // Switch to the data state.
                self.state = State::DataState;

                // Emit the current tag token.
                return vec_with_token(self.current_token());
            },
            'A' ... 'Z' | '\u{0041}' ... '\u{005A}' => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current tag token’s tag name.
                self.append_char_to_tag_name(lowercase_char(cur));
                return Vec::new();
            },
            '\u{0000}' => {
                // TODO: Parse error.
                // Append a U+FFFD REPLACEMENT CHARACTER character to the current tag token’s tag name.
                self.append_char_to_tag_name('\u{FFFD}');
                return Vec::new()
            },
            _ => {
                // Append the current input character to the current tag token’s tag name.
                self.append_char_to_tag_name(cur);
                return Vec::new()
            }

        }
    }

    fn consume_rcdata_less_than_sign_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            '/' | '\u{002F}' => {
                // Set the temporary buffer to the empty string.
                // Switch to the RCDATA end tag open state.
                self.state = State::RCDataEndTagOpenState;
                return Vec::new();
            }
            _ => {
                let mut tokens = Vec::new();
                // Reconsume in the RCDATA state.
                self.reconsume_char();
                self.state = State::RCDataState;

                // Emit a U+003C LESS-THAN SIGN character token.
                tokens.push(Token::CharToken('<'));
                return tokens;
            }
        }
    }

    fn consume_rcdata_end_tag_open_state(&mut self) -> Vec<Token> {
        let cur = self.consume_char();
        match cur {
            // ASCII Letter
            x if is_ascii(x) => {
                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

                // Reconsume in RCDATA end tag name state.
                self.reconsume_char();
                self.state = State::RCDataEndTagNameState;
                return Vec::new();
            }
            _ => {
                // Reconsume in the RCDATA state.
                self.reconsume_char();
                self.state = State::RCDataState;

                let mut tokens = Vec::new();
                // Emit a U+003C LESS-THAN SIGN character token
                tokens.push(Token::CharToken('<'));
                // and a U+002F SOLIDUS character token.
                tokens.push(Token::CharToken('/'));
                return tokens;
            }
        };
    }

    fn consume_rcdata_end_tag_name_state(&mut self) -> Vec<Token> {
        self.consume_end_tag_name_state(State::RCDataState)
    }

    // This is the anything else portion of an end tag name state.
    // TODO: Name this more descriptively
    fn handle_end_tag_name(&mut self, new_state: State) -> Vec<Token> {
        let mut tokens = Vec::new();
        // Emit a U+003C LESS-THAN SIGN character token,
        tokens.push(Token::CharToken('<'));

        // a U+002F SOLIDUS character token,
        tokens.push(Token::CharToken('/'));

        // and a character token for each of the characters in the temporary
        // buffer (in the order they were added to the buffer).
        for c in self.tmp_buffer.chars() {
            tokens.push(Token::CharToken(c));
        }

        // Reconsume in the RCDATA state.
        self.reconsume_char();
        self.state = new_state;
        return tokens;
    }

    fn consume_rawtext_less_than_sign_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            '/' | '\u{002F}' => {
                // Set the temporary buffer to the empty string.
                self.tmp_buffer = String::new();

                // Switch to the RAWTEXT end tag open state.
                self.state = State::RawtextEndTagOpenState;
                Vec::new()
            },
            _ => {
                // Reconsume in the RAWTEXT state.
                self.reconsume_char();
                self.state = State::RawtextState;

                // Emit a U+003C LESS-THAN SIGN character token.
                vec_with_token(Token::CharToken('<'))
            }
        }
    }

    fn consume_rawtext_end_tag_open_state(&mut self) -> Vec<Token> {
        match self.consume_char() {
            x if is_ascii(x) => {
                // Create a new end tag token, set its tag name to the empty string.
                self.current_token = Some(Token::EndTagToken(Tag::new(String::new())));

                // Reconsume in the RAWTEXT end tag name state.
                self.reconsume_char();
                self.state = State::RawtextEndTagNameState;
                Vec::new()
            }
            _ => {
                // Reconsume in the RAWTEXT state.
                self.reconsume_char();
                self.state = State::RawtextState;

                let mut tokens = Vec::new();
                // Emit a U+003C LESS-THAN SIGN character token
                // and a U+002F SOLIDUS character token.
                tokens.push(Token::CharToken('<'));
                tokens.push(Token::CharToken('/'));
                tokens
            }
        }
    }

    fn consume_rawtext_end_tag_name_state(&mut self) -> Vec<Token> {
        self.consume_end_tag_name_state(State::RawtextState)
    }


    fn consume_end_tag_name_state(&mut self, new_state: State) -> Vec<Token> {
        match self.consume_char() {
            '\t' | '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' | '\u{0020}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the before attribute name state.
                    self.state = State::BeforeAttrNameState;
                    Vec::new()
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            '/' | '\u{002f}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the self-closing start tag state.
                    self.state = State::SelfClosingStartTagState;
                    Vec::new()
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            '>' | '\u{003E}' => {
                // If the current end tag token is an appropriate end tag token,
                if self.is_appropriate_end_tag_token() {
                    // then switch to the data state and emit the current tag token.
                    self.state = State::DataState;
                    vec_with_token(self.current_token())
                } else {
                    // Otherwise, treat it as per the "anything else" entry below.
                    self.handle_end_tag_name(new_state)
                }
            },
            x if is_upper_ascii(x) => {
                // Append the lowercase version of the current input character (add 0x0020
                // to the character’s code point) to the current tag token’s tag name.
                self.append_char_to_tag_name(lowercase_char(x));

                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push(x);
                Vec::new()
            },
            x if is_lower_ascii(x) => {
                // Append the current input character to the current tag token’s tag name.
                self.append_char_to_tag_name(x);

                // Append the current input character to the temporary buffer.
                self.tmp_buffer.push(x);
                Vec::new()
            },
            _ => {
                self.handle_end_tag_name(new_state)
            }
        }
    }

    // fn consume_script_data_escape_start_slash_state(&mut self) -> Option<Token> {
    //     let cur = self.consume_char();
    //     match cur {
    //         '-' | '\u{002D}' => {
    //             self.state = State::ScriptDataEscapedDashDashState;
    //             return Some(Token::CharToken('-'));
    //         }
    //         _ => {
    //             self.reconsume_char();
    //             self.state = State::ScriptDataState;
    //         }
    //     }
    //     return None;
    // }

    //
    // Helpers
    //
    fn append_char_to_tag_name(&mut self, letter: char) {
        match self.current_token() {
            Token::StartTagToken(mut tag) => {
                tag.append_name(letter);
                self.current_token = Some(Token::StartTagToken(tag))
            },
            Token::EndTagToken(mut tag) => {
                tag.append_name(letter);
                self.current_token = Some(Token::EndTagToken(tag))
            }
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
                    },
                    Token::EndTagToken(ref mut tag) => {
                        return Token::EndTagToken(tag.clone());
                    }
                    _ => panic!("Unimplemented token")
                }
            }
            None => panic!("No token found")
        }
    }

    // An appropriate end tag token is an end tag token whose tag name matches the tag
    // name of the last start tag to have been emitted from this tokenizer, if any. If
    // no start tag has been emitted from this tokenizer, then no end tag token is
    // appropriate.
    // http://w3c.github.io/html/syntax.html#appropriate-end-tag-token
    fn is_appropriate_end_tag_token(&mut self) -> bool {
        let end_tag = self.current_token();
        let mut tokens = self.tokens.clone();
        tokens.reverse();
        let name = match end_tag {
            Token::EndTagToken(tag) => tag.name,
            _ => String::from("")
        };

        for token in tokens {
            match token {
                Token::StartTagToken(tag) => {
                    return name == tag.name;
                }
                _ => {}
            }
        }
        return false;
    }

}

fn is_upper_ascii(c: char) -> bool {
    match c {
        'A' ... 'Z' | '\u{0041}' ... '\u{005A}' => true,
        _ => false,
    }
}

fn is_lower_ascii(c: char) -> bool {
    match c {
        'a' ... 'z' | '\u{0061}' ... '\u{007A}' => true,
        _ => false
    }
}

fn lowercase_char(c: char) -> char {
    c.to_lowercase().collect::<Vec<_>>()[0]
}

fn is_ascii(c: char) -> bool {
    is_upper_ascii(c) || is_lower_ascii(c)
}

fn vec_with_token(t: Token) -> Vec<Token> {
    let mut tokens = Vec::new();
    tokens.push(t);
    return tokens;
}

