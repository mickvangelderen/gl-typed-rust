pub mod array;
pub mod constants;
pub mod convert;
pub mod gl;
pub mod locations;
pub mod names;
pub mod num;
pub mod string;
pub mod symbols;
pub mod traits;
pub mod types;

pub use array::*;
pub use constants::*;
pub use convert::*;
pub use locations::*;
pub use names::*;
use std::ffi::CStr;
use std::os::raw::*;
pub use types::*;

pub struct Gl {
    gl: gl::Gl,
}

impl Gl {
    #[inline]
    pub unsafe fn load_with<F>(f: F) -> Self
    where
        F: FnMut(&'static str) -> *const c_void,
    {
        Gl {
            gl: gl::Gl::load_with(f),
        }
    }

    #[inline]
    pub unsafe fn get_string<P>(&self, name: P) -> &'static str
    where
        P: Into<GetStringParam>,
    {
        // NOTE(ZERO): We have to count the string length at some point. Do
        // it here for ergonomics.
        // NOTE(SAFETY): Specification says the returned string must be a UTF8
        // encoded, null-terminated static string.
        std::str::from_utf8_unchecked(
            CStr::from_ptr(self.gl.GetString(name.into() as u32) as *const c_char).to_bytes(),
        )
    }

    // Drawing.
    #[inline]
    pub unsafe fn enable<C>(&self, cap: C)
    where
        C: Into<Capability>,
    {
        self.gl.Enable(cap.into() as u32);
    }

    #[inline]
    pub unsafe fn disable<C>(&self, cap: C)
    where
        C: Into<Capability>,
    {
        self.gl.Disable(cap.into() as u32);
    }

    #[inline]
    pub unsafe fn polygon_mode<F, M>(&self, face: F, mode: M)
    where
        F: Into<PolygonModeFace>,
        M: Into<PolygonMode>,
    {
        self.gl.PolygonMode(face.into() as u32, mode.into() as u32);
    }

    #[inline]
    pub unsafe fn cull_face<F>(&self, face: F)
    where
        F: Into<CullFace>,
    {
        self.gl.CullFace(face.into() as u32);
    }

    #[inline]
    pub unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.Viewport(x, y, width, height);
    }

    #[inline]
    pub unsafe fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.ClearColor(r, g, b, a);
    }

    #[inline]
    pub unsafe fn clear(&self, flags: ClearFlags) {
        self.gl.Clear(flags.bits());
    }

    #[inline]
    pub unsafe fn draw_arrays<M>(&self, mode: M, first: usize, count: usize)
    where
        M: Into<DrawMode>,
    {
        self.gl
            .DrawArrays(mode.into() as u32, first as i32, count as i32);
    }

    #[inline]
    pub unsafe fn draw_elements<M, T>(&self, mode: M, count: usize, ty: T, offset: usize)
    where
        M: Into<DrawMode>,
        T: Into<DrawElementsType>,
    {
        self.gl.DrawElements(
            mode.into() as u32,
            count as i32,
            ty.into() as u32,
            offset as *const c_void,
        );
    }

    // Shaders.

    #[inline]
    pub unsafe fn create_shader<K>(&self, kind: K) -> Option<ShaderName>
    where
        K: Into<ShaderKind>,
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
    pub unsafe fn get_shaderiv<P>(&self, name: &ShaderName, param: P, value: &mut P::Value)
    where
        P: traits::GetShaderivParam,
    {
        self.gl
            .GetShaderiv(name.as_u32(), param.into() as u32, value.transmute_as_mut())
    }

    #[inline]
    pub unsafe fn get_shaderiv_move<P>(&self, name: &ShaderName, param: P) -> P::Value
    where
        P: traits::GetShaderivParam,
    {
        let mut value: P::Value = ::std::mem::uninitialized();
        self.get_shaderiv(name, param, &mut value);
        value
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

    /// Returns Vec<u8> because the user should decide whether or not to trust
    /// that OpenGL writes valid UTF-8 into the buffer.
    #[inline]
    pub unsafe fn get_shader_info_log_move(&self, name: &ShaderName) -> Vec<u8> {
        let mut buffer = {
            let capacity = self.get_shaderiv_move(name, INFO_LOG_LENGTH);
            assert!(capacity >= 0);
            Vec::with_capacity(capacity as usize)
        };
        let mut length = ::std::mem::uninitialized();
        self.get_shader_info_log(
            name,
            &mut length,
            ::std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer.capacity()),
        );
        assert!(length >= 0 && length <= buffer.capacity() as i32);
        buffer.set_len(length as usize);
        buffer
    }

    #[inline]
    pub unsafe fn shader_source<
        's,
        A: Array<Item = &'s [u8]> + ArrayMap<*const i8> + ArrayMap<i32> + ?Sized,
    >(
        &self,
        name: &mut ShaderName,
        sources: &A,
    ) {
        let pointers = sources.map(|s| s.as_ptr() as *const c_char);
        let lengths = sources.map(|s| s.len() as i32);
        assert_eq!(pointers.len(), lengths.len());
        self.gl.ShaderSource(
            name.as_u32(),
            pointers.len() as i32,
            pointers.as_ptr(),
            lengths.as_ptr(),
        )
    }

    // Programs.

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
    pub unsafe fn get_programiv<P>(&self, name: &ProgramName, param: P, value: &mut P::Value)
    where
        P: traits::GetProgramivParam,
    {
        self.gl
            .GetProgramiv(name.as_u32(), param.into() as u32, value.transmute_as_mut());
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

    #[inline]
    pub unsafe fn get_attrib_location(
        &self,
        program_name: &ProgramName,
        attrib_name: &CStr,
    ) -> Option<AttributeLocation> {
        AttributeLocation::from_raw(
            self.gl
                .GetAttribLocation(program_name.as_u32(), attrib_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn vertex_attrib_pointer<T, N>(
        &self,
        loc: &AttributeLocation,
        size: usize,
        ty: T,
        normalized: N,
        stride: usize,
        offset: usize,
    ) where
        T: Into<VertexAttribPointerType>,
        N: Into<Bool>,
    {
        self.gl.VertexAttribPointer(
            loc.as_u32(),
            size as i32,
            ty.into() as u32,
            normalized.into() as u8,
            stride as i32,
            offset as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn get_uniform_location<T>(
        &self,
        program_name: &ProgramName,
        uniform_name: &CStr,
    ) -> Option<UniformLocation<T>> {
        UniformLocation::from_raw(
            self.gl
                .GetUniformLocation(program_name.as_u32(), uniform_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, loc: &AttributeLocation) {
        self.gl.EnableVertexAttribArray(loc.as_u32());
    }

    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, loc: &AttributeLocation) {
        self.gl.DisableVertexAttribArray(loc.as_u32());
    }

    // Textures.

    #[inline]
    pub unsafe fn gen_textures(&self, names: &mut [Option<TextureName>]) {
        self.gl
            .GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn delete_textures(&self, names: &mut [Option<TextureName>]) {
        self.gl
            .DeleteTextures(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn active_texture(&self, unit: TextureUnit) {
        self.gl.ActiveTexture(unit.as_u32());
    }

    #[inline]
    pub unsafe fn bind_texture<T, N>(&self, target: T, name: &N)
    where
        T: Into<TextureTarget>,
        N: MaybeUnbindTextureName,
    {
        self.gl.BindTexture(target.into() as u32, name.as_u32());
    }

    // FIXME: Figure out why we need the additional type bounds even though
    // TexParameteriParam already specifies P::Target to be Into<TextureTarget>
    // etc.
    #[inline]
    pub unsafe fn tex_parameter_i<P, T, V>(&self, target: T, param: P, value: V)
    where
        P: traits::TexParameteriParam,
        P::Target: Into<TextureTarget>,
        P::Value: Into<i32>,
        T: Into<P::Target>,
        V: Into<P::Value>,
    {
        self.gl.TexParameteri(
            target.into().into() as u32,
            param.into() as u32,
            Into::into(value.into()),
        )
    }

    #[inline]
    pub unsafe fn generate_mipmap<T>(&self, target: T)
    where
        T: Into<TextureTarget>,
    {
        self.gl.GenerateMipmap(target.into() as u32);
    }

    #[inline]
    pub unsafe fn tex_image_2d<T, IF, F, CF>(
        &self,
        target: T,
        mipmap_level: i32,
        internal_format: IF,
        width: i32,
        height: i32,
        format: F,
        component_format: CF,
        data: *const c_void,
    ) where
        T: Into<TextureTarget>,
        IF: Into<InternalFormat>,
        F: Into<Format>,
        CF: Into<ComponentFormat>,
    {
        self.gl.TexImage2D(
            target.into() as u32,
            mipmap_level,
            internal_format.into() as i32,
            width,
            height,
            0, // border, must be zero
            format.into() as u32,
            component_format.into() as u32,
            data,
        );
    }

    // Buffer names.

    #[inline]
    pub unsafe fn gen_buffers(&self, names: &mut [Option<BufferName>]) {
        self.gl
            .GenBuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn delete_buffers<A>(&self, names: &mut [Option<BufferName>]) {
        self.gl
            .DeleteBuffers(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn bind_buffer<T, N>(&self, target: T, name: &N)
    where
        T: Into<BufferTarget>,
        N: MaybeUnbindBufferName,
    {
        self.gl.BindBuffer(target.into() as u32, name.as_u32());
    }

    #[inline]
    pub unsafe fn buffer_data<T, D, U>(&self, target: T, data: &[D], usage: U)
    where
        T: Into<BufferTarget>,
        U: Into<BufferUsage>,
    {
        self.gl.BufferData(
            target.into() as u32,
            std::mem::size_of_val(data) as isize,
            data.as_ptr() as *const c_void,
            usage.into() as u32,
        );
    }

    // Vertex array names.

    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, names: &mut [Option<VertexArrayName>]) {
        self.gl
            .GenVertexArrays(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, names: &mut [Option<VertexArrayName>]) {
        self.gl
            .DeleteVertexArrays(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn bind_vertex_array<N>(&self, name: &N)
    where
        N: MaybeUnbindVertexArrayName,
    {
        self.gl.BindVertexArray(name.as_u32());
    }

    // Framebuffer names.

    #[inline]
    pub unsafe fn gen_framebuffers(&self, names: &mut [Option<FramebufferName>]) {
        self.gl
            .GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn delete_framebuffers(&self, names: &mut [Option<FramebufferName>]) {
        self.gl
            .GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn bind_framebuffer<T, N>(&self, target: T, name: &N)
    where
        T: Into<FramebufferTarget>,
        N: MaybeDefaultFramebufferName,
    {
        self.gl.BindFramebuffer(target.into() as u32, name.as_u32());
    }

    #[inline]
    pub unsafe fn check_framebuffer_status<T>(&self, target: T) -> UncheckedFramebufferStatus
    where
        T: Into<FramebufferTarget>,
    {
        UncheckedFramebufferStatus::transmute_from(
            self.gl.CheckFramebufferStatus(target.into() as u32),
        )
    }

    #[inline]
    pub unsafe fn framebuffer_texture_2d<FT, FA, TT>(
        &self,
        framebuffer_target: FT,
        framebuffer_attachment: FA,
        texture_target: TT,
        texture_name: &TextureName,
        mipmap_level: i32,
    ) where
        FT: Into<FramebufferTarget>,
        FA: Into<FramebufferAttachment>,
        TT: Into<TextureTarget>,
    {
        self.gl.FramebufferTexture2D(
            framebuffer_target.into() as u32,
            framebuffer_attachment.into().as_u32(),
            texture_target.into() as u32,
            texture_name.as_u32(),
            mipmap_level,
        );
    }

    // // Uniform setters.

    // #[inline]
    // pub unsafe fn uniform_1i(&self, uniform_location: &UniformLocation<i32>, value: i32) {
    //     self.gl.Uniform1i(uniform_location.as_i32(), value);
    // }

    // #[inline]
    // pub unsafe fn uniform_2i<T: AsRef<[i32; 2]>>(
    //     uniform_location: &UniformLocation<[i32; 2]>,
    //     value: T,
    // ) {
    //     let value = value.as_ref();
    //     self.gl.Uniform2i(uniform_location.as_i32(), value[0], value[1]);
    // }

    // #[inline]
    // pub unsafe fn uniform_3i<T: AsRef<[i32; 3]>>(
    //     uniform_location: &UniformLocation<[i32; 3]>,
    //     value: T,
    // ) {
    //     let value = value.as_ref();
    //     self.gl.Uniform3i(uniform_location.as_i32(), value[0], value[1], value[2]);
    // }

    // #[inline]
    // pub unsafe fn uniform_4i<T: AsRef<[i32; 4]>>(
    //     uniform_location: &UniformLocation<[i32; 4]>,
    //     value: T,
    // ) {
    //     let value = value.as_ref();
    //     self.gl.Uniform4i(
    //         uniform_location.as_i32(),
    //         value[0],
    //         value[1],
    //         value[2],
    //         value[3],
    //     );
    // }

    // #[inline]
    // pub unsafe fn uniform_1f(&self, uniform_location: &UniformLocation<f32>, value: f32) {
    //     self.gl.Uniform1f(uniform_location.as_i32(), value);
    // }

    // #[inline]
    // pub unsafe fn uniform_2f(&self, uniform_location: &UniformLocation<[f32; 2]>, value: [f32; 2]) {
    //     self.gl.Uniform2f(uniform_location.as_i32(), value[0], value[1]);
    // }

    // #[inline]
    // pub unsafe fn uniform_3f(&self, uniform_location: &UniformLocation<[f32; 3]>, value: [f32; 3]) {
    //     self.gl.Uniform3f(uniform_location.as_i32(), value[0], value[1], value[2]);
    // }

    // #[inline]
    // pub unsafe fn uniform_4f(&self, uniform_location: &UniformLocation<[f32; 4]>, value: [f32; 4]) {
    //     self.gl.Uniform4f(
    //         uniform_location.as_i32(),
    //         value[0],
    //         value[1],
    //         value[2],
    //         value[3],
    //     );
    // }

    // #[inline]
    // pub unsafe fn uniform_1fv(&self, uniform_location: &UniformLocation<*const f32>, value: &[f32]) {
    //     self.gl.Uniform1fv(
    //         uniform_location.as_i32(),
    //         value.len() as i32,
    //         value.as_ptr(),
    //     );
    // }

    #[inline]
    pub unsafe fn uniform_matrix4f(
        &self,
        uniform_location: &UniformLocation<[[f32; 4]; 4]>,
        major_axis: MajorAxis,
        value: &[[f32; 4]; 4],
    ) {
        self.gl.UniformMatrix4fv(
            uniform_location.as_i32(),
            1,
            major_axis as u8,
            value.as_ptr() as *const f32,
        );
    }

    #[inline]
    pub unsafe fn uniform_matrix4fv(
        &self,
        uniform_location: &UniformLocation<[[f32; 4]; 4]>,
        major_axis: MajorAxis,
        values: &[[[f32; 4]; 4]],
    ) {
        self.gl.UniformMatrix4fv(
            uniform_location.as_i32(),
            values.len() as i32,
            major_axis as u8,
            values.as_ptr() as *const f32,
        );
    }

    // macro_rules! impl_uniform_matrix {
    //     ($(($n:ident, $M:ident, $Flat:ty)),+ $(,)*) => {
    //         $(
    //             pub unsafe fn $n<M: $M>(loc: &UniformLocation<$Flat>, val: &M) {
    //                 self.gl.UniformMatrix4fv(
    //                     loc.as_i32(),
    //                     1,
    //                     M::major_axis() as u8,
    //                     val.as_ref().as_ptr(),
    //                 );
    //             }
    //         )+
    //     }
    // }

    // impl_uniform_matrix!(
    //     (uniform_matrix2f, Matrix2f, [f32; 4]),
    //     (uniform_matrix3f, Matrix3f, [f32; 9]),
    //     (uniform_matrix4f, Matrix4f, [f32; 16]),
    //     (uniform_matrix2x3f, Matrix2x3f, [f32; 6]),
    //     (uniform_matrix3x2f, Matrix3x2f, [f32; 6]),
    //     (uniform_matrix2x4f, Matrix2x4f, [f32; 8]),
    //     (uniform_matrix4x2f, Matrix4x2f, [f32; 8]),
    //     (uniform_matrix3x4f, Matrix3x4f, [f32; 12]),
    //     (uniform_matrix4x3f, Matrix4x3f, [f32; 12]),
    // );
}
