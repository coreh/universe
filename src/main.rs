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

use shader::{ Shader, Uniform };
use geometry::{ Geometry, Vertex };
use field::{ Field, isosurface };
use cgmath::prelude::*;
use cgmath::{ Vector3, Matrix4, Deg, Rotation3 };
use gl::types::*;

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
        .window("isosurface", 512, 512)
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

    shader.select();

    'main: loop {
        let geometry = isosurface(&move |x: f64, y: f64, z: f64| ((16.0-x).powi(2) + (16.0-y).powi(2) + (16.0-z).powi(2)).sqrt() - 10.0);// + (x + f64::from(t/10.0)).cos() + (y + f64::from(t/10.0)).cos() + (z + f64::from(t/10.0)).cos());

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            let proj: Matrix4<GLfloat> = cgmath::perspective(Deg(90.0), 1.0, 0.1, 1000.0);
            let model_view: Matrix4<GLfloat> = Matrix4::from_translation(Vector3::new(0.0, 0.0, -30.0)) * Matrix4::from_angle_y(Deg(23.0)) * Matrix4::from_angle_x(Deg(23.0 + t))  * Matrix4::from_translation(Vector3::new(-16.0, -16.0, -16.0));
            gl::UniformMatrix4fv(Uniform::Projection as GLint, 1, gl::FALSE, proj.as_ptr());
            gl::UniformMatrix4fv(Uniform::ModelView as GLint, 1, gl::FALSE, model_view.as_ptr());
        }
        t += 1.0;

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
