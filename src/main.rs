extern crate glium;

use glium::*;
use glium::glutin::*;
use glium::glutin::Api;
use glium::glutin::dpi::LogicalSize;
use glium::index::*;
use glium::vertex::VertexBuffer;
use glium::uniforms::EmptyUniforms;

#[derive(Copy, Clone)]
struct KLinesVertex {
    vertex_position: [f32; 3],
    vertex_color: [f32; 4],
}
implement_vertex!(KLinesVertex, vertex_position, vertex_color);

fn main() {
    // setup events loop
    let mut events_loop = EventsLoop::new();

    // setup window
    let window_builder = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(1280.0, 720.0))
        .with_title("Hello World");

    // setup context
    let context_builder = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5)))
        .with_gl_profile(GlProfile::Core);

    // setup display
    let display = Display::new(window_builder, context_builder, &events_loop).unwrap();

    // load shader strings
    let simple_vert_glsl = include_str!("simple.vert.glsl");
    let simple_frag_glsl = include_str!("simple.frag.glsl");

    // create triangle vertices
    let triangle = vec![
        KLinesVertex { vertex_position: [-0.5, -0.5, 0.0], vertex_color: [1.0, 0.0, 0.0, 1.0] },
        KLinesVertex { vertex_position: [0.5, -0.5, 0.0], vertex_color: [0.0, 1.0, 0.0, 1.0] },
        KLinesVertex { vertex_position: [0.0, 0.5, 0.0], vertex_color: [0.0, 0.0, 1.0, 1.0] }
    ];

    // create vertex buffer
    let vertices = VertexBuffer::new(&display, &triangle).unwrap();

    // don't use indices yet
    let indices = NoIndices(PrimitiveType::TrianglesList);

    // compile and link the shader program
    let program = Program::from_source(&display, simple_vert_glsl, simple_frag_glsl, None).unwrap();

    // detect window close
    let mut close_requested = false;
    while !close_requested {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => close_requested = true,
                    _ => ()
                },
                _ => ()
            }
        });

        // draw to the window
        let mut target = display.draw();
        // clear before every draw
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        // do the draw
        target.draw(&vertices, &indices, &program, &EmptyUniforms, &Default::default()).unwrap();
        // done
        target.finish().unwrap();
    }
}
