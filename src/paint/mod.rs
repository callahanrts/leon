// // UI Backend.
// // The Paint module is responsible for painting pixels on the screen.
// use glium;

// use glium::{DisplayBuild, Surface};
// use glium::backend::glutin_backend::{GlutinFacade};

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 2],
// }

// implement_vertex!(Vertex, position);

// pub fn draw(display: &GlutinFacade, frame: &mut glium::Frame, program: &glium::Program) {
//     let vertex1 = Vertex { position: [-0.5, -0.5] };
//     let vertex2 = Vertex { position: [ 0.0,  0.5] };
//     let vertex3 = Vertex { position: [ 0.5, -0.25] };
//     let shape = vec![vertex1, vertex2, vertex3];

//     let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
//     let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


//     frame.clear_color(0.0, 0.0, 1.0, 1.0);
//     frame.draw(&vertex_buffer, &indices, program, &glium::uniforms::EmptyUniforms,
//                 &Default::default()).unwrap();
// }

// pub fn painter_args() -> (GlutinFacade, glium::Frame, glium::Program) {
//     let mut display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
//     let mut frame = display.draw();
//     let mut program = glium::Program::from_source(&display, vertex_shader_src(), fragment_shader_src(), None).unwrap();
//     return (display, frame, program);
// }

// // PRIVATE

// fn vertex_shader_src<'a>() -> &'a str {
//     let src = r#"
//         #version 140
//         in vec2 position;
//         void main() {
//             gl_Position = vec4(position, 0.0, 1.0);
//         }
//     "#;
//     return src;
// }

// fn fragment_shader_src<'a>() -> &'a str {
//     let src = r#"
//         #version 140
//         out vec4 color;
//         void main() {
//             color = vec4(1.0, 0.0, 0.0, 1.0);
//         }
//     "#;
//     return src;
// }
