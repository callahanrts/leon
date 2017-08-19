use std::collections::{HashSet};
use html5ever::rcdom::{RcDom, Handle, NodeData};
// use css_parser::*;
use css_parser::parser::*;
// use css_parser::tokenizer::*;

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
        values: style_values(&node.data, stylesheet),
        children: node.children.borrow().iter().map(|child| style_subtree(child.clone(), stylesheet)).collect(),
    }
}

// Apply Styles to a single element, returning the specified values
fn style_values<'a>(elem: &NodeData, stylesheet: &'a StyleSheet) -> Vec<Declaration> {
    let mut values = Vec::new();
    let rules: Vec<Rule> = stylesheet.rules.iter()
                                           .filter(|r| is_match(elem, r))
                                           .cloned()
                                           .collect();

    // Go through the rules from lowest to highestspecificity
    // TODO: Sort by specificity
    // rules.sort_by(|&(a,_), &(b,_)| a.cmp(&b));

    for rule in rules {
        match rule {
            Rule::BasicRule(data) => {
                let mut declarations = parse_block_declarations(data.block);
                values.append(&mut declarations);
            }
            _ => {}
        }
    }

    return values;
}

fn is_match(elem: &NodeData, rule: &Rule) -> bool {
    match *rule {
        Rule::BasicRule(ref data) => match_basic_rule(elem, data),
        _ => false // At rules are not currrently supported
    }
}

fn match_basic_rule(elem: &NodeData, rule_data: &RuleData) -> bool {
    let prelude = rule_data.prelude.clone();
    let (_type, id, classes) = parse_prelude(prelude);
    let (el_type, el_id, el_classes) = parse_element_selectors(elem);
    let matches_type = _type == el_type;                    // Check Type
    let matches_id = id == el_id;                           // Check Id
    let matches_class = matches_class(el_classes, classes); // Check ClassList

    return matches_type && matches_id && matches_class;
}

fn matches_class(elem_classes: HashSet<String>, stylesheet_classes: Vec<String>) -> bool {
    let mut matches = true; // Assume the classes match initially
    for class in stylesheet_classes {
        // Make sure all classes match. This is a very simple comparison, it will
        // grow to be more complex when commas are allowed.
        matches = matches && elem_classes.contains(&class);
    }

    return matches;
}

fn parse_element_selectors(elem: &NodeData) -> (String, String, HashSet<String>) {
    let name = element_name(elem);
    let id = element_id(elem);
    let classes = element_classes(elem);
    return (name.to_owned(), id, classes);
}

pub fn element_name(data: &NodeData) -> &str {
    match *data {
        // Return the name (type) of element
        NodeData::Element { ref name, .. } => &*name.local,
        _ => ""
    }
}

pub fn element_id(data: &NodeData) -> String {
    match *data {
        NodeData::Element { ref attrs, .. } => {
            // Search for the id attribute and return the value
            for attr in attrs.borrow().iter() {
                if &*attr.name.local == "id" {
                    return String::from(&*attr.value);
                }
            }
            return String::new();
        }
        _ => { String::new() }
    }
}

pub fn element_classes(data: &NodeData) -> HashSet<String> {
    match *data {
        NodeData::Element { ref attrs, .. } => {
            // Search for the class attribute and return a vector of the space separated classes
            for attr in attrs.borrow().iter() {
                if &*attr.name.local == "class" {
                    return (&*attr.value).split(" ").map(|c| c.to_string()).collect();
                }
            }
            return HashSet::new();
        }
        _ => { HashSet::new() }
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
