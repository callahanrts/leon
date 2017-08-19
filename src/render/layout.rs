// Layout Tree
// The layout tree starts from a style tree and is made up of objects that
// represent boxes that will be painted to the page.

use render::style::{StyleNode,Display};
use css_parser::tokenizer::*;

// CSS box model. All sizes are in PX

#[derive(Clone,Copy,Default)]
pub struct Dimensions {
    // Position of the content area relative to the document origin.
    pub content: Rect,

    // Surrounding edges
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    // The area covered by the content area  plus its padding
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    // The area covered by the content area plus padding and borders
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    // The area covered by the content area plus padding, borders, and margin
    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[derive(Clone,Copy,Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Clone,Copy,Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>
}

impl<'a> LayoutBox<'a> {
    // Lay out a box and its descendants
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => {}, // TODO
            BoxType::AnonymousBlock => {}, // TODO
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate
        // this box's width before laying out its children
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container
        self.calculate_block_position(containing_block);

        // Recursively layout the children of this box
        self.layout_block_children();

        // Parent height can depend on child height, so calculate_height must
        // be called after the children are layed out
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

    //     // Width has initial value auto.
    //     let auto = Value::Keyword("auto".to_string());
    //     let mut width = style.value("width").unwrap_or(auto.clone());

    //     // Margin, border, and padding all have initial value 0.0
    //     let zero = Value::Length(0.0, Unit::Px);

    //     let mut margin_left = style.lookup("margin-left", "margin", &zero);
    //     let mut margin_right = style.lookup("margin-right", "margin", &zero);

    //     let border_left = style.lookup("border-left-width", "border-width", &zero);
    //     let border_right = style.lookup("border-right-width", "border-width", &zero);

    //     let padding_left = style.lookup("padding-left", "padding", &zero);
    //     let padding_right = style.lookup("padding-right", "padding", &zero);

    //     let total = sum([&margin_left, &margin_right, &border_left, &border_right,
    //                  &padding_left, &padding_right].iter().map(|v| v.to_px()));

    //     if width != auto && total > containing_block.content.width {
    //         if margin_left == auto {
    //             margin_left = Value::Length(0.0, Unit::Px);
    //         }
    //         if margin_right == auto {
    //             margin_right = Value::Length(0.0, Unit::Px);
    //         }
    //     }

    //     let underflow = containing_block.content.width - total;

    //     match  (width == auto, margin_left == auto, margin_right == auto) {
    //         // If the values are overconstrained, calculate margin_right.
    //         (false, false, false) => {
    //             margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
    //         }

    //         // If exactly one size is auto, its used value follows from the equality
    //         (false, false, true) => { margin_right = Value::Length(underflow, Unit::Px) }
    //         (false, true, false) => { margin_left = Value::Length(underflow, Unit::Px) }

    //         // If width is set to auto, any other auto values become 0.
    //         (true, _, _) => {
    //             if margin_left == auto { margin_left = Value::Length(0.0, Unit::Px) }
    //             if margin_right == auto { margin_right = Value::Length(0.0, Unit::Px) }

    //             if underflow >= 0.0 {
    //                 // Expand width to fill the underflow
    //                 width = Value::Length(underflow, Unit::Px);
    //             } else {
    //                 // Width can't be negative. Adjust right margin instead
    //                 width = Value::Length(0.0, Unit::Px);
    //                 margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
    //             }
    //         }

    //         // If margin left and margin right are both auto, their used values are equal
    //         (false, true, true) => {
    //             margin_left = Value::Length(underflow / 2.0, Unit::Px);
    //             margin_right = Value::Length(underflow / 2.0, Unit::Px);
    //         }
    //     }

    //     let d = &mut self.dimensions;
    //     d.content.width = width.to_px();

    //     d.padding.left = padding_left.to_px();
    //     d.padding.right = padding_right.to_px();

    //     d.border.left = border_left.to_px();
    //     d.border.right = border_right.to_px();

    //     d.margin.left = margin_left.to_px();
    //     d.margin.right = margin_right.to_px();
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
    //     let d = &mut self.dimensions;

    //     // margin, border, and padding have initial value 0.
    //     let zero = Value::Length(0.0, Unit::Px);

    //     d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
    //     d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

    //     d.border.top = style.lookup("border-bottom-width", "border-width", &zero).to_px();
    //     d.border.bottom = style.lookup("border-top-width", "border-width", &zero).to_px();

    //     d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
    //     d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

    //     d.content.x = containing_block.content.x +
    //                   d.margin.left + d.border.left + d.padding.left;

    //     // Position the block below all the previous boxes in the container
    //     d.content.y = containing_block.content.height + containing_block.content.y +
    //                   d.margin.top + d.border.top + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Track the height so each child is layed out below the previous content
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        // If the height property is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by layout_block_children.
        self.dimensions.content.height = 0.0; // initialize height

        // if let Some(mut dec) = self.get_style_node().value("height") {
        //     if let Some(token) = dec.number_value() {
        //         self.dimensions.content.height = match token {
        //             Token::PercentageToken(per) => 0.0,
        //             Token::DimensionToken{value: val, num_type: tpe, unit: unit} => val,
        //             Token::NumberToken{value: val, num_type: tpe} => val,
        //             _ => 0.0
        //         };
        //     }
        // }
    }
}

pub enum BoxType<'a> {
    BlockNode(&'a StyleNode),
    InlineNode(&'a StyleNode),
    AnonymousBlock,
}

// Transform a style tree into a layout tree.
pub fn layout_tree<'a>(node: &'a StyleNode, mut containing_block: Dimensions) -> LayoutBox<'a> {
    // The layout algorithm expects the container height to start at 0.
    // TODO: Save the initial containing block height, for calculating percent heights.
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet
pub fn build_layout_tree<'a>(style_node: &'a StyleNode) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has display: none"),
    });

    // Create the descendant boxes
    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root.get_inline_container().children.push(build_layout_tree(child)),
            Display::None => {}, // Skip nodes with display: none
        }
    }

    return root;
}

impl<'a> LayoutBox<'a> {
    // Constructor function
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(), // Initially set all fields to 0.0
            children: Vec::new(),
        }
    }

    // Where a new inline child should go
    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                // If we've just generated an anonymous block, keep using it.
                // Otherwise, create a new one
                match self.children.last() {
                    Some(&LayoutBox { box_type: BoxType::AnonymousBlock,..})  => {},
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }

                return self.children.last_mut().unwrap();
            }
        }
    }

    fn get_style_node(&self) -> &'a StyleNode {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block has no style node")
        }
    }
}


fn sum<I>(iter: I) -> f32 where I: Iterator<Item=f32> {
    iter.fold(0., |a, b| a + b)
}

