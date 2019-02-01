#![feature(trait_alias)]

// Macros first.
#[macro_use]
mod name;

pub mod gl;

pub mod enums;
pub mod symbols;
pub mod traits;

pub mod constants;
pub use constants::*;

pub mod shader;
pub use shader::*;

pub struct GlTyped {
    gl: gl::Gl,
}

impl GlTyped {
    #[inline]
    pub unsafe fn create_shader<K: traits::ShaderKind>(
        &self,
        kind: K,
    ) -> Option<Shader<K, symbols::Uncompiled>> {
        ShaderName::from_raw(self.gl.CreateShader(kind.into() as u32))
            .map(|name| Shader::from_raw_parts(kind, name, symbols::Uncompiled))
    }

    #[inline]
    pub unsafe fn delete_shader<K, S>(&self, shader: Shader<K, S>) {
        let (_, name, _) = shader.into_raw_parts();
        self.gl.DeleteShader(name.into_raw());
    }

    /// Sets the shader status to [CompileStatus::Uncompiled].
    #[inline]
    pub unsafe fn compile_shader<K>(&self, shader: &mut Shader<K, enums::CompileStatus>) {
        self.gl.CompileShader(shader.name().as_u32());
        *shader.status_mut() = enums::CompileStatus::Uncompiled;
    }

    #[inline]
    pub unsafe fn compile_shader_move<K, S: traits::CompileStatus>(
        &self,
        shader: Shader<K, S>,
    ) -> Shader<K, symbols::Unknown> {
        let (kind, name, _) = shader.into_raw_parts();
        self.gl.CompileShader(name.as_u32());
        Shader::from_raw_parts(kind, name, symbols::Unknown)
    }

    /// Queries and sets the shader status.
    #[inline]
    pub unsafe fn check_shader_status<K>(&self, shader: &mut Shader<K, enums::CompileStatus>) {
        let mut status = ::std::mem::uninitialized();
        self.get_shaderiv(shader, symbols::CompileStatus, &mut status);
        *shader.status_mut() = status;
    }

    /// Turn compile-time unknown shader status into run-time known shader
    /// status.
    #[inline]
    pub unsafe fn check_shader_status_move<K>(
        &self,
        shader: Shader<K, symbols::Unknown>,
    ) -> Shader<K, enums::CompileStatus> {
        let mut status = ::std::mem::uninitialized();
        self.get_shaderiv(&shader, symbols::CompileStatus, &mut status);
        let (kind, name, _) = shader.into_raw_parts();
        Shader::from_raw_parts(kind, name, status)
    }

    #[inline]
    pub unsafe fn get_shaderiv<
        K,
        S,
        P: traits::GetShaderivParam,
        V: traits::GetShaderivValue<Param = P>,
    >(
        &self,
        shader: &Shader<K, S>,
        pname: P,
        pvalue: &mut V,
    ) {
        self.gl.GetShaderiv(
            shader.name().as_u32(),
            pname.into() as u32,
            pvalue.as_i32_mut() as *mut i32,
        );
    }

    #[inline]
    pub unsafe fn get_shader_info_log<K, S>(
        &self,
        shader: &Shader<K, S>,
        length: &mut i32,
        buffer: &mut [u8],
    ) {
        self.gl.GetShaderInfoLog(
            shader.name().as_u32(),
            buffer.len() as i32,
            length,
            buffer.as_mut_ptr() as *mut i8,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_time_shader_fun() {
        use enums::CompileStatus;
        use symbols::{Compiled, Uncompiled, Unknown, Vertex};
        unsafe {
            let gl: GlTyped = std::mem::zeroed();
            let vs: Shader<Vertex, Uncompiled> = gl.create_shader(VERTEX_SHADER).unwrap();
            let vs: Shader<Vertex, Unknown> = gl.compile_shader_move(vs);
            let vs: Shader<Vertex, CompileStatus> = gl.check_shader_status_move(vs);
            match vs.into_compiled() {
                Ok(compiled) => {
                    let _: Shader<Vertex, Compiled> = compiled;
                }
                Err(uncompiled) => {
                    let _: Shader<Vertex, Uncompiled> = uncompiled;
                }
            }
        }
    }
}
