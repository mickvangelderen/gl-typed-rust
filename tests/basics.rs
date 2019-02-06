use gl_typed::enums::*;
use gl_typed::*;

pub struct Basics {
    pub events_loop: glutin::EventsLoop,
    pub gl_window: glutin::GlWindow,
    pub gl: gl_typed::GlTyped,
}

impl Basics {
    fn new() -> Self {
        let events_loop = glutin::EventsLoop::new();
        let gl_window = glutin::GlWindow::new(
            glutin::WindowBuilder::new()
                .with_title("gl-typed integration test")
                .with_visibility(false)
                .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0)),
            glutin::ContextBuilder::new()
                .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
                .with_gl_profile(glutin::GlProfile::Core),
            &events_loop,
        )
        .unwrap();

        unsafe {
            glutin::GlContext::make_current(&gl_window).unwrap();
        }

        let gl = unsafe {
            gl_typed::GlTyped::load_with(|symbol| {
                glutin::GlContext::get_proc_address(&gl_window, symbol) as *const std::ffi::c_void
            })
        };

        Basics {
            events_loop,
            gl_window,
            gl,
        }
    }
}

unsafe fn get_shader_info_log(gl: &GlTyped, name: &ShaderName) -> String {
    let buffer = {
        let capacity = {
            let mut capacity: i32 = std::mem::uninitialized();
            gl.get_shaderiv(&name, INFO_LOG_LENGTH, &mut capacity);
            assert!(capacity > 0);
            capacity as usize
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
        buffer.set_len(capacity);
        let mut length: i32 = std::mem::uninitialized();
        gl.get_shader_info_log(&name, &mut length, &mut buffer[..]);
        assert!(length > 0);
        let length = length as usize;
        assert!(length <= capacity);
        buffer.set_len(length);
        buffer
    };

    String::from_utf8(buffer).expect("Shader info log contains invalid UTF-8.")
}

#[test]
fn create_a_shader() {
    let Basics {
        events_loop: _events_loop,
        gl_window: _gl_window,
        gl,
    } = Basics::new();

    let vs = unsafe {
        let mut name: ShaderName = gl.create_shader(VERTEX_SHADER).unwrap();
        gl.shader_source(&mut name, &[BASIC_VERTEX_SHADER.as_bytes()]);
        gl.compile_shader(&mut name);
        let mut status: RawShaderCompileStatus = std::mem::uninitialized();
        gl.get_shaderiv(&name, COMPILE_STATUS, &mut status);
        match status.into() {
            ShaderCompileStatus::Uncompiled => {
                eprintln!("{}", get_shader_info_log(&gl, &name));
                panic!("Failed to compile shader.");
            }
            ShaderCompileStatus::Compiled => name,
        }
    };

    let fs = unsafe {
        let mut name: ShaderName = gl.create_shader(FRAGMENT_SHADER).unwrap();
        gl.shader_source(&mut name, &[BASIC_FRAGMENT_SHADER.as_bytes()]);
        gl.compile_shader(&mut name);
        let mut status: RawShaderCompileStatus = std::mem::uninitialized();
        gl.get_shaderiv(&name, COMPILE_STATUS, &mut status);
        match status.into() {
            ShaderCompileStatus::Uncompiled => {
                eprintln!("{}", get_shader_info_log(&gl, &name));
                panic!("Failed to compile shader.");
            }
            ShaderCompileStatus::Compiled => name,
        }
    };

    let p = unsafe {
        let mut name = gl.create_program().unwrap();
        gl.attach_shader(&mut name, &vs);
        gl.attach_shader(&mut name, &fs);
        gl.link_program(&mut name);
        let mut status: RawProgramLinkStatus = std::mem::uninitialized();
        gl.get_programiv(&name, LINK_STATUS, &mut status);
        assert_eq!(status, ProgramLinkStatus::Linked.into());
        name
    };
}

const BASIC_VERTEX_SHADER: &'static str = r##"
#version 330 core
in vec3 vs_pos;

out vec4 vs_color;

void main()
{
    gl_Position = vec4(vs_pos, 1.0);
    vs_color = vec4(vs_pos.x, vs_pos.y, 0.5, 1.0);
}
"##;

const BASIC_FRAGMENT_SHADER: &'static str = r##"
#version 330 core
in vec4 vs_color;

out vec4 fs_color;

void main()
{
    fs_color = vs_color;
}
"##;
