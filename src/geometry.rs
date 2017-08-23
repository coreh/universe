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
#[derive(Clone)]
pub struct Vertex {
    pub position: [GLfloat; 3],
    pub normal: [GLfloat; 3],
    pub uv: [GLfloat; 2],
}

impl traits::Vertex for Vertex {
    #[inline]
    fn position_offset() -> Option<usize> {
        Some(0)
    }
    #[inline]
    fn normal_offset() -> Option<usize> {
        Some(std::mem::align_of::<GLfloat>() * 3)
    }
    #[inline]
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

        if num_vertices > 0 {

            unsafe {
                // TODO: pool vao's across Geometrys with the same Vertex type
                gl::CreateVertexArrays(1, &mut vao);
                gl::CreateBuffers(1, &mut vbo);

                gl::NamedBufferData(vbo,
                                    (data.len() * std::mem::size_of::<V>()) as GLsizeiptr,
                                    std::mem::transmute(&data[0]),
                                    gl::STATIC_DRAW);

                gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, std::mem::size_of::<V>() as GLsizei);

                if let Some(position_offset) = V::position_offset() {
                    gl::VertexArrayAttribBinding(vao, Attribute::Position as GLuint, 0);
                    gl::EnableVertexArrayAttrib(vao, Attribute::Position as GLuint);
                    gl::VertexArrayAttribFormat(vao,
                                                Attribute::Position as GLuint,
                                                3,
                                                gl::FLOAT,
                                                gl::FALSE,
                                                position_offset as GLuint);
                }

                if let Some(normal_offset) = V::normal_offset() {
                    gl::EnableVertexArrayAttrib(vao, Attribute::Normal as GLuint);
                    gl::VertexArrayAttribFormat(vao,
                                                Attribute::Normal as GLuint,
                                                3,
                                                gl::FLOAT,
                                                gl::FALSE,
                                                normal_offset as GLuint);
                    gl::VertexArrayAttribBinding(vao, Attribute::Normal as GLuint, 0);
                }

                if let Some(uv_offset) = V::uv_offset() {
                    gl::EnableVertexArrayAttrib(vao, Attribute::UV as GLuint);
                    gl::VertexArrayAttribFormat(vao,
                                                Attribute::UV as GLuint,
                                                2,
                                                gl::FLOAT,
                                                gl::FALSE,
                                                uv_offset as GLuint);
                    gl::VertexArrayAttribBinding(vao, Attribute::UV as GLuint, 0);
                }
            }
        }

        Geometry {
            vao,
            vbo,
            num_vertices,
        }
    }

    pub fn draw(&self) {
        if self.num_vertices == 0 {
            return;
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices);
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
