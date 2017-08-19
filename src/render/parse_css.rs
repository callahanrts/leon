use css_parser::*;
use css_parser::parser::*;
use css_parser::tokenizer::*;

pub fn parse(css: String) {
    let mut parser = Parser::new(&*css);
    let stylesheet = parser.parse_stylesheet();
    for rule in stylesheet.rules {
        match rule {
            Rule::BasicRule(data) => {
                match data.block {
                    Block::SimpleBlock(data) => {
                        // Value is a vector of component values. These will need to be
                        // parsed into declarations to be most useful
                    },
                    _ => println!("no data")
                }
                for cv in data.prelude {
                    match cv {
                        ComponentValue::Token(token) => {
                            match token {
                                Token::DelimToken(c) => println!("{}", c),
                                _ => println!("token")
                            }
                        },
                        _ => {} // There will only ever be tokens in a prelud
                    }
                }
                // println!("\nname: {} \nvalue: {}", data.name, data.value);
            },
            _ => println!("non basic rule")
        }
    }
}

// It seems like there is an extra step involved here. Essentially the prelude
// is going to be something like:
//
// DelimToken('.')
// IdentToken('class')
// DelimToken(',')
// DelimToken('.')
// IdentToken('other-class')
//
// These tokens need to be interpreted to decide which declarations need to be
// applied to which elements in the style tree matching by ids and classes.
