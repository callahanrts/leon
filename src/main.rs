#[macro_use]
extern crate glium;

pub mod render;
pub mod paint;

use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;

use render::{parser, style, css, display};

fn main() {
    start_window();
}

fn start_window() {
    let html = read_file("html/basic.html".to_string());
    let css = read_file("html/basic.css".to_string());
    let stylesheet = css::parse(css);
    let root_node = parser::parse(html);
    let style_root = render::style::style_tree(&root_node, &stylesheet);
    let layout_root = render::layout::build_layout_tree(&style_root);

    use glium::{DisplayBuild,Surface};
    let gdisplay = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {

        for ev in gdisplay.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return, // Window has been closed by the user
                _ => ()
            }
        }

        // Sleep for a few ms to save cpu. In the future, maybe we can pause this thread for
        // inactive windows/tabs
        thread::sleep(Duration::from_millis(20))
    }
}

fn read_file(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    return str;
}
