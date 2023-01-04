use glium::{
    glutin::{self, event::ElementState, dpi::PhysicalPosition}, 
    Surface, 
    implement_vertex, 
    VertexBuffer, 
    draw_parameters::DrawParameters
};

#[derive(Copy, Clone)]
struct Vertex {
position: [f32; 2],
in_color: [f32; 4]
}

fn add_vertex(display: &glium::Display, shape: &mut Vec<Vertex> , position: PhysicalPosition<f64>,
          in_color: [f32;4], vertex_buffer: &mut VertexBuffer<Vertex>) {
let (width, height) = display.get_framebuffer_dimensions();
let x:f32 = 2.0*(position.x/width as f64 - 0.5) as f32;
let y:f32 = 2.0*(0.5-position.y/height as f64) as f32;
let new_vertex = Vertex {position: [x, y], in_color};                               
shape.push(new_vertex);                                 
*vertex_buffer = glium::VertexBuffer::dynamic(display, &shape).unwrap(); //update the vertex buffer
}

fn main() {

let event_loop = glutin::event_loop::EventLoop::new();
let wb = glutin::window::WindowBuilder::new()
                            .with_inner_size(glium::glutin::dpi::LogicalSize::new(640.0, 480.0))
                            .with_title("Paint");
let cb = glutin::ContextBuilder::new();
let display = glium::Display::new(wb, cb, &event_loop).unwrap();

let mut drawing = false;
implement_vertex!(Vertex, position, in_color);

let mut shape:Vec<Vertex> = Vec::new();

let mut vertex_buffer = glium::VertexBuffer::dynamic(&display, &shape).unwrap();
let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);

let vertex_shader_src = r#"
    #version 460

    out vec4 v_in_color;
    in vec2 position;
    in vec4 in_color;

    void main() {
        v_in_color = in_color;
        gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    }
"#;

let fragment_shader_src = r#"
    #version 460

    out vec4 color;
    in vec4 v_in_color;

    void main() {
        color = v_in_color;
    }
"#;

let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
let background:[f32; 4] = [0f32, 0f32, 0f32, 1f32];  // black opaque background
let line_color:[f32; 4] = [1f32, 0f32, 0f32, 0f32];  // red line color
let draw_parameters = DrawParameters { line_width: Some(3.0), ..DrawParameters::default()}; // 3 px line width

event_loop.run(move |event, _, control_flow| {
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
       
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            glutin::event::WindowEvent::CursorMoved {device_id: _ , position, ..} => {
                    if drawing {
                        add_vertex(&display, &mut shape, position, line_color, &mut vertex_buffer);
                    } else {
                        add_vertex(&display, &mut shape, position, background, &mut vertex_buffer);
                    }
            }
            glutin::event::WindowEvent::MouseInput { device_id, state, button, ..} => {
                match state {
                    ElementState::Pressed =>  {
                                               println!("Mouse pressed: dev_id:{:?}, button:{:?}", device_id, button);
                                               drawing = true;
                                            }
                    ElementState::Released =>  {
                                               println!("Mouse releassed: dev_id:{:?}, button:{:?}", device_id, button);
                                               drawing = false;
                                            } 
                }
            }
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }

    let mut target = display.draw();
    target.clear_color(background[0], background[1], background[2], background[3]);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &draw_parameters).unwrap();
    target.finish().unwrap();
});
}