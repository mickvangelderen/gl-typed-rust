#[macro_use]
mod macros;

pub mod array;
pub mod convert;
pub mod gl;
pub mod locations;
pub mod names;
pub mod num;
pub mod params;
pub mod string;
pub mod symbols;
pub mod types;

pub use array::*;
pub use convert::*;
pub use locations::*;
pub use names::*;
pub use params::*;
pub use symbols::*;
pub use types::*;

use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::os::raw::*;

macro_rules! impl_uniform_setters {
    ($fn1: ident, $glfn1: ident, $fn2: ident, $glfn2: ident, $fn3: ident, $glfn3: ident, $fn4: ident, $glfn4: ident, $ty: ty) => {
        #[inline]
        pub unsafe fn $fn1(&self, uniform_location: UniformLocation, value: $ty) {
            self.gl.$glfn1(uniform_location.into_i32(), value);
        }

        #[inline]
        pub unsafe fn $fn2(&self, uniform_location: UniformLocation, value: [$ty; 2]) {
            let [v0, v1] = value;
            self.gl.$glfn2(uniform_location.into_i32(), v0, v1);
        }

        #[inline]
        pub unsafe fn $fn3(&self, uniform_location: UniformLocation, value: [$ty; 3]) {
            let [v0, v1, v2] = value;
            self.gl.$glfn3(uniform_location.into_i32(), v0, v1, v2);
        }

        #[inline]
        pub unsafe fn $fn4(&self, uniform_location: UniformLocation, value: [$ty; 4]) {
            let [v0, v1, v2, v3] = value;
            self.gl
                .$glfn4(uniform_location.into_i32(), v0, v1, v2, v3);
        }
    }
}

pub struct Gl {
    gl: gl::Gl,
}

impl Gl {
    // Uniform setters.

    impl_uniform_setters!(
        uniform_1i, Uniform1i, uniform_2i, Uniform2i, uniform_3i, Uniform3i, uniform_4i, Uniform4i,
        i32
    );

    impl_uniform_setters!(
        uniform_1ui,
        Uniform1ui,
        uniform_2ui,
        Uniform2ui,
        uniform_3ui,
        Uniform3ui,
        uniform_4ui,
        Uniform4ui,
        u32
    );

    impl_uniform_setters!(
        uniform_1f, Uniform1f, uniform_2f, Uniform2f, uniform_3f, Uniform3f, uniform_4f, Uniform4f,
        f32
    );

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
    pub unsafe fn finish(&self) {
        self.gl.Finish();
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

    // NOTE(mickvangelderen): Can do this with type parameters and group by
    // return type length but thats rather clunky. Functions like these are
    // unlikely to receive a run-time parameter. Most of the time you'll know
    // what data you want to get. In the uncommon case you don't, simply use a
    // switch statement around your dynamic parameter to decide which function
    // to call.
    #[inline]
    pub unsafe fn get_context_flags(&self) -> ContextFlags {
        let mut values: [i32; 1] = std::mem::uninitialized();
        self.gl.GetIntegerv(gl::CONTEXT_FLAGS, values.as_mut_ptr());
        context_flags::ContextFlags::from_bits_truncate(values[0] as u32)
    }

    #[inline]
    pub unsafe fn get_max_texture_max_anisotropy(&self) -> f32 {
        let mut values: [f32; 1] = std::mem::uninitialized();
        self.gl
            .GetFloatv(gl::MAX_TEXTURE_MAX_ANISOTROPY, values.as_mut_ptr());
        values[0]
    }

    #[inline]
    pub unsafe fn get_uniform_buffer_offset_alignment(&self) -> i32 {
        let mut values: [i32; 1] = std::mem::uninitialized();
        self.gl
            .GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, values.as_mut_ptr());
        values[0]
    }

    #[inline]
    pub unsafe fn get_shader_storage_buffer_offset_alignment(&self) -> i32 {
        let mut values: [i32; 1] = std::mem::uninitialized();
        self.gl.GetIntegerv(
            gl::SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT,
            values.as_mut_ptr(),
        );
        values[0]
    }

    #[inline]
    pub unsafe fn get_error(&self) -> u32 {
        self.gl.GetError()
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
    pub unsafe fn clip_control<O, D>(&self, origin: O, depth: D)
    where
        O: Into<ClipControlOrigin>,
        D: Into<ClipControlDepth>,
    {
        self.gl
            .ClipControl(origin.into() as u32, depth.into() as u32)
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
    pub unsafe fn clear_depth(&self, depth: f64) {
        self.gl.ClearDepth(depth);
    }

    #[inline]
    pub unsafe fn clear(&self, flags: ClearFlags) {
        self.gl.Clear(flags.bits());
    }

    #[inline]
    pub unsafe fn color_mask<R, G, B, A>(&self, r: R, g: G, b: B, a: A)
    where
        R: Into<WriteMask>,
        G: Into<WriteMask>,
        B: Into<WriteMask>,
        A: Into<WriteMask>,
    {
        self.gl.ColorMask(
            r.into() as u8,
            g.into() as u8,
            b.into() as u8,
            a.into() as u8,
        );
    }

    #[inline]
    pub unsafe fn depth_mask<D>(&self, d: D)
    where
        D: Into<WriteMask>,
    {
        self.gl.DepthMask(d.into() as u8);
    }

    #[inline]
    pub unsafe fn depth_func<DF>(&self, func: DF)
    where
        DF: Into<DepthFunc>,
    {
        self.gl.DepthFunc(func.into() as u32);
    }

    #[inline]
    pub unsafe fn depth_range(&self, n: f64, f: f64) {
        self.gl.DepthRange(n, f);
    }

    #[inline]
    pub unsafe fn stencil_mask(&self, mask: u32) {
        self.gl.StencilMask(mask);
    }

    #[inline]
    pub unsafe fn draw_buffers(&self, framebuffer_attachments: &[FramebufferAttachment]) {
        self.gl.DrawBuffers(
            framebuffer_attachments.len() as i32,
            framebuffer_attachments.as_ptr() as *const u32,
        );
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
    pub unsafe fn create_shader<K>(&self, kind: K) -> ShaderName
    where
        K: Into<ShaderKind>,
    {
        self.try_create_shader(kind).unwrap()
    }

    #[inline]
    pub unsafe fn try_create_shader<K>(
        &self,
        kind: K,
    ) -> Result<ShaderName, ReceivedInvalidShaderName>
    where
        K: Into<ShaderKind>,
    {
        ShaderName::new(self.gl.CreateShader(kind.into() as u32))
    }

    #[inline]
    pub unsafe fn delete_shader(&self, name: ShaderName) {
        self.gl.DeleteShader(ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn compile_shader(&self, name: ShaderName) {
        self.gl.CompileShader(name.into_u32());
    }

    #[inline]
    pub unsafe fn get_shaderiv<P>(&self, name: ShaderName, _param: P) -> P::Value
    where
        P: get_shaderiv_param::Variant,
        <P::Value as TryFrom<P::Intermediate>>::Error: std::fmt::Debug,
    {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl
            .GetShaderiv(name.into_u32(), P::VALUE, value.as_mut_ptr());
        P::Intermediate::cast_from(value.assume_init())
            .try_into()
            .unwrap()
    }

    #[inline]
    pub unsafe fn get_shader_info_log(&self, name: ShaderName) -> String {
        String::from_utf8(self.get_shader_info_log_bytes(name)).unwrap()
    }

    #[inline]
    pub unsafe fn get_shader_info_log_bytes(&self, name: ShaderName) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.get_shaderiv(name, INFO_LOG_LENGTH));
        let mut length = MaybeUninit::<i32>::uninit();
        self.gl.GetShaderInfoLog(
            name.into_u32(),
            buffer.capacity() as i32,
            length.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut i8,
        );
        let length = length.assume_init();
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
        name: ShaderName,
        sources: &A,
    ) {
        let pointers = sources.map(|s| s.as_ptr() as *const c_char);
        let lengths = sources.map(|s| s.len() as i32);
        assert_eq!(pointers.len(), lengths.len());
        self.gl.ShaderSource(
            name.into_u32(),
            pointers.len() as i32,
            pointers.as_ptr(),
            lengths.as_ptr(),
        )
    }

    // Programs.

    #[inline]
    pub unsafe fn create_program(&self) -> ProgramName {
        self.try_create_program().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_program(&self) -> Result<ProgramName, ReceivedInvalidProgramName> {
        ProgramName::new(self.gl.CreateProgram())
    }

    #[inline]
    pub unsafe fn delete_program(&self, name: ProgramName) {
        self.gl.DeleteProgram(ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn use_program(&self, program: ProgramName) {
        self.gl.UseProgram(program.into_u32());
    }

    #[inline]
    pub unsafe fn unuse_program(&self) {
        self.gl.UseProgram(0);
    }

    #[inline]
    pub unsafe fn attach_shader(&self, program: ProgramName, shader: ShaderName) {
        self.gl.AttachShader(program.into_u32(), shader.into_u32());
    }

    #[inline]
    pub unsafe fn link_program(&self, program: ProgramName) {
        self.gl.LinkProgram(program.into_u32());
    }

    #[inline]
    pub unsafe fn get_programiv<P>(&self, name: ProgramName, _param: P) -> P::Value
    where
        P: get_programiv_param::Variant,
        <P::Value as TryFrom<P::Intermediate>>::Error: std::fmt::Debug,
    {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl
            .GetProgramiv(name.into_u32(), P::VALUE, value.as_mut_ptr());
        P::Intermediate::cast_from(value.assume_init())
            .try_into()
            .unwrap()
    }

    #[inline]
    pub unsafe fn get_program_info_log(&self, name: ProgramName) -> String {
        String::from_utf8(self.get_program_info_log_bytes(name)).unwrap()
    }

    #[inline]
    pub unsafe fn get_program_info_log_bytes(&self, name: ProgramName) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.get_programiv(name, INFO_LOG_LENGTH));
        let mut length = MaybeUninit::<i32>::uninit();
        self.gl.GetProgramInfoLog(
            name.into_u32(),
            buffer.capacity() as i32,
            length.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut i8,
        );
        let length = length.assume_init();
        assert!(length >= 0 && length <= buffer.capacity() as i32);
        buffer.set_len(length as usize);
        buffer
    }

    #[inline]
    pub unsafe fn get_attrib_location(
        &self,
        program_name: ProgramName,
        attrib_name: &CStr,
    ) -> OptionAttributeLocation {
        OptionAttributeLocation::new(
            self.gl
                .GetAttribLocation(program_name.into_u32(), attrib_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn vertex_attrib_pointer<T>(
        &self,
        loc: AttributeLocation,
        size: usize,
        ty: T,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) where
        T: Into<VertexAttribPointerType>,
    {
        self.gl.VertexAttribPointer(
            loc.into_u32(),
            size as i32,
            ty.into() as u32,
            normalized as u8,
            stride as i32,
            offset as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn get_uniform_location(
        &self,
        program_name: ProgramName,
        uniform_name: &CStr,
    ) -> OptionUniformLocation {
        OptionUniformLocation::new(
            self.gl
                .GetUniformLocation(program_name.into_u32(), uniform_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, loc: AttributeLocation) {
        self.gl.EnableVertexAttribArray(loc.into_u32());
    }

    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, loc: AttributeLocation) {
        self.gl.DisableVertexAttribArray(loc.into_u32());
    }

    // Textures.

    #[inline]
    pub unsafe fn create_texture<K>(&self, kind: K) -> TextureName
    where
        K: Into<TextureTarget>,
    {
        self.try_create_texture(kind).unwrap()
    }

    #[inline]
    pub unsafe fn try_create_texture<K>(
        &self,
        kind: K,
    ) -> Result<TextureName, ReceivedInvalidTextureName>
    where
        K: Into<TextureTarget>,
    {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl
            .CreateTextures(kind.into() as u32, 1, name.as_mut_ptr());
        TextureName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_texture(&self, name: TextureName) {
        self.gl
            .DeleteTextures(1, &ManuallyDrop::new(name).into_u32());
    }

    #[deprecated]
    #[inline]
    pub unsafe fn gen_textures(&self, names: &mut [Option<TextureName>]) {
        self.gl
            .GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn delete_textures(&self, names: &mut [Option<TextureName>]) {
        self.gl
            .DeleteTextures(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn active_texture<U>(&self, unit: U)
    where
        U: Into<TextureUnit>,
    {
        self.gl.ActiveTexture(unit.into().into_u32());
    }

    #[inline]
    pub unsafe fn bind_texture<T>(&self, target: T, name: TextureName)
    where
        T: Into<TextureTarget>,
    {
        self.gl.BindTexture(target.into() as u32, name.into_u32());
    }

    #[inline]
    pub unsafe fn unbind_texture<T>(&self, target: T)
    where
        T: Into<TextureTarget>,
    {
        self.gl.BindTexture(target.into() as u32, 0);
    }

    #[inline]
    pub unsafe fn texture_parameteri<P, V>(&self, name: TextureName, _param: P, value: V)
    where
        P: tex_parameteri_param::Variant,
        V: Into<P::Value>,
    {
        self.gl
            .TextureParameteri(name.into_u32(), P::VALUE, value.into().into().cast_into());
    }

    #[inline]
    pub unsafe fn texture_parameterf<P, V>(&self, name: TextureName, _param: P, value: V)
    where
        P: tex_parameterf_param::Variant,
        V: Into<P::Value>,
    {
        self.gl
            .TextureParameterf(name.into_u32(), P::VALUE, value.into().into().cast_into());
    }

    #[deprecated]
    #[inline]
    pub unsafe fn tex_parameteri<T, P, V>(&self, target: T, _param: P, value: V)
    where
        T: Into<TextureTarget>,
        P: tex_parameteri_param::Variant,
        V: Into<P::Value>,
    {
        self.gl.TexParameteri(
            target.into() as u32,
            P::VALUE,
            value.into().into().cast_into(),
        );
    }

    #[deprecated]
    #[inline]
    pub unsafe fn tex_parameterf<T, P, V>(&self, target: T, _param: P, value: V)
    where
        T: Into<TextureTarget>,
        P: tex_parameterf_param::Variant,
        V: Into<P::Value>,
    {
        self.gl.TexParameterf(
            target.into() as u32,
            P::VALUE,
            value.into().into().cast_into(),
        );
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

    // Renderbuffers.

    #[inline]
    pub unsafe fn create_renderbuffer(&self) -> RenderbufferName {
        self.try_create_renderbuffer().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_renderbuffer(
        &self,
    ) -> Result<RenderbufferName, ReceivedInvalidRenderbufferName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl.CreateRenderbuffers(1, name.as_mut_ptr());
        RenderbufferName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_renderbuffer(&self, name: RenderbufferName) {
        self.gl
            .DeleteRenderbuffers(1, &ManuallyDrop::new(name).into_u32());
    }

    #[deprecated]
    #[inline]
    pub unsafe fn gen_renderbuffers(&self, names: &mut [Option<RenderbufferName>]) {
        self.gl
            .GenRenderbuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn delete_renderbuffers(&self, names: &mut [Option<RenderbufferName>]) {
        self.gl
            .DeleteRenderbuffers(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn bind_renderbuffer<T>(&self, target: T, name: RenderbufferName)
    where
        T: Into<RenderbufferTarget>,
    {
        self.gl
            .BindRenderbuffer(target.into() as u32, name.into_u32());
    }

    #[inline]
    pub unsafe fn unbind_renderbuffer<T>(&self, target: T)
    where
        T: Into<RenderbufferTarget>,
    {
        self.gl.BindRenderbuffer(target.into() as u32, 0);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn renderbuffer_storage<T, IF>(
        &self,
        target: T,
        internal_format: IF,
        width: i32,
        height: i32,
    ) where
        T: Into<RenderbufferTarget>,
        IF: Into<InternalFormat>,
    {
        self.gl.RenderbufferStorage(
            target.into() as u32,
            internal_format.into() as u32,
            width,
            height,
        );
    }

    #[inline]
    pub unsafe fn named_renderbuffer_storage<IF>(
        &self,
        name: RenderbufferName,
        internal_format: IF,
        width: i32,
        height: i32,
    ) where
        IF: Into<InternalFormat>,
    {
        self.gl.NamedRenderbufferStorage(
            name.into_u32(),
            internal_format.into() as u32,
            width,
            height,
        );
    }

    // Buffers.

    #[deprecated]
    #[inline]
    pub unsafe fn gen_buffers(&self, names: &mut [Option<BufferName>]) {
        self.gl
            .GenBuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn delete_buffers(&self, names: &mut [Option<BufferName>]) {
        self.gl
            .DeleteBuffers(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn create_buffer(&self) -> BufferName {
        self.try_create_buffer().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_buffer(&self) -> Result<BufferName, ReceivedInvalidBufferName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl.CreateBuffers(1, name.as_mut_ptr());
        BufferName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_buffer(&self, name: BufferName) {
        self.gl
            .DeleteBuffers(1, &ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn bind_buffer<T>(&self, target: T, name: BufferName)
    where
        T: Into<BufferTarget>,
    {
        self.gl.BindBuffer(target.into() as u32, name.into_u32());
    }

    #[inline]
    pub unsafe fn unbind_buffer<T>(&self, target: T)
    where
        T: Into<BufferTarget>,
    {
        self.gl.BindBuffer(target.into() as u32, 0);
    }

    #[deprecated]
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

    /// Complement our inability to specify a slice of a certain size without
    /// wanting to write anything.
    #[deprecated]
    #[inline]
    pub unsafe fn buffer_reserve<T, U>(&self, target: T, capacity: usize, usage: U)
    where
        T: Into<BufferTarget>,
        U: Into<BufferUsage>,
    {
        self.gl.BufferData(
            target.into() as u32,
            capacity as isize,
            std::ptr::null(),
            usage.into() as u32,
        );
    }

    #[deprecated]
    #[inline]
    pub unsafe fn buffer_sub_data<T, D>(&self, target: T, offset: usize, data: &[D])
    where
        T: Into<BufferTarget>,
    {
        self.gl.BufferSubData(
            target.into() as u32,
            offset as isize,
            std::mem::size_of_val(data) as isize,
            data.as_ptr() as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn named_buffer_data<U>(&self, name: BufferName, bytes: &[u8], usage: U)
    where
        U: Into<BufferUsage>,
    {
        self.gl.NamedBufferData(
            name.into_u32(),
            bytes.len() as isize,
            bytes.as_ptr() as *const c_void,
            usage.into() as u32,
        );
    }

    #[inline]
    pub unsafe fn named_buffer_reserve<U>(&self, name: BufferName, capacity: usize, usage: U)
    where
        U: Into<BufferUsage>,
    {
        self.gl.NamedBufferData(
            name.into_u32(),
            capacity as isize,
            std::ptr::null(),
            usage.into() as u32,
        );
    }

    #[inline]
    pub unsafe fn named_buffer_sub_data(&self, name: BufferName, offset: usize, bytes: &[u8]) {
        self.gl.NamedBufferSubData(
            name.into_u32(),
            offset as isize,
            bytes.len() as isize,
            bytes.as_ptr() as *const c_void,
        );
    }

    // Vertex array names.

    #[deprecated]
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, names: &mut [Option<VertexArrayName>]) {
        self.gl
            .GenVertexArrays(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, names: &mut [Option<VertexArrayName>]) {
        self.gl
            .DeleteVertexArrays(names.len() as i32, names.as_ptr() as *const u32);
    }

    #[inline]
    pub unsafe fn create_vertex_array(&self) -> VertexArrayName {
        self.try_create_vertex_array().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_vertex_array(
        &self,
    ) -> Result<VertexArrayName, ReceivedInvalidVertexArrayName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl.CreateVertexArrays(1, name.as_mut_ptr());
        VertexArrayName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_vertex_array(&self, name: VertexArrayName) {
        self.gl
            .DeleteVertexArrays(1, &ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn bind_vertex_array(&self, name: VertexArrayName) {
        self.gl.BindVertexArray(name.into_u32());
    }

    #[inline]
    pub unsafe fn unbind_vertex_array(&self) {
        self.gl.BindVertexArray(0);
    }

    // Framebuffer names.

    #[deprecated]
    #[inline]
    pub unsafe fn gen_framebuffers(&self, names: &mut [Option<NonDefaultFramebufferName>]) {
        self.gl
            .GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn delete_framebuffers(&self, names: &mut [Option<NonDefaultFramebufferName>]) {
        self.gl
            .GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }

    #[inline]
    pub unsafe fn create_framebuffer(&self) -> NonDefaultFramebufferName {
        self.try_create_framebuffer().unwrap().into()
    }

    #[inline]
    pub unsafe fn try_create_framebuffer(
        &self,
    ) -> Result<NonDefaultFramebufferName, ReceivedInvalidFramebufferName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl.CreateFramebuffers(1, name.as_mut_ptr());
        NonDefaultFramebufferName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_framebuffer(&self, name: NonDefaultFramebufferName) {
        self.gl
            .DeleteFramebuffers(1, &ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn bind_framebuffer<T, N>(&self, target: T, name: N)
    where
        T: Into<FramebufferTarget>,
        N: Into<FramebufferName>,
    {
        self.gl
            .BindFramebuffer(target.into() as u32, name.into().into_u32())
    }

    #[inline]
    pub unsafe fn check_named_framebuffer_status<N, T>(
        &self,
        name: N,
        target: T,
    ) -> FramebufferStatus
    where
        N: Into<FramebufferName>,
        T: Into<FramebufferTarget>,
    {
        self.gl
            .CheckNamedFramebufferStatus(name.into().into_u32(), target.into() as u32)
            .try_into()
            .unwrap()
    }

    #[deprecated]
    #[inline]
    pub unsafe fn framebuffer_texture_2d<FT, FA, TT>(
        &self,
        framebuffer_target: FT,
        framebuffer_attachment: FA,
        texture_target: TT,
        texture_name: TextureName,
        mipmap_level: i32,
    ) where
        FT: Into<FramebufferTarget>,
        FA: Into<FramebufferAttachment>,
        TT: Into<TextureTarget>,
    {
        self.gl.FramebufferTexture2D(
            framebuffer_target.into() as u32,
            framebuffer_attachment.into().into_u32(),
            texture_target.into() as u32,
            texture_name.into_u32(),
            mipmap_level,
        );
    }

    #[inline]
    pub unsafe fn named_framebuffer_texture<FA>(
        &self,
        framebuffer_name: NonDefaultFramebufferName,
        framebuffer_attachment: FA,
        texture_name: TextureName,
        level: i32,
    ) where
        FA: Into<FramebufferAttachment>,
    {
        self.gl.NamedFramebufferTexture(
            framebuffer_name.into_u32(),
            framebuffer_attachment.into().into_u32(),
            texture_name.into_u32(),
            level,
        );
    }

    #[deprecated]
    #[inline]
    pub unsafe fn framebuffer_renderbuffer<FT, FA, RT>(
        &self,
        framebuffer_target: FT,
        framebuffer_attachment: FA,
        renderbuffer_target: RT,
        renderbuffer: RenderbufferName,
    ) where
        FT: Into<FramebufferTarget>,
        FA: Into<FramebufferAttachment>,
        RT: Into<RenderbufferTarget>,
    {
        self.gl.FramebufferRenderbuffer(
            framebuffer_target.into() as u32,
            framebuffer_attachment.into().into_u32(),
            renderbuffer_target.into() as u32,
            renderbuffer.into_u32(),
        );
    }

    #[inline]
    pub unsafe fn named_framebuffer_renderbuffer<FA, RT>(
        &self,
        framebuffer_name: NonDefaultFramebufferName,
        framebuffer_attachment: FA,
        renderbuffer_target: RT,
        renderbuffer_name: RenderbufferName,
    ) where
        FA: Into<FramebufferAttachment>,
        RT: Into<RenderbufferTarget>,
    {
        self.gl.NamedFramebufferRenderbuffer(
            framebuffer_name.into_u32(),
            framebuffer_attachment.into().into_u32(),
            renderbuffer_target.into() as u32,
            renderbuffer_name.into_u32(),
        );
    }

    #[inline]
    pub unsafe fn uniform_1iv(&self, uniform_location: UniformLocation, value: &[i32]) {
        self.gl.Uniform1iv(
            uniform_location.into_i32(),
            value.len() as i32,
            value.as_ptr(),
        );
    }

    #[inline]
    pub unsafe fn uniform_1fv(&self, uniform_location: UniformLocation, value: &[f32]) {
        self.gl.Uniform1fv(
            uniform_location.into_i32(),
            value.len() as i32,
            value.as_ptr(),
        );
    }

    #[inline]
    pub unsafe fn uniform_matrix4f(
        &self,
        uniform_location: UniformLocation,
        major_axis: MajorAxis,
        value: &[[f32; 4]; 4],
    ) {
        self.gl.UniformMatrix4fv(
            uniform_location.into_i32(),
            1,
            major_axis as u8,
            value.as_ptr() as *const f32,
        );
    }

    #[inline]
    pub unsafe fn uniform_matrix4fv(
        &self,
        uniform_location: UniformLocation,
        major_axis: MajorAxis,
        values: &[[[f32; 4]; 4]],
    ) {
        self.gl.UniformMatrix4fv(
            uniform_location.into_i32(),
            values.len() as i32,
            major_axis as u8,
            values.as_ptr() as *const f32,
        );
    }

    // macro_rules! impl_uniform_matrix {
    //     ($(($n:ident, $M:ident, $Flat:ty)),+ $(,)*) => {
    //         $(
    //             pub unsafe fn $n<M: $M>(loc: UniformLocation<$Flat>, val: &M) {
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

    #[inline]
    pub unsafe fn get_uniform_block_index(
        &self,
        program_name: ProgramName,
        uniform_block_name: &CStr,
    ) -> OptionUniformBlockIndex {
        OptionUniformBlockIndex::new(
            self.gl
                .GetUniformBlockIndex(program_name.into_u32(), uniform_block_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn uniform_block_binding(
        &self,
        program_name: ProgramName,
        uniform_block_index: UniformBlockIndex,
        uniform_block_binding: u32,
    ) {
        self.gl.UniformBlockBinding(
            program_name.into_u32(),
            uniform_block_index.into_u32(),
            uniform_block_binding,
        );
    }

    #[inline]
    pub unsafe fn bind_buffer_base<T>(&self, target: T, index: u32, buffer_name: BufferName)
    where
        T: Into<BindBufferTarget>,
    {
        self.gl
            .BindBufferBase(target.into() as u32, index, buffer_name.into_u32());
    }

    #[inline]
    pub unsafe fn bind_buffer_range<T>(
        &self,
        target: T,
        index: u32,
        buffer_name: BufferName,
        offset: usize,
        size: usize,
    ) where
        T: Into<BindBufferTarget>,
    {
        self.gl.BindBufferRange(
            target.into() as u32,
            index,
            buffer_name.into_u32(),
            offset as isize,
            size as isize,
        );
    }

    // Samplers.

    #[inline]
    pub unsafe fn create_sampler(&self) -> SamplerName {
        self.try_create_sampler().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_sampler(&self) -> Result<SamplerName, ReceivedInvalidSamplerName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl.CreateSamplers(1, name.as_mut_ptr());
        SamplerName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn delete_sampler(&self, name: SamplerName) {
        self.gl
            .DeleteSamplers(1, &ManuallyDrop::new(name).into_u32());
    }

    #[inline]
    pub unsafe fn bind_sampler(&self, unit: u32, name: SamplerName) {
        self.gl.BindSampler(unit, name.into_u32());
    }

    #[inline]
    pub unsafe fn bind_samplers(&self, first_unit: u32, count: u32, names: &[SamplerName]) {
        self.gl.BindSamplers(
            first_unit,
            count as i32,
            names.as_ptr() as *const SamplerName as *const u32,
        );
    }

    #[inline]
    pub unsafe fn unbind_sampler(&self, unit: u32) {
        self.gl.BindSampler(unit, 0);
    }

    #[inline]
    pub unsafe fn sampler_parameteri<P, V>(&self, sampler: SamplerName, _param: P, value: V)
    where
        P: sampler_parameteri_param::Variant,
        V: Into<P::Value>,
    {
        self.gl.SamplerParameteri(
            sampler.into_u32(),
            P::VALUE,
            value.into().into().cast_into(),
        );
    }
}
