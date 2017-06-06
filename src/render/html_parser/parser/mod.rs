use render::html_parser::tokenizer;

struct Document {
}

// NOTE: Never Used
struct Parser<'a> {
    pos: usize,
    input: &'a str,

    insertion_mode: &'a str,
    original_insertion_mode: &'a str,
    template_insertion_modes: Vec<&'a str>,
    // Stack of open elements
    open_elements: Vec<tokenizer::Token>,
}

impl<'a> Parser<'a> {
    // NOTE: Never used
    pub fn new(input: &'a str) -> Parser {
        Parser{
            pos: 0,
            input: input,

            insertion_mode: "",
            original_insertion_mode: "",
            template_insertion_modes: Vec::new(),
            open_elements: Vec::new(),
        }
    }

    // Parse / Tree construction dispatcher
    // http://w3c.github.io/html/syntax.html#tree-construction
    pub fn parse(&mut self) -> Document {
        let tk = tokenizer::Tokenizer::new(self.input);
        loop {
            // The tokenizer can emit more than one token at a time
            // for token in tk.consume_token() {
            //     if self.is_foreign_token(token) {
            //         self.process_foreign_token(token);
            //     } else {
            //         self.process_token(token);
            //     }
            // }
            break
        }
        Document{}
    }

    // http://w3c.github.io/html/syntax.html#the-insertion-mode
    // NOTE: Never used
    // fn reset_insertion_mode() {
    // }

    pub fn process_token(&mut self, token: tokenizer::Token) {
    }

    pub fn process_foreign_token(&mut self, token: tokenizer::Token) {
    }

    fn is_foreign_token(&mut self, token: tokenizer::Token) -> bool {
        self.open_elements.len() == 0 || // If the stack of open elements is empty
        // If the adjusted current node is an element in the HTML namespace
        // If the adjusted current node is a MathML text integration point and the token is a start tag whose tag name is neither "mglyph" nor "malignmark"
        // If the adjusted current node is a MathML text integration point and the token is a character token
        // If the adjusted current node is a MathML annotation-xml element and the token is a start tag whose tag name is "svg"
        // If the adjusted current node is an HTML integration point and the token is a start tag
        // If the adjusted current node is an HTML integration point and the token is a character token
        is_eof_token(token) // If the token is an end-of-file token
    }
}

fn is_eof_token(token: tokenizer::Token) -> bool {
    match token {
        tokenizer::Token::EOFToken => true,
        _ => false
    }
}
