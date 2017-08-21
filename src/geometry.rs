use std;
use gl;
use gl::types::*;

use shader::Attribute;

#[repr(C)]
pub struct Vertex {
    pub position: [GLfloat; 3],
    pub normal: [GLfloat; 3],
}

pub struct Geometry {
    pub(crate) vao: GLuint,
    pub(crate) vbo: GLuint,
}

impl Geometry {
    pub fn from(data: &[Vertex]) -> Geometry {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                           std::mem::transmute(&data[0]),
                           gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(Attribute::position as GLuint);
            gl::EnableVertexAttribArray(Attribute::normal as GLuint);

            gl::VertexAttribPointer(Attribute::position as GLuint,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    std::mem::size_of::<Vertex>() as GLint,
                                    (0 * std::mem::size_of::<GLfloat>()) as *const GLvoid);

            gl::VertexAttribPointer(Attribute::normal as GLuint,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    std::mem::size_of::<Vertex>() as GLint,
                                    (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid);
        }

        Geometry { vao, vbo }
    }
}
