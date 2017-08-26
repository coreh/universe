extern crate cgmath;
extern crate sdl2;
extern crate gl;

#[macro_use]
extern crate quick_error;

use sdl2::event::Event;

mod error;
mod shader;
mod geometry;
mod isosurface;

use shader::{Shader, Uniform};
use geometry::Geometry;
use isosurface::Isosurface;
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, Deg};
use gl::types::*;

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
    let mut t: f32 = 0.0;

    shader.select();

    'main: loop {
        let geometry = {
            let field =
                |x: f64, y: f64, z: f64| x.powi(2) + y.powi(2) + z.powi(2) - 1.0;
            let transform = |x: f64, y: f64, z: f64| field(x, y, z);
            Geometry::isosurface(&transform)
        };

        unsafe {
            //gl::Enable(gl::CULL_FACE);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            let proj: Matrix4<GLfloat> = cgmath::perspective(Deg(90.0), 1.0, 0.1, 1000.0);
            let model_view: Matrix4<GLfloat> =
                Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0)) *
                Matrix4::from_angle_x(Deg(23.0)) *
                Matrix4::from_angle_y(Deg(23.0 + t / 10.0)) *
                Matrix4::from_translation(Vector3::new(-0.5, -0.5, -0.5));
            gl::UniformMatrix4fv(Uniform::Projection as GLint, 1, gl::FALSE, proj.as_ptr());
            gl::UniformMatrix4fv(Uniform::ModelView as GLint,
                                 1,
                                 gl::FALSE,
                                 model_view.as_ptr());
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
