pub mod gl;

// Private.
#[macro_use]
mod name;

// Public.
pub mod shader;
pub use shader::*;

pub struct GlTyped {
    gl: gl::Gl,
}

impl GlTyped {
    #[inline]
    unsafe fn create_shader_any(&self, kind: ShaderKind) -> Option<ShaderName> {
        ShaderName::from_u32(self.gl.CreateShader(kind as u32))
    }

    #[inline]
    pub unsafe fn create_shader<S: Shader>(&self, kind: S::Kind) -> Option<S> {
        self.create_shader_any(kind.into())
            .map(|name| S::from_parts(kind, name))
    }

    #[inline]
    unsafe fn delete_shader_any(&self, name: ShaderName) {
        self.gl.DeleteShader(name.as_u32())
    }

    #[inline]
    pub unsafe fn delete_shader<S: Shader>(&self, shader: S) {
        let (_kind, name) = shader.into_parts();
        self.delete_shader_any(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_generic_shader() {
        unsafe {
            let gl: GlTyped = ::std::mem::zeroed();
            let _shader: ShaderKindName = gl.create_shader(ShaderKind::Vertex).unwrap();
        }
    }

    #[test]
    fn create_specialized_shader() {
        unsafe {
            let gl: GlTyped = ::std::mem::zeroed();
            let _shader: VertexShaderName = gl.create_shader(VERTEX_SHADER).unwrap();
        }
    }
}
