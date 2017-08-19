// #[macro_use]
extern crate glium;
extern crate html5ever;
extern crate css_parser;

pub mod render;

use std::fs::File;
use std::io::Read;
// use std::time::Duration;
// use std::thread;

// HTML5
use html5ever::{parse_document};
use html5ever::driver::ParseOpts;
use html5ever::rcdom::RcDom;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tendril::TendrilSink;


fn main() {
    start_window();
}

fn start_window() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let html_bytes = read_file("html/basic.html".to_string());
    // let css = read_file("html/basic.css".to_string());
    // let stylesheet = css::parse(css);

    let css = read_file("html/basic.css".to_string());
    // parse_css::parse(css2);

    // Parse HTML
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html_bytes.as_bytes())
        .unwrap();

    render::style::style_tree(&dom, css);

    // use glium::{DisplayBuild};
    // let gdisplay = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // loop {

    //     let win = gdisplay.get_window().unwrap();
    //     let (width, height) = win.get_inner_size_pixels().unwrap();
    //     let dim = render::layout::Dimensions{
    //         content: render::layout::Rect{
    //             width: width as f32,
    //             height: height as f32,
    //             x: 0.0,
    //             y: 0.0,
    //         },
    //         .. Default::default()
    //     };

    //     let layout_root = render::layout::layout_tree(&style_root, dim);
    //     render::display::paint(&gdisplay, &layout_root);

    //     for ev in gdisplay.poll_events() {
    //         match ev {
    //             glium::glutin::Event::Closed => return, // Window has been closed by the user
    //             _ => ()
    //         }
    //     }

    //     // Sleep for a few ms to save cpu. In the future, maybe we can pause this thread for
    //     // inactive windows/tabs
    //     thread::sleep(Duration::from_millis(20))
    // }
}

fn read_file(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    return str;
}
