/*
    Rocketman
    2021-04-24
    test main
*/

extern crate gl;
extern crate gl_loader;
//use vulkano::instance::Instance;
//use vulkano::instance::InstanceExtensions;
//use vulkano::instance::PhysicalDevice;

//use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
//use winit::event_loop::{EventLoop, ControlFlow};
//use winit::window::WindowBuilder;

use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use std::ffi::CStr;

const WINDOW_TITLE: &'static str = "Rocket";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    println!("Hello, world!");

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    // load opengl
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
    
    // render loop
    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                return;
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });

    gl_loader::end_gl();

    println!("End");
}
