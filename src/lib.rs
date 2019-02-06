pub mod gl;

pub mod array;
pub use array::*;

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
    pub unsafe fn load_with<F>(f: F) -> Self
    where
        F: FnMut(&'static str) -> *const std::os::raw::c_void,
    {
        GlTyped {
            gl: gl::Gl::load_with(f),
        }
    }

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
    pub unsafe fn get_shaderiv<P>(&self, name: &ShaderName, pname: P, pvalue: &mut P::Value)
    where
        P: traits::GetShaderivParam,
    {
        self.gl.GetShaderiv(
            name.as_u32(),
            pname.into() as u32,
            traits::Transmute::as_mut(pvalue),
        );
    }

    #[inline]
    pub unsafe fn shader_source<
        's,
        A: Array<Item = &'s [u8]> + ArrayMap<*const i8> + ArrayMap<i32> + ?Sized,
    >(
        &self,
        shader: &mut ShaderName,
        sources: &A,
    ) {
        let pointers = sources.map(|s| s.as_ptr() as *const i8);
        let lengths = sources.map(|s| s.len() as i32);
        assert_eq!(pointers.len(), lengths.len());
        self.gl.ShaderSource(
            shader.as_u32(),
            pointers.len() as i32,
            pointers.as_ptr(),
            lengths.as_ptr(),
        )
    }

    #[inline]
    pub unsafe fn get_shader_info_log(
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
