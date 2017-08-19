use std::collections::{HashSet,HashMap};
use html5ever::rcdom::{RcDom, Handle, NodeData};
use css_parser::*;
use css_parser::parser::*;
use css_parser::tokenizer::*;


pub struct StyleNode {
    pub node: Handle,
    pub values: Vec<Declaration>,
    pub children: Vec<StyleNode>,
}

pub fn style_tree<'a>(root: &'a RcDom, css: String) -> StyleNode {
    let mut parser = Parser::new(&*css);
    let stylesheet = parser.parse_stylesheet();
    style_subtree(root.document.clone(), &stylesheet)
}

fn style_subtree<'a>(node: Handle, stylesheet: &'a StyleSheet) -> StyleNode {
    StyleNode {
        node: node.clone(),
        values: Vec::new(), //style_values(&node.data, stylesheet),
        children: node.children.borrow().iter().map(|child| style_subtree(child.clone(), stylesheet)).collect(),
    }
}

// Apply Styles to a single element, returning the specified values
// fn style_values(elem: &NodeData, stylesheet: &css::StyleSheet) -> PropertyMap {
//     let mut values = HashMap::new();
//     let mut rules = matching_rules(elem, stylesheet);

//     // Go through the rules from lowest to highestspecificity
//     rules.sort_by(|&(a,_), &(b,_)| a.cmp(&b));
//     for (_, rule) in rules {
//         for declaration in &rule.declarations {
//             values.insert(declaration.name.clone(), declaration.value.clone());
//         }
//     }

//     return values;
// }

// fn parse(css: String) {
//     for rule in stylesheet.rules {
//         match rule {
//             Rule::BasicRule(data) => {
//                 match data.block {
//                     Block::SimpleBlock(data) => {
//                         // Value is a vector of component values. These will need to be
//                         // parsed into declarations to be most useful
//                     },
//                     _ => println!("no data")
//                 }
//                 for cv in data.prelude {
//                     match cv {
//                         ComponentValue::Token(token) => {
//                             match token {
//                                 Token::DelimToken(c) => println!("{}", c),
//                                 _ => println!("token")
//                             }
//                         },
//                         _ => {} // There will only ever be tokens in a prelud
//                     }
//                 }
//                 // println!("\nname: {} \nvalue: {}", data.name, data.value);
//             },
//             _ => println!("non basic rule")
//         }
//     }
// }

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
