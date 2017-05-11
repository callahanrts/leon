// http://w3c.github.io/html/syntax.html#tokenization
#[cfg(test)]
mod test;

// State modules
mod before_attribute_name_state;
mod data_state;
mod end_tag_name_state;
mod end_tag_open_state;
mod plaintext_state;
mod rawtext_end_tag_name_state;
mod rawtext_end_tag_open_state;
mod rawtext_less_than_sign_state;
mod rawtext_state;
mod rcdata_end_tag_name_state;
mod rcdata_end_tag_open_state;
mod rcdata_less_than_sign_state;
mod rcdata_state;
mod script_data_double_escape_end_state;
mod script_data_double_escape_start_state;
mod script_data_double_escaped_dash_dash_state;
mod script_data_double_escaped_dash_state;
mod script_data_double_escaped_less_than_sign_state;
mod script_data_double_escaped_state;
mod script_data_end_tag_name_state;
mod script_data_end_tag_open_state;
mod script_data_escape_start_dash_state;
mod script_data_escape_start_state;
mod script_data_escaped_dash_dash_state;
mod script_data_escaped_dash_state;
mod script_data_escaped_end_tag_name_state;
mod script_data_escaped_end_tag_open_state;
mod script_data_escaped_less_than_sign_state;
mod script_data_escaped_state;
mod script_data_less_than_sign_state;
mod script_data_state;
mod tag_name_state;
mod tag_open_state;

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

    pub fn append_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
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
    AfterAttrNameState,
    AttrNameState,
    BeforeAttrNameState,
    BogusCommentState,
    CharReferenceState,
    DataState,
    EndTagOpenState,
    MarkupDeclarationOpenState,
    PlaintextState,
    RCDataEndTagNameState,
    RCDataEndTagOpenState,
    RCDataLessThanSignState,
    RCDataState,
    RawtextEndTagNameState,
    RawtextEndTagOpenState,
    RawtextLessThanSignState,
    RawtextState,
    ScriptDataDoubleEscapeEndState,
    ScriptDataDoubleEscapeStartState,
    ScriptDataDoubleEscapedDashDashState,
    ScriptDataDoubleEscapedDashState,
    ScriptDataDoubleEscapedLessThanSignState,
    ScriptDataDoubleEscapedState,
    ScriptDataEndTagNameState,
    ScriptDataEndTagOpenState,
    ScriptDataEscapeStartDashState,
    ScriptDataEscapeStartState,
    ScriptDataEscapedDashDashState,
    ScriptDataEscapedDashState,
    ScriptDataEscapedEndTagNameState,
    ScriptDataEscapedEndTagOpenState,
    ScriptDataEscapedLessThanSignState,
    ScriptDataEscapedState,
    ScriptDataLessThanSignState,
    ScriptDataState,
    SelfClosingStartTagState,
    TagNameState,
    TagOpenState,
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
            State::ScriptDataLessThanSignState => self.consume_script_data_less_than_sign_state(),
            State::ScriptDataEndTagOpenState => self.consume_script_data_end_tag_open_state(),
            State::ScriptDataEndTagNameState => self.consume_script_data_end_tag_name_state(),
            State::ScriptDataEscapeStartState => self.consume_script_data_escape_start_state(),
            State::ScriptDataEscapeStartDashState => self.consume_script_data_escape_start_dash_state(),
            State::ScriptDataEscapedState => self.consume_script_data_escaped_state(),
            State::ScriptDataEscapedDashState => self.consume_script_data_escaped_dash_state(),
            State::ScriptDataEscapedDashDashState => self.consume_script_data_escaped_dash_dash_state(),
            State::ScriptDataEscapedLessThanSignState => self.consume_script_data_escaped_less_than_sign_state(),
            State::ScriptDataEscapedEndTagOpenState => self.consume_script_data_escaped_end_tag_open_state(),
            State::ScriptDataEscapedEndTagNameState => self.consume_script_data_escaped_end_tag_name_state(),
            State::ScriptDataDoubleEscapeStartState => self.consume_script_data_double_escape_start_state(),
            State::ScriptDataDoubleEscapedState => self.consume_script_data_double_escaped_state(),
            State::ScriptDataDoubleEscapedDashState => self.consume_script_data_double_escaped_dash_state(),
            State::ScriptDataDoubleEscapedDashDashState => self.consume_script_data_double_escaped_dash_dash_state(),
            State::ScriptDataDoubleEscapedLessThanSignState => self.consume_script_data_double_escaped_less_than_sign_state(),
            State::ScriptDataDoubleEscapeEndState => self.consume_script_data_double_escape_end_state(),
            State::BeforeAttrNameState => self.consume_before_attr_name_state(),

            // TODO: Cover all states instead of using a catchall
            _ => Vec::new()
        }
    }

    //
    // Tokenizer States
    //

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

    //
    // Helpers
    //
    fn append_char_to_tag_name(&mut self, letter: char) {
        self.edit_current_tag(|tag| tag.append_name(letter));
    }

    fn edit_current_tag<F>(&mut self, f: F)
        where F: Fn(&mut Tag) {

        match self.current_token() {
            Token::StartTagToken(mut tag) => {
                f(&mut tag);
                self.current_token = Some(Token::StartTagToken(tag))
            },
            Token::EndTagToken(mut tag) => {
                f(&mut tag);
                self.current_token = Some(Token::EndTagToken(tag))
            },
            _ => panic!("Unimplemented token")
        }
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

