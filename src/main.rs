extern crate cgmath;
extern crate sdl2;
extern crate gl;

#[macro_use]
extern crate quick_error;

use sdl2::event::Event;

mod error;
mod shader;
mod geometry;
mod field;

use shader::{ Shader };
use geometry::{ Geometry, Vertex };
use field::{ Field, isosurface };

static VERTEX_DATA: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5, 0.0],
        normal: [0.0, 0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        normal: [0.5, -0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        normal: [-0.0, -0.5, 0.5],
        uv: [0.0, 0.0],
    },
];

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("universe", 1280, 720)
        //.fullscreen_desktop()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .present_vsync()
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    canvas.window().gl_set_context_to_current().unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    let shader = Shader::load("base").unwrap();
    //let geometry = Geometry::from(&VERTEX_DATA);
    let mut t = 0.0;

    'main: loop {

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        let geometry = isosurface(&move |x: f64, y: f64, z: f64| f64::sin(x+t)+f64::sin(y)+f64::sin(z));
        t += 0.1;

        shader.select();
        geometry.draw();
        canvas.present();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }
    }
}
