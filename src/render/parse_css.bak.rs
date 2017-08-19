
// CSS3
use cssparser::*;
// use cssparser::{
//     Parser,
//     ParserInput,
//     AtRuleParser,
//     QualifiedRuleParser,
//     RuleListParser,
//     parse_one_rule,
//     ParseError,
//     CompactCowStr,
//     AtRuleType,
// };

pub struct CSSParser;

pub fn parse(css: String) {
    let mut input = ParserInput::new(&*css);
    let mut input = Parser::new(&mut input);
    let mut rule_parser = CSSParser{};

    let mut iter = RuleListParser::new_for_stylesheet(&mut input, rule_parser);
    println!("\n\nCSS:\n{}\n\n", css);

    while let Some(result) = iter.next() {
        match result {
            Ok(rule) => println!("{}: {}", rule.0, rule.1),
            Err(err) => {
                // let error = ContextualParseError::InvalidRule(
                //                 iter.input.slice(err.span), err.error);
                // log_css_error(iter.input, pos, error, iter.parser.context());
                println!("{}", iter.input.slice(err.span));
                match err.error.basic() {
                    // UnexpectedToken(_) => println!("unexpected token"),
                    EndOfInput => println!("end of input"),
                    // AtRuleInvalid(_) => println!("at rule invalid"),
                    AtRuleBodyInvalid => println!("at rule body invalid"),
                    QualifiedRuleInvalid => println!("qualified rule invalid"),
                    _ => println!("unknown error")
                }
            }
        }
    }

    // match parse_one_rule(&mut input, &mut rule_parser) {
    //     Ok(result) => println!("OK"),
    //     Err(e) => {
    //         match e.basic() {
    //             // UnexpectedToken(_) => println!("unexpected token"),
    //             EndOfInput => println!("end of input"),
    //             // AtRuleInvalid(_) => println!("at rule invalid"),
    //             AtRuleBodyInvalid => println!("at rule body invalid"),
    //             QualifiedRuleInvalid => println!("qualified rule invalid"),
    //             _ => println!("unknown error")
    //         }
    //     }
    // }
}

impl<'a> QualifiedRuleParser<'a> for CSSParser {
    type Prelude = String;
    type QualifiedRule = (String, String);
    type Error = String;

    fn parse_prelude<'b>(&mut self, input: &mut Parser<'a, 'b>)
                         -> Result<String, ParseError<'a, String>> {
        let namespaces = self.context.namespaces.unwrap();
        for space in namespaces {
            println!("{}", space);
        }
        Ok("TEST".to_owned())
    }

}


impl<'a>AtRuleParser<'a> for CSSParser {
    type Prelude = String;
    type AtRule = (String, String);
    type Error = String;

    fn parse_prelude<'t>(&mut self, name: CompactCowStr<'a>, input: &mut Parser<'a, 't>)
                         -> Result<AtRuleType<Self::Prelude, Self::AtRule>, ParseError<'a, Self::Error>> {
        Ok(AtRuleType::WithBlock("ATRULE".to_owned()))
    }

}
