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

#[test]
fn create_a_shader() {
    let Basics {
        events_loop: _events_loop,
        gl_window: _gl_window,
        gl,
    } = Basics::new();

    use gl_typed::enums::*;
    use gl_typed::*;

    unsafe {
        let mut vs: ShaderName = gl.create_shader(VERTEX_SHADER).unwrap();
        let src = String::from("#version 420\n");
        gl.shader_source(&mut vs, &[src.as_bytes()]);
        gl.compile_shader(&mut vs);
        let mut status: RawShaderCompileStatus = std::mem::uninitialized();
        gl.get_shaderiv(&vs, COMPILE_STATUS, &mut status);
        assert_eq!(status, ShaderCompileStatus::Compiled.into())
    }
}
