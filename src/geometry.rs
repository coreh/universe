use std;
use gl;
use gl::types::*;

use shader::Attribute;

pub mod traits {
    pub trait Vertex {
        fn position_offset() -> Option<usize>;
        fn normal_offset() -> Option<usize>;
        fn uv_offset() -> Option<usize>;
    }
}

#[repr(C)]
pub struct Vertex {
    pub position: [GLfloat; 3],
    pub normal: [GLfloat; 3],
    pub uv: [GLfloat; 2],
}

impl traits::Vertex for Vertex {
    fn position_offset() -> Option<usize> {
        Some(0)
    }
    fn normal_offset() -> Option<usize> {
        Some(std::mem::align_of::<GLfloat>() * 3)
    }
    fn uv_offset() -> Option<usize> {
        Some(std::mem::align_of::<GLfloat>() * 6)
    }
}

pub struct Geometry {
    pub(crate) vao: GLuint,
    pub(crate) vbo: GLuint,
    pub(crate) num_vertices: GLsizei,
}

impl Geometry {
    pub fn from<V: traits::Vertex>(data: &[V]) -> Geometry {
        let mut vao: GLuint = gl::NONE;
        let mut vbo: GLuint = gl::NONE;
        let num_vertices: GLsizei = data.len() as GLsizei;

        unsafe {
            // TODO: pool vao's across Geometrys with the same Vertex type
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<V>()) as GLsizeiptr,
                           std::mem::transmute(&data[0]),
                           gl::STATIC_DRAW);


            if let Some(position_offset) = V::position_offset() {
                gl::EnableVertexAttribArray(Attribute::Position as GLuint);
                gl::VertexAttribPointer(Attribute::Position as GLuint,
                                        3,
                                        gl::FLOAT,
                                        gl::FALSE,
                                        std::mem::size_of::<V>() as GLint,
                                        position_offset as *const GLvoid);
            }

            if let Some(normal_offset) = V::normal_offset() {
                gl::EnableVertexAttribArray(Attribute::Normal as GLuint);
                gl::VertexAttribPointer(Attribute::Normal as GLuint,
                                        3,
                                        gl::FLOAT,
                                        gl::FALSE,
                                        std::mem::size_of::<V>() as GLint,
                                        normal_offset as *const GLvoid);
            }

            if let Some(uv_offset) = V::uv_offset() {
                gl::EnableVertexAttribArray(Attribute::UV as GLuint);
                gl::VertexAttribPointer(Attribute::UV as GLuint,
                                        2,
                                        gl::FLOAT,
                                        gl::FALSE,
                                        std::mem::size_of::<V>() as GLint,
                                        uv_offset as *const GLvoid);
            }

            gl::BindVertexArray(gl::NONE);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl::NONE);

        }

        Geometry {
            vao,
            vbo,
            num_vertices,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
