extern crate cgmath;
extern crate sdl2;
extern crate gl;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quick_error;

use sdl2::event::Event;

mod error;
mod shader;
mod geometry;
mod isosurface;
mod octree;
mod worker;
mod reference_frame;

use crate::shader::{Shader, Uniform};
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, Deg};
use gl::types::*;
use crate::octree::{Octree};
use crate::reference_frame::ReferenceFrame;

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
        .window("octree + isosurface", 768, 768)
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
    let mut t: f32 = 10.0;

    let r_universe = ReferenceFrame::privileged();
    let r_solar_system = ReferenceFrame::new("Solar System", &r_universe);
    let r_orbit = ReferenceFrame::new("Orbit", &r_solar_system);
    let r_planet = ReferenceFrame::new("Planet", &r_orbit);
    let r_ship = ReferenceFrame::new("Ship", &r_orbit);

    shader.select();

    let scalar_field = |x: f64, y: f64, z: f64| ((((z * 3.0).cos() + x+y) * ((y*6.0).cos() + 1.1) * 300.0).cos() * 0.0003 + (((x * 3.0).cos() + y+z) * ((z*6.0).cos() + 1.1) * 250.0).cos() * 0.001 + (((y * 3.0).cos() + z+x) * ((x*6.0).cos() + 1.1) * 200.0).cos() * 0.0004).abs() + (x*300.0).cos() * 0.0001 + x.powi(2) + y.powi(2) + z.powi(2)  - 0.248;
    let mut octree = Octree::new(scalar_field);

    let mut target_x: f64;
    let target_y: f64 = 0.0;
    let mut target_z: f64;

    'main: loop {
        /*let geometry = {
            let field =
                |x: f64, y: f64, z: f64| x.powi(2) + y.powi(2) + z.powi(2) - 0.25;
            let transform = |x: f64, y: f64, z: f64| field(x, y, z);
            Geometry::isosurface(&transform)
        };*/

        target_x = (f64::from(t) / 57.2958).cos() * (0.625 - f64::from(t/10.0).cos() * 0.125);
        target_z = (f64::from(t) / 57.2958).sin() * (0.625 - f64::from(t/10.0).cos() * 0.125);

        octree.walk(&|node, info, path, level, x, y, z| {
            //println!("{{ level: {}, x: {}, y: {}, z: {} }}", level, x, y, z);
            let inc = 0.5 / f64::from(1 << level);
            if level < 12 &&
                target_x + 4.0 * inc >= x - inc && target_x - 4.0 * inc <= x + inc &&
                target_y + 4.0 * inc >= y - inc && target_y - 4.0 * inc <= y + inc &&
                target_z + 4.0 * inc >= z - inc && target_z - 4.0 * inc <= z + inc {
                node.create_children(info, path, level, x, y, z);
            } else {
                node.destroy_children(info, path, level, x, y, z);
            }
        });

        unsafe {
            gl::Enable(gl::CULL_FACE);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            let proj: Matrix4<GLfloat> = cgmath::perspective(Deg(90.0), 1.0/1.0, 0.01, 1e20);
            gl::UniformMatrix4fv(Uniform::Projection as GLint, 1, gl::FALSE, proj.as_ptr());
            /*gl::UniformMatrix4fv(Uniform::ModelView as GLint,
                                 1,
                                 gl::FALSE,
                                 model_view.as_ptr());*/
        }
        //t -= 0.002 * (2.0 - (t/10.0).cos()).powi(2);

        t *= 1.02;

        r_planet.set(Matrix4::from_scale(60.0 * t as f64));
        r_ship.set(Matrix4::from_translation(Vector3::new(0.0, 0.0, 60.0 * t as f64)));

        /*geometry.draw();*/

        println!("{:?}", ReferenceFrame::transform(&r_planet, &r_ship).unwrap());

        let model_view: Matrix4<GLfloat> = ReferenceFrame::transform(&r_planet, &r_ship).unwrap().cast();
        octree.draw(model_view);

        r_planet.set(Matrix4::from_scale(0.01));
        r_ship.set(Matrix4::from_translation(Vector3::new(0.0, 0.005, 0.02)));
        let model_view: Matrix4<GLfloat> = ReferenceFrame::transform(&r_planet, &r_ship).unwrap().cast();
        octree.draw(model_view);

        canvas.present();
        octree.update();

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
