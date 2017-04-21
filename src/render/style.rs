// Style tree
// The style tree combines the dom tree and css styles into a single object. It
// is set up so the order reflects the order in which elements should be painted.

use std::collections::HashMap;
use render::css;
use render::dom;

pub type PropertyMap = HashMap<String, css::Value>;

#[derive(Debug)]
pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Debug)]
pub struct StyleNode<'a> {
    pub node: &'a dom::Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyleNode<'a>>,
}

impl<'a> StyleNode<'a> {
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

fn matches(elem: &dom::ElementData, selector: &css::Selector) -> bool {
    match *selector {
        css::Selector::Simple(ref simple_selector) => {
            matches_simple_selector(elem, simple_selector)
        }
    }
}

fn matches_simple_selector(elem: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID Selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check Class Selectors
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem.classes().contains(&**class)) {
        return false;
    }

    // We didn't find any non-matching selector components
    return true;
}

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);
// if rule matches elem, return a MatchedRule, otherwise return None
fn match_rule<'a>(elem: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
    // Find the first (highest specificity) matching selector
    rule.selectors.iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match a given element
fn matching_rules<'a>(elem: &dom::ElementData, stylesheet: &'a css::StyleSheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

// Apply Styles to a single element, returning the specified values
fn specified_values(elem: &dom::ElementData, stylesheet: &css::StyleSheet) -> PropertyMap {
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

pub fn style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::StyleSheet) -> StyleNode<'a> {
    StyleNode {
        node: root,
        specified_values: match root.node_type {
            dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            dom::NodeType::Text(_) => HashMap::new(),
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect()
    }
}
