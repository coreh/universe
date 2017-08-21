extern crate sdl2;
extern crate gl;

#[macro_use]
extern crate quick_error;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use gl::types::*;

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;

mod error;
mod shader;
mod geometry;

use error::Error;
use shader::{ Shader, Attribute };
use geometry::{ Geometry, Vertex };

static VERTEX_DATA: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5, 0.0],
        normal: [0.0, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        normal: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        normal: [-0.0, -0.5, 0.5],
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

    canvas.window().gl_set_context_to_current();
    //canvas.set_logical_size(320, 240).unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u8 = 0;

    let shader = Shader::load("base").unwrap();
    let geometry = Geometry::from(&VERTEX_DATA);

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut vbo_color: GLuint = 0;

    unsafe {
        //gl::Enable(gl::FRAMEBUFFER_SRGB);
        gl::UseProgram(shader.program);
    }

    loop {
        if (i == 255) {
            i = 0;
            j += 1;
        }
        if (j == 255) {
            j = 0;
            k += 1;
        }
        if (k == 255) {
            k = 0;
            j += 1;
        }
        i += 1;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        canvas.present();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break;
                }
                _ => {}
            }
        }
    }
}
