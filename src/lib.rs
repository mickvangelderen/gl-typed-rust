pub mod gl;

pub mod array;
pub use array::*;

pub mod enums;
pub mod symbols;
pub mod traits;

pub mod names;
pub use names::*;

#[rustfmt::skip]
pub mod constants;
pub use constants::*;

use std::ffi::CStr;
use std::os::raw::*;

pub struct Gl {
    gl: gl::Gl,
}

impl Gl {
    #[inline]
    pub unsafe fn load_with<F>(f: F) -> Self
    where
        F: FnMut(&'static str) -> *const std::os::raw::c_void,
    {
        Gl {
            gl: gl::Gl::load_with(f),
        }
    }

    #[inline]
    pub unsafe fn get_string<P>(&self, name: P) -> &'static str
    where
        P: Into<enums::GetStringParam>,
    {
        // NOTE(ZERO): We have to count the string length at some point. Do
        // it here for ergonomics.
        // NOTE(SAFETY): Specification says the returned string must be a UTF8
        // encoded, null-terminated static string.
        std::str::from_utf8_unchecked(
            CStr::from_ptr(self.gl.GetString(name.into() as u32) as *const c_char).to_bytes(),
        )
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

    #[inline]
    pub unsafe fn create_program(&self) -> Option<ProgramName> {
        ProgramName::from_raw(self.gl.CreateProgram())
    }

    #[inline]
    pub unsafe fn delete_program(&self, name: ProgramName) {
        self.gl.DeleteProgram(name.into_raw());
    }

    #[inline]
    pub unsafe fn use_program(&self, program: &ProgramName) {
        self.gl.UseProgram(program.as_u32());
    }

    #[inline]
    pub unsafe fn attach_shader(&self, program: &mut ProgramName, shader: &ShaderName) {
        self.gl.AttachShader(program.as_u32(), shader.as_u32());
    }

    #[inline]
    pub unsafe fn link_program(&self, program: &mut ProgramName) {
        self.gl.LinkProgram(program.as_u32());
    }

    #[inline]
    pub unsafe fn get_programiv<P>(&self, name: &ProgramName, pname: P, pvalue: &mut P::Value)
    where
        P: traits::GetProgramivParam,
    {
        self.gl.GetProgramiv(
            name.as_u32(),
            pname.into() as u32,
            traits::Transmute::as_mut(pvalue),
        );
    }

    #[inline]
    pub unsafe fn get_program_info_log(
        &self,
        name: &ProgramName,
        length: &mut i32,
        buffer: &mut [u8],
    ) {
        self.gl.GetProgramInfoLog(
            name.as_u32(),
            buffer.len() as i32,
            length,
            buffer.as_mut_ptr() as *mut i8,
        );
    }
}
