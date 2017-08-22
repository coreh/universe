use std;
use sdl2;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Nul {
            description("NUL character found on shader source")
            from(std::ffi::NulError)
        }

        Utf8(err: std::string::FromUtf8Error) {
            from()
            cause(err)
            description(err.description())
        }

        Io(err: std::io::Error) {
            from()
            cause(err)
            description(err.description())
        }

        GLSL(log: String) {
            description(log)
        }

        SDL(err: sdl2::Error) {
            from()
            cause(err)
            description(err.description())
        }

        Format(err: std::fmt::Error) {
            from()
            cause(err)
            description(err.description())
        }
    }
}
