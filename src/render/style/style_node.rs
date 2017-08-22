use css_parser::parser::*;
use css_parser::tokenizer::{Token};
use html5ever::rcdom::{Handle};

pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Unit {
    Px,
    Pt,
    Percentage,
    None
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

pub struct StyleNode {
    pub node: Handle,
    pub values: Vec<Declaration>,
    pub children: Vec<StyleNode>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    KeywordValue(String),
    LengthValue(f32, Unit),
    ColorValue(Color),
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::LengthValue(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

impl StyleNode {
    // Return the specified value of a property if it exists. Otherwise, None
    pub fn value(&self, name: &str) -> Option<Value> {
        for declaration in self.values.clone() {
            if declaration.name == name {
                return parse_value(name, declaration);
            }
        }
        None
    }

    // pub fn lookup(&self, name: &str, fallback_name: &str, default: &css::Value) -> Option<Value> {
    //     match self.value(name) {
    //         Some(val) => Some(val),
    //         None => self.value(fallback_name)
    //     }
    // }
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

    // The value of the display property -- defaults to inline
    pub fn display(&self) -> Display {
        if let Some(val) = self.value("display") {
            match keyword(val).as_ref() {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            }
        } else {
            Display::Block
        }
    }

}

fn keyword(val: Value) -> String {
    match val {
        Value::KeywordValue(name) => name,
        _ => String::new()
    }
}


// Parse supported CSS values
fn parse_value(name: &str, dec: Declaration) -> Option<Value> {
    match name {
        x if is_keyword_value(x) => Some(parse_keyword(dec)),
        x if is_number_value(x) => Some(parse_number(dec)),
        x if is_color_value(x) => Some(parse_color(dec)),
        _ => None
    }
}

fn parse_keyword(dec: Declaration) -> Value {
    let ref component_value = dec.value[0];
    let name = match *component_value {
        ComponentValue::Token(ref token) => {
            match *token {
                Token::IdentToken(ref name) => name.clone(),
                _ => String::new()
            }
        },
        _ => String::new()
    };

    return Value::KeywordValue(name);
}

fn parse_number(dec: Declaration) -> Value {
    let ref component_value = dec.value[0];
    match *component_value {
        ComponentValue::Token(ref token) => {
            match *token {
                Token::PercentageToken(percent) => Value::LengthValue(percent, Unit::Percentage),
                Token::NumberToken{value: v, num_type: _} => Value::LengthValue(v, Unit::None),
                Token::DimensionToken{value: v, num_type: _, unit: ref unit} => Value::LengthValue(v, parse_unit(unit.clone())),
                _ => Value::LengthValue(0.0, Unit::Px)
            }
        },
        _ => Value::LengthValue(0.0, Unit::Px),
    }
}

fn parse_unit(unit: String) -> Unit {
    match unit.as_ref() {
        "px" => Unit::Px,
        "pt" => Unit::Pt,
        _ => Unit::Px
    }
}

fn parse_color(dec: Declaration) -> Value {
    let color = parse_hex_value(dec);
    return Value::ColorValue(color);
}

fn parse_hex_value(dec: Declaration) -> Color {
    let ref component_value = dec.value[0];
    let mut color = Color::new(0,0,0,0);
    match *component_value {
        ComponentValue::Token(ref token) => {
            match *token {
                // TODO: Could also be a function. rgb() rgba()
                Token::HashToken{hash_type: _, name: ref name} => {
                    let r = parse_hex_pair(name, 0);
                    let g = parse_hex_pair(name, 1);
                    let b = parse_hex_pair(name, 2);
                    Color::new(r, g, b, 1)
                },
                _ => color
            }
        },
        _ => color
    }
}

fn parse_hex_pair(name: &str, index: usize) -> u8 {
    let chars_per = name.len() / 3;
    let mut s = &name[index * chars_per .. (index + 1) * chars_per];
    u8::from_str_radix(s, 16).unwrap()
}

fn is_keyword_value(name: &str) -> bool {
    [
        "display"
    ].contains(&name)
}

fn is_number_value(name: &str) -> bool {
    [
        "height",
        "width",

        "margin-left",
        "margin-right",
        "margin-top",
        "margin-bottom",

        "padding-left",
        "padding-right",
        "padding-top",
        "padding-bottom",

        "border-left-width",
        "border-right-width",
        "border-top-width",
        "border-bottom-width",
    ].contains(&name)
}

fn is_color_value(name: &str) -> bool {
    [
        "box-color",
        "background",
        "color",
    ].contains(&name)
}

pub fn number_value(dec: Declaration) -> Option<Token> {
    let ref component_value = dec.value[0];
    match *component_value {
        ComponentValue::Token(ref token) => Some(token.clone()),
        _ => None
    }
}
