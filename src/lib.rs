pub mod gl;

pub mod enums;
pub mod symbols;
pub mod traits;

pub mod names;
pub use names::*;

pub mod constants;
pub use constants::*;

pub struct GlTyped {
    gl: gl::Gl,
}

impl GlTyped {
    #[inline]
    pub unsafe fn create_shader<K>(&self, kind: K) -> Option<ShaderName>
    where
        K: Into<enums::ShaderKind>,
    {
        ShaderName::from_raw(self.gl.CreateShader(kind.into() as u32))
    }

    #[inline]
    pub unsafe fn delete_shader(&self, name: ShaderName) {
        self.gl.DeleteShader(name.into_raw());
    }

    #[inline]
    pub unsafe fn compile_shader(&self, name: &mut ShaderName) {
        self.gl.CompileShader(name.as_u32());
    }

    #[inline]
    pub unsafe fn get_shaderiv<
        P: Into<enums::GetShaderivParam>,
        V: traits::GetShaderivValue<Param = P>,
    >(
        &self,
        name: &ShaderName,
        pname: P,
        pvalue: &mut V,
    ) {
        self.gl.GetShaderiv(
            name.as_u32(),
            pname.into() as u32,
            pvalue.as_mut(),
        );
    }

    #[inline]
    pub unsafe fn get_shader_info_log<K, S>(
        &self,
        name: &ShaderName,
        length: &mut i32,
        buffer: &mut [u8],
    ) {
        self.gl.GetShaderInfoLog(
            name.as_u32(),
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
        unsafe {
            let gl: GlTyped = std::mem::zeroed();
            let mut vs: ShaderName = gl.create_shader(VERTEX_SHADER).unwrap();
            gl.compile_shader(&mut vs);
            let mut status: enums::RawShaderCompileStatus = std::mem::uninitialized();
            gl.get_shaderiv(&vs, COMPILE_STATUS, &mut status);
            if status != enums::ShaderCompileStatus::Compiled.into() {
                panic!("Boom");
            }
        }
    }

    #[test]
    fn unknown_shader_type() {
        use std::mem;
        use symbols::{Uncompiled, Unknown};

        unsafe {
            let gl: GlTyped = std::mem::zeroed();
            let s: Shader<Unknown, Uncompiled> = gl.create_shader(VERTEX_SHADER).unwrap();
            assert_eq!(mem::size_of_val(&s), 4);
        }
    }
}
