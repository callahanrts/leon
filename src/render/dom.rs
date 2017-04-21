
// DOM Module
use std::collections::{HashMap,HashSet};

// Struct representing a DOM node
#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

// Nodes can either be Text, or Element nodes.
#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

// Elements, for now, will contain a tag name and a map of key/value
// attribute pairs
#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

// Constructors
pub fn text(data: String) -> Node {
    Node{ children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node{
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
