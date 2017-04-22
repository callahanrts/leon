use render::layout::{LayoutBox,Rect,BoxType};
use render::css::{Color,Value};

use glium;
use glium::{Surface};
use glium::backend::glutin_backend::{GlutinFacade};

type DisplayList = Vec<DisplayCommand>;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);


#[derive(Debug)]
enum DisplayCommand {
    SolidColor(Color, Rect),
    // Insert more commands here
}

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    // TODO: render text

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(color, layout_box.dimensions.border_box()))
    });
}

// Return the specified color for css property name, or none if no color was specified
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None,
        },
        BoxType::AnonymousBlock => None,
    }
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "box-color") {
        Some(color) => color,
        _ => return // bail out if no border color is specified
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));

}

fn paint_item(item: &DisplayCommand, display: &GlutinFacade, frame: &mut glium::Frame) {
    match item {
        &DisplayCommand::SolidColor(color, rect) => {
            let program = glium::Program::from_source(display, vertex_shader_src(), &*fragment_shader_src(color), None).unwrap();
            draw_rect(display, frame, &program, rect);
        }
    }
}

// Paint a tree of layout boxes
pub fn paint(display: &glium::backend::glutin_backend::GlutinFacade, layout_root: &LayoutBox) {
    let mut frame = display.draw();
    frame.clear_color(1.0, 1.0, 1.0, 1.0);

    let display_list = build_display_list(layout_root);
    for item in &display_list {
        paint_item(&item, display, &mut frame);
    }
    frame.finish().unwrap();
}

fn xcoord(x: f32, w: u32) -> f32 {
    ((x * 2.0) / w as f32) - 1.0
}

fn ycoord(y: f32, h: u32) -> f32 {
    (((y * 2.0) / h as f32) - 1.0) * -1.0
}


fn draw_rect(display: &GlutinFacade, frame: &mut glium::Frame, program: &glium::Program, rect: Rect) {
    let win = display.get_window().unwrap();
    let (width, height) = win.get_inner_size_pixels().unwrap();

    let vertex1 = Vertex { position: [xcoord(rect.x, width), ycoord(rect.y, height)] };
    let vertex2 = Vertex { position: [xcoord(rect.x + rect.width, width), ycoord(rect.y, height)] };
    let vertex3 = Vertex { position: [xcoord(rect.x + rect.width, width), ycoord(rect.y + rect.height, height)] };
    let vertex4 = Vertex { position: [xcoord(rect.x, width), ycoord(rect.y + rect.height, height)] };
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let shape = vec![vertex1, vertex2, vertex3];
    let shape2 = vec![vertex1, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(display, &shape2).unwrap();

    let draw_parameters = glium::DrawParameters{
        .. Default::default()
    };

    frame.draw(&vertex_buffer, &indices, program, &glium::uniforms::EmptyUniforms, &draw_parameters).unwrap();
    frame.draw(&vertex_buffer2, &indices, program, &glium::uniforms::EmptyUniforms, &draw_parameters).unwrap();
}

fn vertex_shader_src<'a>() -> &'a str {
    let src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    return src;
}

fn fragment_shader_src<'a>(color: Color) -> String {
    let src = format!(r##"
        #version 140
        out vec4 color;
        void main() {{
            color = vec4({}, {}, {}, {});
        }}
    "##, color.r / 255, color.g / 255, color.b / 255, color.a / 255).to_owned();
    return src;
}
