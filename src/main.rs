// piston
#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;

pub mod render;
pub mod paint;

use std::fs::File;
use std::io::Read;

use render::{parser, style, css};
use piston_window::*;

fn main() {
    start_window();
}

fn start_window() {
    let mut window: PistonWindow = WindowSettings::new("TEST", (640, 480))
          .exit_on_esc(true)
          .build()
          .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let html = read_file("html/basic.html".to_string());
    let css = read_file("html/basic.css".to_string());
    let stylesheet = css::parse(css);
    let root_node = parser::parse(html);
    let style_root = render::style::style_tree(&root_node, &stylesheet);
    let layout_root = render::layout::build_layout_tree(&style_root);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |_c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            render::display::paint(&layout_root);
        });
    }

}

fn read_file(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    return str;
}
