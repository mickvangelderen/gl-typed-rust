#[macro_use]
mod macros;

mod attribute_format;
pub mod convert;
pub mod gl;
pub mod locations;
pub mod names;
pub mod num;
pub mod params;
pub mod string;
pub mod symbols;
pub mod types;

pub use attribute_format::*;
pub use convert::*;
pub use locations::*;
pub use names::*;
pub use params::*;
pub use symbols::*;
pub use types::*;

use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::num::NonZeroU64;
use std::os::raw::{c_char, c_void};

macro_rules! impl_uniform_setters {
    ($fn1: ident, $glfn1: ident, $fn2: ident, $glfn2: ident, $fn3: ident, $glfn3: ident, $fn4: ident, $glfn4: ident, $ty: ty) => {
        #[inline]
        pub unsafe fn $fn1(&self, uniform_location: UniformLocation, value: $ty) {
            self.gl.$glfn1(uniform_location.to_i32(), value);
        }

        #[inline]
        pub unsafe fn $fn2(&self, uniform_location: UniformLocation, value: [$ty; 2]) {
            let [v0, v1] = value;
            self.gl.$glfn2(uniform_location.to_i32(), v0, v1);
        }

        #[inline]
        pub unsafe fn $fn3(&self, uniform_location: UniformLocation, value: [$ty; 3]) {
            let [v0, v1, v2] = value;
            self.gl.$glfn3(uniform_location.to_i32(), v0, v1, v2);
        }

        #[inline]
        pub unsafe fn $fn4(&self, uniform_location: UniformLocation, value: [$ty; 4]) {
            let [v0, v1, v2, v3] = value;
            self.gl
                .$glfn4(uniform_location.to_i32(), v0, v1, v2, v3);
        }
    }
}

macro_rules! impl_object_label {
    ($(
        ($fn: ident, $name: ident, $Name: ty, $variant: expr),
    )*) => {
        $(
            #[inline]
            pub unsafe fn $fn(&self, $name: impl AsRef<$Name>, label: &str) {
                self.gl.ObjectLabel(
                    $variant,
                    $name.as_ref().to_u32(),
                    label.len() as i32,
                    label.as_ptr() as *const i8,
                );
            }
        )*
    };
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

    impl_object_label! {
        (buffer_label, buffer_name, BufferName, gl::BUFFER),
        (shader_label, shader_name, ShaderName, gl::SHADER),
        (program_label, program_name, ProgramName, gl::PROGRAM),
        (vertex_array_label, vertex_array_name, VertexArrayName, gl::VERTEX_ARRAY),
        (query_label, query_name, QueryName, gl::QUERY),
        (program_pipeline_label, program_pipeline_name, ProgramPipelineName, gl::PROGRAM_PIPELINE),
        (transform_feedback_label, transform_feedback_name, TransformFeedbackName, gl::TRANSFORM_FEEDBACK),
        (sampler_label, sampler_name, SamplerName, gl::SAMPLER),
        (texture_label, texture_name, TextureName, gl::TEXTURE),
        (renderbuffer_label, renderbuffer_name, RenderbufferName, gl::RENDERBUFFER),
        (framebuffer_label, framebuffer_name, NonDefaultFramebufferName, gl::FRAMEBUFFER),
    }

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
    pub unsafe fn memory_barrier(&self, flags: MemoryBarrierFlag) {
        self.gl.MemoryBarrier(flags.bits());
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

    #[inline]
    pub unsafe fn get_integer_v(&self, name: u32, data: &mut [u32]) {
        self.gl.GetIntegerv(name, data.as_mut_ptr() as *mut i32);
    }

    // NOTE(mickvangelderen): Can do this with type parameters and group by
    // return type length but thats rather clunky. Functions like these are
    // unlikely to receive a run-time parameter. Most of the time you'll know
    // what data you want to get. In the uncommon case you don't, simply use a
    // switch statement around your dynamic parameter to decide which function
    // to call.
    #[inline]
    pub unsafe fn get_context_flags(&self) -> ContextFlag {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl.GetIntegerv(gl::CONTEXT_FLAGS, value.as_mut_ptr());
        ContextFlag::from_bits_truncate(value.assume_init().try_into().unwrap())
    }

    #[inline]
    pub unsafe fn get_max_texture_max_anisotropy(&self) -> f32 {
        let mut value = MaybeUninit::<f32>::uninit();
        self.gl
            .GetFloatv(gl::MAX_TEXTURE_MAX_ANISOTROPY, value.as_mut_ptr());
        value.assume_init()
    }

    #[inline]
    pub unsafe fn get_uniform_buffer_offset_alignment(&self) -> i32 {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl
            .GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, value.as_mut_ptr());
        value.assume_init()
    }

    #[inline]
    pub unsafe fn get_shader_storage_buffer_offset_alignment(&self) -> i32 {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl.GetIntegerv(
            gl::SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT,
            value.as_mut_ptr(),
        );
        value.assume_init()
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
    pub unsafe fn front_face<F>(&self, face: F)
    where
        F: Into<FrontFace>,
    {
        self.gl.FrontFace(face.into() as u32);
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
    pub unsafe fn clear(&self, flags: ClearFlag) {
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
    pub unsafe fn blend_func(&self, src: impl Into<BlendFactor>, dst: impl Into<BlendFactor>) {
        self.gl.BlendFunc(src.into() as u32, dst.into() as u32);
    }

    #[inline]
    pub unsafe fn blend_funci(&self, draw_buffer: u32, src: impl Into<BlendFactor>, dst: impl Into<BlendFactor>) {
        self.gl.BlendFunci(draw_buffer, src.into() as u32, dst.into() as u32);
    }

    #[inline]
    pub unsafe fn pixel_store_pack_alignment(&self, alignment: PixelAlignment) {
        self.gl.PixelStorei(gl::PACK_ALIGNMENT, alignment.to_gl())
    }

    #[inline]
    pub unsafe fn pixel_store_unpack_alignment(&self, alignment: PixelAlignment) {
        self.gl.PixelStorei(gl::UNPACK_ALIGNMENT, alignment.to_gl())
    }

    #[deprecated]
    #[inline]
    pub unsafe fn draw_buffers(&self, framebuffer_attachments: &[FramebufferAttachment]) {
        self.gl.DrawBuffers(
            framebuffer_attachments.len() as i32,
            framebuffer_attachments.as_ptr() as *const u32,
        );
    }

    #[inline]
    pub unsafe fn named_framebuffer_draw_buffers(
        &self,
        framebuffer_name: NonDefaultFramebufferName,
        framebuffer_attachments: &[FramebufferAttachment],
    ) {
        self.gl.NamedFramebufferDrawBuffers(
            framebuffer_name.to_u32(),
            framebuffer_attachments.len() as i32,
            framebuffer_attachments.as_ptr() as *const u32,
        );
    }

    #[inline]
    pub unsafe fn clear_named_framebufferiv(
        &self,
        framebuffer_name: FramebufferName,
        buffer: impl Into<FramebufferBuffer>,
        draw_buffer: u32,
        value: [i32; 4],
    ) {
        self.gl.ClearNamedFramebufferiv(
            framebuffer_name.to_u32(),
            buffer.into() as u32,
            draw_buffer as i32,
            value.as_ptr(),
        )
    }

    #[inline]
    pub unsafe fn clear_named_framebufferuiv(
        &self,
        framebuffer_name: FramebufferName,
        buffer: impl Into<FramebufferBuffer>,
        draw_buffer: u32,
        value: [u32; 4],
    ) {
        self.gl.ClearNamedFramebufferuiv(
            framebuffer_name.to_u32(),
            buffer.into() as u32,
            draw_buffer as i32,
            value.as_ptr(),
        )
    }

    #[inline]
    pub unsafe fn clear_named_framebufferfv(
        &self,
        framebuffer_name: FramebufferName,
        buffer: impl Into<FramebufferBuffer>,
        draw_buffer: u32,
        value: [f32; 4],
    ) {
        self.gl.ClearNamedFramebufferfv(
            framebuffer_name.to_u32(),
            buffer.into() as u32,
            draw_buffer as i32,
            value.as_ptr(),
        )
    }

    #[inline]
    pub unsafe fn clear_named_framebufferfi(
        &self,
        framebuffer_name: FramebufferName,
        buffer: impl Into<FramebufferBuffer>,
        draw_buffer: u32,
        depth: f32,
        stencil: u32,
    ) {
        self.gl.ClearNamedFramebufferfi(
            framebuffer_name.to_u32(),
            buffer.into() as u32,
            draw_buffer as i32,
            depth,
            stencil as i32,
        )
    }


    #[inline]
    pub unsafe fn blit_named_framebuffer(
        &self,
        read_framebuffer_name: FramebufferName,
        draw_framebuffer_name: FramebufferName,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: impl Into<BlitMask>,
        filter: impl Into<BlitFilter>,
    ) {
        self.gl.BlitNamedFramebuffer(
            read_framebuffer_name.to_u32(),
            draw_framebuffer_name.to_u32(),
            src_x0,
            src_y0,
            src_x1,
            src_y1,
            dst_x0,
            dst_y0,
            dst_x1,
            dst_y1,
            mask.into().bits(),
            filter.into() as u32,
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
    pub unsafe fn draw_elements<M, T>(&self, mode: M, count: u32, ty: T, offset: u32)
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

    #[inline]
    pub unsafe fn draw_elements_base_vertex<M, T>(
        &self,
        mode: M,
        count: u32,
        ty: T,
        offset: usize,
        base_vertex: u32,
    ) where
        M: Into<DrawMode>,
        T: Into<DrawElementsType>,
    {
        self.gl.DrawElementsBaseVertex(
            mode.into() as u32,
            count as i32,
            ty.into() as u32,
            offset as *const c_void,
            base_vertex as i32,
        );
    }

    #[inline]
    pub unsafe fn draw_elements_instanced_base_vertex<M, T>(
        &self,
        mode: M,
        count: u32,
        ty: T,
        offset: usize,
        instance_count: u32,
        base_vertex: u32,
    ) where
        M: Into<DrawMode>,
        T: Into<DrawElementsType>,
    {
        self.gl.DrawElementsInstancedBaseVertex(
            mode.into() as u32,
            count as i32,
            ty.into() as u32,
            offset as *const c_void,
            instance_count as i32,
            base_vertex as i32,
        );
    }

    #[inline]
    pub unsafe fn draw_elements_indirect<M, T>(&self, mode: M, ty: T, offset: usize)
    where
        M: Into<DrawMode>,
        T: Into<DrawElementsType>,
    {
        self.gl.DrawElementsIndirect(
            mode.into() as u32,
            ty.into() as u32,
            offset as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn multi_draw_elements_indirect<M, T>(
        &self,
        mode: M,
        ty: T,
        offset: usize,
        draw_count: i32,
        stride: i32,
    ) where
        M: Into<DrawMode>,
        T: Into<DrawElementsType>,
    {
        self.gl.MultiDrawElementsIndirect(
            mode.into() as u32,
            ty.into() as u32,
            offset as *const c_void,
            draw_count,
            stride,
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
        self.gl.DeleteShader(ManuallyDrop::new(name).to_u32());
    }

    #[inline]
    pub unsafe fn shader_source<'i, I>(&self, shader_name: ShaderName, sources: I)
    where
        I: IntoIterator,
        I::Item: 'i + AsRef<[u8]>,
    {
        let (pointers, lengths) = sources.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut pointers, mut lengths), source| {
                let bytes = source.as_ref();
                pointers.push(bytes.as_ptr() as *const i8);
                lengths.push(bytes.len() as i32);
                (pointers, lengths)
            },
        );

        self.gl.ShaderSource(
            shader_name.to_u32(),
            pointers.len() as i32,
            pointers.as_ptr(),
            lengths.as_ptr(),
        );
    }

    #[inline]
    pub unsafe fn compile_shader(&self, name: ShaderName) {
        self.gl.CompileShader(name.to_u32());
    }

    #[inline]
    pub unsafe fn get_shaderiv<P>(&self, name: ShaderName, _param: P) -> P::Value
    where
        P: get_shaderiv_param::Variant,
        <P::Value as TryFrom<P::Intermediate>>::Error: std::fmt::Debug,
    {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl
            .GetShaderiv(name.to_u32(), P::VALUE, value.as_mut_ptr());
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
            name.to_u32(),
            buffer.capacity() as i32,
            length.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut i8,
        );
        let length = length.assume_init();
        assert!(length >= 0 && length <= buffer.capacity() as i32);
        buffer.set_len(length as usize);
        buffer
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
        self.gl.DeleteProgram(ManuallyDrop::new(name).to_u32());
    }

    #[inline]
    pub unsafe fn use_program(&self, program_name: ProgramName) {
        self.gl.UseProgram(program_name.to_u32());
    }

    #[inline]
    pub unsafe fn unuse_program(&self) {
        self.gl.UseProgram(0);
    }

    #[inline]
    pub unsafe fn attach_shader(&self, program_name: ProgramName, shader_name: ShaderName) {
        self.gl
            .AttachShader(program_name.to_u32(), shader_name.to_u32());
    }

    #[inline]
    pub unsafe fn link_program(&self, program_name: ProgramName) {
        self.gl.LinkProgram(program_name.to_u32());
    }

    #[inline]
    pub unsafe fn get_programiv<P>(&self, name: ProgramName, _param: P) -> P::Value
    where
        P: get_programiv_param::Variant,
        <P::Value as TryFrom<P::Intermediate>>::Error: std::fmt::Debug,
    {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl
            .GetProgramiv(name.to_u32(), P::VALUE, value.as_mut_ptr());
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
            name.to_u32(),
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
        OptionAttributeLocation::from_i32(
            self.gl
                .GetAttribLocation(program_name.to_u32(), attrib_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn get_uniform_location(
        &self,
        program_name: ProgramName,
        uniform_name: &CStr,
    ) -> OptionUniformLocation {
        OptionUniformLocation::from_i32(
            self.gl
                .GetUniformLocation(program_name.to_u32(), uniform_name.as_ptr()),
        )
    }

    #[inline]
    pub unsafe fn dispatch_compute(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
        self.gl
            .DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
    }

    #[inline]
    pub unsafe fn dispatch_compute_indirect(&self, byte_offset: usize) {
        self.gl.DispatchComputeIndirect(byte_offset as isize);
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
        self.gl.DeleteTextures(1, &ManuallyDrop::new(name).to_u32());
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

    #[deprecated]
    #[inline]
    pub unsafe fn active_texture<U>(&self, unit: U)
    where
        U: Into<TextureUnit>,
    {
        self.gl.ActiveTexture(unit.into().to_u32());
    }

    #[inline]
    pub unsafe fn bind_texture_unit(&self, unit: u32, texture_name: TextureName) {
        self.gl.BindTextureUnit(unit, texture_name.to_u32());
    }

    #[inline]
    pub unsafe fn bind_texture<T>(&self, target: T, name: TextureName)
    where
        T: Into<TextureTarget>,
    {
        self.gl.BindTexture(target.into() as u32, name.to_u32());
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
            .TextureParameteri(name.to_u32(), P::VALUE, value.into().into().cast_into());
    }

    #[inline]
    pub unsafe fn texture_parameterf<P, V>(&self, name: TextureName, _param: P, value: V)
    where
        P: tex_parameterf_param::Variant,
        V: Into<P::Value>,
    {
        self.gl
            .TextureParameterf(name.to_u32(), P::VALUE, value.into().into().cast_into());
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

    #[deprecated]
    #[inline]
    pub unsafe fn generate_mipmap<T>(&self, target: T)
    where
        T: Into<TextureTarget>,
    {
        self.gl.GenerateMipmap(target.into() as u32);
    }

    #[inline]
    pub unsafe fn generate_texture_mipmap(&self, texture_name: TextureName) {
        self.gl.GenerateTextureMipmap(texture_name.to_u32());
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

    #[inline]
    pub unsafe fn tex_image_2d_multisample(
        &self,
        target: impl Into<TextureTarget>,
        samples: i32,
        internal_format: impl Into<InternalFormat>,
        width: i32,
        height: i32,
        fixed_sample_locations: bool,
    ) {
        self.gl.TexImage2DMultisample(
            target.into() as u32,
            samples,
            internal_format.into() as u32,
            width,
            height,
            fixed_sample_locations as u8,
        );
    }

    #[inline]
    pub unsafe fn texture_storage_2d(
        &self,
        texture_name: impl AsRef<TextureName>,
        levels: i32,
        internal_format: impl Into<InternalFormat>,
        width: i32,
        height: i32,
    ) {
        self.gl.TextureStorage2D(
            texture_name.as_ref().to_u32(),
            levels,
            internal_format.into() as u32,
            width,
            height,
        );
    }

    #[inline]
    pub unsafe fn texture_storage_3d(
        &self,
        texture_name: impl AsRef<TextureName>,
        levels: i32,
        internal_format: impl Into<InternalFormat>,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        self.gl.TextureStorage3D(
            texture_name.as_ref().to_u32(),
            levels,
            internal_format.into() as u32,
            width,
            height,
            depth,
        );
    }

    #[inline]
    pub unsafe fn texture_sub_image_2d(
        &self,
        texture_name: impl AsRef<TextureName>,
        level: i32,
        offset_x: i32,
        offset_y: i32,
        width: i32,
        height: i32,
        format: impl Into<Format>,
        ty: impl Into<ComponentFormat>,
        pixels: *const c_void,
    ) {
        self.gl.TextureSubImage2D(
            texture_name.as_ref().to_u32(),
            level,
            offset_x,
            offset_y,
            width,
            height,
            format.into() as u32,
            ty.into() as u32,
            pixels,
        );
    }

    #[inline]
    pub unsafe fn compressed_tex_image_2d(
        &self,
        target: impl Into<TextureTarget>,
        level: i32,
        internal_format: impl Into<InternalFormat>,
        width: i32,
        height: i32,
        data: &[u8],
    ) {
        self.gl.CompressedTexImage2D(
            target.into() as u32,
            level,
            internal_format.into() as u32,
            width,
            height,
            0, // Border must be 0
            data.len() as i32,
            data.as_ptr() as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn get_texture_image(
        &self,
        texture_name: impl AsRef<TextureName>,
        level: i32,
        format: impl Into<Format>,
        component_format: impl Into<ComponentFormat>,
        data: &mut [u8],
    ) {
        self.gl.GetTextureImage(
            texture_name.as_ref().to_u32(),
            level,
            format.into() as u32,
            component_format.into() as u32,
            data.len() as i32,
            data.as_mut_ptr() as *mut c_void,
        );
    }

    #[inline]
    pub unsafe fn clear_tex_image(
        &self,
        texture_name: impl AsRef<TextureName>,
        level: i32,
        format: impl Into<Format>,
        ty: impl Into<ComponentFormat>,
        data: *const c_void,
    ) {
        self.gl.ClearTexImage(
            texture_name.as_ref().to_u32(),
            level,
            format.into() as u32,
            ty.into() as u32,
            data
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
            .DeleteRenderbuffers(1, &ManuallyDrop::new(name).to_u32());
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
            .BindRenderbuffer(target.into() as u32, name.to_u32());
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
            name.to_u32(),
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
        self.gl.DeleteBuffers(1, &ManuallyDrop::new(name).to_u32());
    }

    #[inline]
    pub unsafe fn bind_buffer<T>(&self, target: T, name: BufferName)
    where
        T: Into<BufferTarget>,
    {
        self.gl.BindBuffer(target.into() as u32, name.to_u32());
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
    pub unsafe fn invalidate_buffer_data(&self, name: BufferName) {
        self.gl.InvalidateBufferData(name.to_u32());
    }

    #[inline]
    pub unsafe fn named_buffer_data<U>(&self, name: BufferName, bytes: &[u8], usage: U)
    where
        U: Into<BufferUsage>,
    {
        self.gl.NamedBufferData(
            name.to_u32(),
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
            name.to_u32(),
            capacity as isize,
            std::ptr::null(),
            usage.into() as u32,
        );
    }

    #[inline]
    pub unsafe fn named_buffer_sub_data(&self, name: BufferName, offset: usize, bytes: &[u8]) {
        self.gl.NamedBufferSubData(
            name.to_u32(),
            offset as isize,
            bytes.len() as isize,
            bytes.as_ptr() as *const c_void,
        );
    }

    #[inline]
    pub unsafe fn named_buffer_storage(
        &self,
        name: impl AsRef<BufferName>,
        bytes: &[u8],
        flags: BufferStorageFlag,
    ) {
        self.gl.NamedBufferStorage(
            name.as_ref().to_u32(),
            bytes.len() as isize,
            bytes.as_ptr() as *const c_void,
            flags.bits(),
        );
    }

    #[inline]
    pub unsafe fn named_buffer_storage_reserve(
        &self,
        name: impl AsRef<BufferName>,
        byte_size: usize,
        flags: BufferStorageFlag,
    ) {
        self.gl.NamedBufferStorage(
            name.as_ref().to_u32(),
            byte_size as isize,
            std::ptr::null(),
            flags.bits(),
        );
    }

    #[inline]
    pub unsafe fn copy_named_buffer_sub_data(
        &self,
        read_buffer_name: impl AsRef<BufferName>,
        write_buffer_name: impl AsRef<BufferName>,
        read_byte_offset: usize,
        write_byte_offset: usize,
        byte_count: usize,
    ) {
        self.gl.CopyNamedBufferSubData(
            read_buffer_name.as_ref().to_u32(),
            write_buffer_name.as_ref().to_u32(),
            read_byte_offset as isize,
            write_byte_offset as isize,
            byte_count as isize,
        );
    }

    #[inline]
    pub unsafe fn clear_named_buffer_sub_data(
        &self,
        buffer_name: impl AsRef<BufferName>,
        internal_format: impl Into<InternalFormat>,
        byte_offset: usize,
        byte_count: usize,
        format: impl Into<Format>,
        ty: impl Into<ComponentFormat>,
        bytes: Option<&[u8]>,
    ) {
        self.gl.ClearNamedBufferSubData(
            buffer_name.as_ref().to_u32(),
            internal_format.into() as u32,
            byte_offset as isize,
            byte_count as isize,
            format.into() as u32,
            ty.into() as u32,
            match bytes {
                Some(bytes) => bytes.as_ptr() as *const c_void,
                None => std::ptr::null(),
            },
        );
    }

    #[inline]
    pub unsafe fn get_named_buffer_sub_data(
        &self,
        buffer_name: impl AsRef<BufferName>,
        byte_offset: usize,
        bytes: &mut [u8],
    ) {
        self.gl.GetNamedBufferSubData(
            buffer_name.as_ref().to_u32(),
            byte_offset as isize,
            bytes.len() as isize,
            bytes.as_ptr() as *mut c_void,
        );
    }

    #[inline]
    pub unsafe fn map_named_buffer(
        &self,
        buffer_name: impl AsRef<BufferName>,
        access: MapAccessFlag,
    ) -> *mut c_void {
        self.gl
            .MapNamedBuffer(buffer_name.as_ref().to_u32(), access.bits())
    }

    #[inline]
    pub unsafe fn unmap_named_buffer(&self, buffer_name: impl AsRef<BufferName>) {
        self.gl.UnmapNamedBuffer(buffer_name.as_ref().to_u32());
    }

    /// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferRange.xhtml
    #[inline]
    pub unsafe fn map_named_buffer_range(
        &self,
        buffer_name: impl AsRef<BufferName>,
        byte_offset: usize,
        byte_count: usize,
        access: MapRangeAccessFlag,
    ) -> *mut c_void {
        self.gl.MapNamedBufferRange(
            buffer_name.as_ref().to_u32(),
            byte_offset as isize,
            byte_count as isize,
            access.bits(),
        )
    }

    #[inline]
    pub unsafe fn flush_mapped_named_buffer_range(
        &self,
        buffer_name: impl AsRef<BufferName>,
        byte_offset: usize,
        byte_count: usize,
    ) {
        self.gl.FlushMappedNamedBufferRange(
            buffer_name.as_ref().to_u32(),
            byte_offset as isize,
            byte_count as isize,
        );
    }

    #[inline]
    pub unsafe fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: impl Into<Format>,
        ty: impl Into<ComponentFormat>,
        data: *mut c_void,
    ) {
        self.gl.ReadPixels(
            x,
            y,
            width,
            height,
            format.into() as u32,
            ty.into() as u32,
            data,
        )
    }

    // Vertex array names.

    #[deprecated]
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, vertex_array_names: &mut [Option<VertexArrayName>]) {
        self.gl.GenVertexArrays(
            vertex_array_names.len() as i32,
            vertex_array_names.as_mut_ptr() as *mut u32,
        );
    }

    #[inline]
    pub unsafe fn create_vertex_array(&self) -> VertexArrayName {
        self.try_create_vertex_array().unwrap()
    }

    #[inline]
    pub unsafe fn try_create_vertex_array(
        &self,
    ) -> Result<VertexArrayName, ReceivedInvalidVertexArrayName> {
        let mut vertex_array_name = MaybeUninit::<u32>::uninit();
        self.gl
            .CreateVertexArrays(1, vertex_array_name.as_mut_ptr());
        VertexArrayName::new(vertex_array_name.assume_init())
    }

    #[deprecated]
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, vertex_array_names: &mut [Option<VertexArrayName>]) {
        self.gl.DeleteVertexArrays(
            vertex_array_names.len() as i32,
            vertex_array_names.as_ptr() as *const u32,
        );
    }

    #[inline]
    pub unsafe fn delete_vertex_array(&self, vertex_array_name: VertexArrayName) {
        self.gl
            .DeleteVertexArrays(1, &ManuallyDrop::new(vertex_array_name).to_u32());
    }

    #[inline]
    pub unsafe fn bind_vertex_array(&self, vertex_array_name: VertexArrayName) {
        self.gl.BindVertexArray(vertex_array_name.to_u32());
    }

    #[inline]
    pub unsafe fn unbind_vertex_array(&self) {
        self.gl.BindVertexArray(0);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn bind_vertex_buffer(
        &self,
        index: VertexArrayBufferBindingIndex,
        buffer: BufferName,
        offset: usize,
        stride: u32,
    ) {
        self.gl.BindVertexBuffer(
            index.to_u32(),
            buffer.to_u32(),
            offset as isize,
            stride as i32,
        );
    }

    #[inline]
    pub unsafe fn vertex_array_vertex_buffer(
        &self,
        vertex_array_name: VertexArrayName,
        index: VertexArrayBufferBindingIndex,
        buffer_name: BufferName,
        offset: usize,
        stride: u32,
    ) {
        self.gl.VertexArrayVertexBuffer(
            vertex_array_name.to_u32(),
            index.to_u32(),
            buffer_name.to_u32(),
            offset as isize,
            stride as i32,
        );
    }

    #[inline]
    pub unsafe fn vertex_array_vertex_buffers(
        &self,
        vertex_array_name: VertexArrayName,
        first_vertex_array_buffer_binding_index: VertexArrayBufferBindingIndex,
        buffer_names: &[BufferName],
        offsets: &[usize],
        strides: &[u32],
    ) {
        let count = buffer_names.len();
        assert_eq!(count, offsets.len());
        assert_eq!(count, strides.len());
        self.gl.VertexArrayVertexBuffers(
            vertex_array_name.to_u32(),
            first_vertex_array_buffer_binding_index.to_u32(),
            count as i32,
            buffer_names.as_ptr() as *const u32,
            offsets.as_ptr() as *const isize,
            strides.as_ptr() as *const i32,
        );
    }

    #[inline]
    pub unsafe fn vertex_array_element_buffer(
        &self,
        vertex_array_name: VertexArrayName,
        element_buffer_name: BufferName,
    ) {
        self.gl
            .VertexArrayElementBuffer(vertex_array_name.to_u32(), element_buffer_name.to_u32());
    }

    #[deprecated]
    #[inline]
    pub unsafe fn vertex_binding_divisor(
        &self,
        attribute_location: AttributeLocation,
        divisor: u32,
    ) {
        self.gl
            .VertexBindingDivisor(attribute_location.to_u32(), divisor);
    }

    #[inline]
    pub unsafe fn vertex_array_binding_divisor(
        &self,
        vertex_array_name: VertexArrayName,
        index: VertexArrayBufferBindingIndex,
        divisor: u32,
    ) {
        self.gl
            .VertexArrayBindingDivisor(vertex_array_name.to_u32(), index.to_u32(), divisor);
    }

    #[deprecated]
    #[inline]
    pub unsafe fn vertex_attrib_binding(
        &self,
        attribute_location: AttributeLocation,
        vertex_array_buffer_binding_index: VertexArrayBufferBindingIndex,
    ) {
        self.gl.VertexAttribBinding(
            attribute_location.to_u32(),
            vertex_array_buffer_binding_index.to_u32(),
        );
    }

    #[inline]
    pub unsafe fn vertex_array_attrib_binding(
        &self,
        vertex_array_name: VertexArrayName,
        attribute_location: AttributeLocation,
        vertex_array_buffer_binding_index: VertexArrayBufferBindingIndex,
    ) {
        self.gl.VertexArrayAttribBinding(
            vertex_array_name.to_u32(),
            attribute_location.to_u32(),
            vertex_array_buffer_binding_index.to_u32(),
        );
    }

    #[deprecated]
    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, attribute_location: AttributeLocation) {
        self.gl.EnableVertexAttribArray(attribute_location.to_u32());
    }

    #[inline]
    pub unsafe fn enable_vertex_array_attrib(
        &self,
        vertex_array_name: VertexArrayName,
        attribute_location: AttributeLocation,
    ) {
        self.gl
            .EnableVertexArrayAttrib(vertex_array_name.to_u32(), attribute_location.to_u32());
    }

    #[deprecated]
    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, attribute_location: AttributeLocation) {
        self.gl
            .DisableVertexAttribArray(attribute_location.to_u32());
    }

    #[inline]
    pub unsafe fn disable_vertex_array_attrib(
        &self,
        vertex_array_name: VertexArrayName,
        attribute_location: AttributeLocation,
    ) {
        self.gl
            .DisableVertexArrayAttrib(vertex_array_name.to_u32(), attribute_location.to_u32());
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
            .DeleteFramebuffers(1, &ManuallyDrop::new(name).to_u32());
    }

    #[inline]
    pub unsafe fn bind_framebuffer<T, N>(&self, target: T, name: N)
    where
        T: Into<FramebufferTarget>,
        N: Into<FramebufferName>,
    {
        self.gl
            .BindFramebuffer(target.into() as u32, name.into().to_u32())
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
            .CheckNamedFramebufferStatus(name.into().to_u32(), target.into() as u32)
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
            framebuffer_attachment.into().to_u32(),
            texture_target.into() as u32,
            texture_name.to_u32(),
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
            framebuffer_name.to_u32(),
            framebuffer_attachment.into().to_u32(),
            texture_name.to_u32(),
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
            framebuffer_attachment.into().to_u32(),
            renderbuffer_target.into() as u32,
            renderbuffer.to_u32(),
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
            framebuffer_name.to_u32(),
            framebuffer_attachment.into().to_u32(),
            renderbuffer_target.into() as u32,
            renderbuffer_name.to_u32(),
        );
    }

    #[inline]
    pub unsafe fn uniform_1iv(&self, uniform_location: UniformLocation, value: &[i32]) {
        self.gl.Uniform1iv(
            uniform_location.to_i32(),
            value.len() as i32,
            value.as_ptr(),
        );
    }

    #[inline]
    pub unsafe fn uniform_1fv(&self, uniform_location: UniformLocation, value: &[f32]) {
        self.gl.Uniform1fv(
            uniform_location.to_i32(),
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
            uniform_location.to_i32(),
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
            uniform_location.to_i32(),
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
        OptionUniformBlockIndex::from_u32(
            self.gl
                .GetUniformBlockIndex(program_name.to_u32(), uniform_block_name.as_ptr()),
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
            program_name.to_u32(),
            uniform_block_index.to_u32(),
            uniform_block_binding,
        );
    }

    #[inline]
    pub unsafe fn bind_buffer_base<T>(&self, target: T, index: u32, buffer_name: BufferName)
    where
        T: Into<BindBufferTarget>,
    {
        self.gl
            .BindBufferBase(target.into() as u32, index, buffer_name.to_u32());
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
            buffer_name.to_u32(),
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
        self.gl.DeleteSamplers(1, &ManuallyDrop::new(name).to_u32());
    }

    #[inline]
    pub unsafe fn bind_sampler(&self, unit: u32, name: SamplerName) {
        self.gl.BindSampler(unit, name.to_u32());
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
        self.gl
            .SamplerParameteri(sampler.to_u32(), P::VALUE, value.into().into().cast_into());
    }

    // Queries.

    #[inline]
    pub unsafe fn create_query(&self, target: impl Into<QueryTarget>) -> QueryName {
        self.try_create_query(target).unwrap()
    }

    #[inline]
    pub unsafe fn try_create_query(
        &self,
        target: impl Into<QueryTarget>,
    ) -> Result<QueryName, ReceivedInvalidQueryName> {
        let mut name = MaybeUninit::<u32>::uninit();
        self.gl
            .CreateQueries(target.into() as u32, 1, name.as_mut_ptr());
        QueryName::new(name.assume_init())
    }

    #[inline]
    pub unsafe fn create_queries(
        &self,
        target: impl Into<QueryTarget>,
        count: usize,
    ) -> Vec<QueryName> {
        self.try_create_queries(target, count)
            .into_iter()
            .map(|name| name.unwrap())
            .collect()
    }

    #[inline]
    pub unsafe fn try_create_queries(
        &self,
        target: impl Into<QueryTarget>,
        count: usize,
    ) -> Vec<Result<QueryName, ReceivedInvalidQueryName>> {
        let mut names: Vec<Result<QueryName, ReceivedInvalidQueryName>> = Vec::with_capacity(count);
        self.gl.CreateQueries(
            target.into() as u32,
            i32::try_from(count).unwrap(),
            names.as_mut_ptr() as *mut u32,
        );
        names.set_len(count);
        names
    }

    #[inline]
    pub unsafe fn delete_query(&self, query_name: QueryName) {
        self.gl
            .DeleteQueries(1, &ManuallyDrop::new(query_name).to_u32());
    }

    #[inline]
    pub unsafe fn delete_queries(&self, query_names: Vec<QueryName>) {
        let query_names = ManuallyDrop::new(query_names);
        self.gl.DeleteQueries(
            i32::try_from(query_names.len()).unwrap(),
            query_names.as_ptr() as *const u32,
        );
    }

    #[inline]
    pub unsafe fn begin_query(
        &self,
        target: impl Into<ScopeQueryTarget>,
        query_name: impl AsRef<QueryName>,
    ) {
        self.gl
            .BeginQuery(target.into() as u32, query_name.as_ref().to_u32());
    }

    #[inline]
    pub unsafe fn end_query(&self, target: impl Into<ScopeQueryTarget>) {
        self.gl.EndQuery(target.into() as u32);
    }

    #[inline]
    pub unsafe fn query_counter(&self, query_name: impl AsRef<QueryName>) {
        self.gl
            .QueryCounter(query_name.as_ref().to_u32(), gl::TIMESTAMP);
    }

    /// Blocking.
    #[inline]
    pub unsafe fn query_result_u64(&self, query_name: impl AsRef<QueryName>) -> u64 {
        let mut value = MaybeUninit::<u64>::uninit();
        self.gl.GetQueryObjectui64v(
            query_name.as_ref().to_u32(),
            gl::QUERY_RESULT,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }

    /// Non-blocking.
    #[inline]
    pub unsafe fn try_query_result_u64(
        &self,
        query_name: impl AsRef<QueryName>,
    ) -> Option<NonZeroU64> {
        let mut value = 0u64;
        self.gl.GetQueryObjectui64v(
            query_name.as_ref().to_u32(),
            gl::QUERY_RESULT_NO_WAIT,
            &mut value,
        );
        NonZeroU64::new(value)
    }

    #[inline]
    pub unsafe fn query_result_available(&self, query_name: impl AsRef<QueryName>) -> bool {
        let mut value = MaybeUninit::<i32>::uninit();
        self.gl.GetQueryObjectiv(
            query_name.as_ref().to_u32(),
            gl::QUERY_RESULT_AVAILABLE,
            value.as_mut_ptr(),
        );
        value.assume_init() != 0
    }


    // Sync objects

    #[inline]
    pub unsafe fn fence_sync(&self) -> SyncName {
        self.try_fence_sync().unwrap()
    }
    #[inline]
    pub unsafe fn try_fence_sync(&self) -> Result<SyncName, ReceivedInvalidSyncName> {
        SyncName::new(self.gl.FenceSync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0))
    }

    #[inline]
    pub unsafe fn delete_sync(&self, name: SyncName) {
        self.gl.DeleteSync(ManuallyDrop::new(name).to_gl());
    }

    #[inline]
    pub unsafe fn wait_sync(&self, name: SyncName) {
        self.gl.WaitSync(name.to_gl(), 0, gl::TIMEOUT_IGNORED)
    }
}
