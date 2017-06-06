// https://dom.spec.whatwg.org/#dom-node-getrootnode

use std::collections::HashMap;
use super::{Document,Element};

pub type DOMString = String;
pub type USVString = String;

const ELEMENT_NODE: u32 = 1;
const ATTRIBUTE_NODE: u32 = 2;
const TEXT_NODE: u32 = 3;
const CDATA_SECTION_NODE: u32 = 4;
const ENTITY_REFERENCE_NODE: u32 = 5; // historical
const ENTITY_NODE: u32 = 6; // historical
const PROCESSING_INSTRUCTION_NODE: u32 = 7;
const COMMENT_NODE: u32 = 8;
const DOCUMENT_NODE: u32 = 9;
const DOCUMENT_TYPE_NODE: u32 = 10;
const DOCUMENT_FRAGMENT_NODE: u32 = 11;
const NOTATION_NODE: u32 = 12; // historical

const DOCUMENT_POSITION_DISCONNECTED: u32 = 0x01;
const DOCUMENT_POSITION_PRECEDING: u32 = 0x02;
const DOCUMENT_POSITION_FOLLOWING: u32 = 0x04;
const DOCUMENT_POSITION_CONTAINS: u32 = 0x08;
const DOCUMENT_POSITION_CONTAINED_BY: u32 = 0x10;
const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u32 = 0x20;

// Has EventTarget trait
struct Node<T> {
  nodeType: u32,
  nodeName: DOMString,

  baseURI: USVString,

  isConnected: bool,
  ownerDocument: Option<Document>,
  parentNode: Option<Box<T>>,
  parentElement: Option<Element>,
  firstChild: Option<Box<T>>,
  lastChild: Option<Box<T>>,
  previousSibling: Option<Box<T>>,
  nextSibling: Option<Box<T>>,

  // [SameObject]
  childNodes: Vec<Option<Box<T>>>,

  // [CEReactions]
  nodeValue: Option<DOMString>,
  textContent: Option<DOMString>,
}

impl<T> Node<T> {
    fn new() -> Node<T> {
        Node{
            nodeType: 0,
            nodeName: DOMString::new(),

            baseURI: USVString::new(),

            isConnected: false,
            ownerDocument: None,
            parentNode: None,
            parentElement: None,
            firstChild: None,
            lastChild: None,
            previousSibling: None,
            nextSibling: None,

            // [SameObject]
            childNodes: Vec::new(),

            // [CEReactions]
            nodeValue: None,
            textContent: None,
        }
    }

    fn isConnected(&self) {
    }

    fn getRootNode(&mut self, options: Option<HashMap<&str, &str>>) -> Node<T> {
      let mut default = HashMap::new();
      default.insert("composed", "false");
      // options = options.unwrap_or(default);
      Node::new()
    }

    fn hasChildNodes(&self) -> bool {
        false
    }

    // [CEReactions]
    fn normalize() {
    }

    // [CEReactions, NewObject]
    fn cloneNode(&self, deep: Option<bool>) -> Node<T> {
      let deep = deep.unwrap_or(false);
      Node::new()
    }

    fn isEqualNode(&self, otherNode: Option<Node<T>>) -> bool {
      false
    }

    // historical alias of ===
    fn isSameNode(&self, otherNode: Option<Node<T>>) -> bool {
      false
    }

    fn compareDocumentPosition(&self, other: Node<T>) -> u32 {
      1
    }

    fn contains(&self, other: Option<Node<T>>) -> bool {
      false
    }

    fn lookupPrefix(&self, namespace: Option<DOMString>) -> Option<DOMString> {
        None
    }

    fn lookupNamespaceURI(&self, prefix: Option<DOMString>) -> Option<DOMString> {
        None
    }

    fn isDefaultNamespace(&self, namespace: Option<DOMString>) -> bool {
        false
    }

    // CEReactions
    fn insertBefore(&self, node: Node<T>, child: Option<Node<T>>) -> Node<T> {
        Node::new()
    }

    // CEReactions
    fn appendChild(&self, node: Node<T>) -> Node<T> {
        Node::new()
    }

    // CEReactions
    fn replaceChild(&self, node: Node<T>, child: Node<T>) -> Node<T> {
        Node::new()
    }

    // CEReactions
    fn removeChild(&self, child: Node<T>) -> Node<T> {
        Node::new()
    }
}
