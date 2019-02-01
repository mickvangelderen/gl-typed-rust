// Macros first.
#[macro_use]
mod name;

pub mod gl;

pub mod shader;
pub use shader::*;

pub struct GlTyped {
    gl: gl::Gl,
}

impl GlTyped {
    #[inline]
    pub unsafe fn create_shader<K: Into<ShaderKind> + Copy>(
        &self,
        kind: K,
    ) -> Option<shader::generic::Shader<K>> {
        ShaderName::from_raw(self.gl.CreateShader(kind.into() as u32))
            .map(|name| shader::generic::Shader::from_raw_parts(kind, name))
    }

    #[inline]
    pub unsafe fn delete_shader<N: Into<ShaderName>>(&self, name: N) {
        self.gl.DeleteShader(name.into().into_raw());
    }
}
