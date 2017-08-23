use std;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;

use gl;
use gl::types::*;

use error::Error;

pub enum Attribute {
    Position = 0,
    Normal = 1,
    UV = 2,
}

pub enum Uniform {
    ModelView = 0,
    Projection = 1,
}

pub struct Shader {
    pub(crate) program: GLuint,
}

impl Shader {
    pub fn load(path: &str) -> Result<Shader, Error> {
        let mut vert_path = String::new();
        let mut frag_path = String::new();
        write!(&mut vert_path, "{}.vert", path)?;
        write!(&mut frag_path, "{}.frag", path)?;
        let vert = Self::compile(vert_path.as_str(), gl::VERTEX_SHADER)?;
        let frag = Self::compile(frag_path.as_str(), gl::FRAGMENT_SHADER)?;
        let program = Self::link(vert, frag)?;
        Ok(Shader { program })
    }

    fn compile(path: &str, ty: GLenum) -> Result<GLuint, Error> {

        let mut src = String::from("#version 430\n");
        writeln!(&mut src, "#define ATTRIB_POSITION {}", Attribute::Position as GLuint)?;
        writeln!(&mut src, "#define ATTRIB_NORMAL {}", Attribute::Normal as GLuint)?;
        writeln!(&mut src, "#define ATTRIB_UV {}", Attribute::UV as GLuint)?;
        writeln!(&mut src, "#define UNIFORM_MODEL_VIEW {}", Uniform::ModelView as GLuint)?;
        writeln!(&mut src, "#define UNIFORM_PROJECTION {}", Uniform::Projection as GLuint)?;

        let mut file = File::open(path)?;
        file.read_to_string(&mut src)?;

        let shader;
        unsafe {
            shader = gl::CreateShader(ty);

            // Attempt to compile the shader
            let c_str = std::ffi::CString::new(src.as_bytes())?;
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader,
                                     len,
                                     std::ptr::null_mut(),
                                     buf.as_mut_ptr() as *mut GLchar);

                return Err(Error::GLSL(String::from_utf8(buf)?));
            }
        }
        Ok(shader)
    }

    fn link(vs: GLuint, fs: GLuint) -> Result<GLuint, Error> {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program,
                                      len,
                                      std::ptr::null_mut(),
                                      buf.as_mut_ptr() as *mut GLchar);
                return Err(Error::GLSL(String::from_utf8(buf)?));
            }

            gl::DetachShader(program, vs);
            gl::DetachShader(program, fs);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            Ok(program)
        }
    }

    pub fn select(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}
