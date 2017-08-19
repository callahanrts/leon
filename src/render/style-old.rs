// Style tree
// The style tree combines the dom tree and css styles into a single object. It
// is set up so the order reflects the order in which elements should be painted.

use std::collections::{HashSet,HashMap};
use render::css;
use html5ever::rcdom::{RcDom, Handle, NodeData};

pub type PropertyMap = HashMap<String, css::Value>;

pub enum Display {
    Inline,
    Block,
    None,
}

pub struct StyleNode {
    pub node: Handle,
    pub specified_values: PropertyMap,
    pub children: Vec<StyleNode>,
}

impl StyleNode {
    // Return the specified value of a property if it exists. Otherwise, None
    pub fn value(&self, name: &str) -> Option<css::Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &css::Value) -> css::Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

    // The value of the display property -- defaults to inline
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(css::Value::Keyword(s)) => match  &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

fn matches(elem: &NodeData, selector: &css::Selector) -> bool {
    match *selector {
        css::Selector::Simple(ref simple_selector) => {
            matches_simple_selector(elem, simple_selector)
        }
    }
}

fn matches_simple_selector(elem: &NodeData, selector: &css::SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| element_name(elem) != *name) {
        return false;
    }

    // Check ID Selector
    if selector.id.iter().any(|id| &*element_id(elem) != id) {
        return false;
    }

    // Check Class Selectors
    if selector.class.iter().any(|class| !element_classes(elem).contains(&**class)) {
        return false;
    }

    // We didn't find any non-matching selector components
    return true;
}

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);
// if rule matches elem, return a MatchedRule, otherwise return None
fn match_rule<'a>(elem: &NodeData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
    // Find the first (highest specificity) matching selector
    rule.selectors.iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match a given element
fn matching_rules<'a>(elem: &NodeData, stylesheet: &'a css::StyleSheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

// Apply Styles to a single element, returning the specified values
fn specified_values(elem: &NodeData, stylesheet: &css::StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highestspecificity
    rules.sort_by(|&(a,_), &(b,_)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    return values;
}

pub fn style_tree<'a>(root: &'a RcDom, stylesheet: &'a css::StyleSheet) -> StyleNode {
    style_subtree(root.document.clone(), stylesheet)
}

pub fn style_subtree<'a>(node: Handle, stylesheet: &'a css::StyleSheet) -> StyleNode {
    StyleNode {
        node: node.clone(),
        specified_values: specified_values(&node.data, stylesheet),
        children: node.children.borrow().iter().map(|child| style_subtree(child.clone(), stylesheet)).collect(),
    }
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
