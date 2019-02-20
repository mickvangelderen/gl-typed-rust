//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::convert::Transmute;
use crate::gl;
use std::convert::TryFrom;

// TODO: Some of the unchecked types probably never will be used. Figure out if we
// want to expose a unchecked variant for all enums blindly or only specific ones.

macro_rules! impl_enums {
    ($($(#[$rm:meta])* struct $r:ident($b:ident); $(#[$em:meta])* $e:ident { $($v:ident = $g:path,)* })*) => {
        $(
            $(#[$rm])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $r($b);

            unsafe impl Transmute<$b> for $r {
                #[inline]
                fn transmute_from(val: $b) -> Self {
                    $r(val)
                }

                #[inline]
                fn transmute_into(self) -> $b {
                    self.0
                }

                #[inline]
                fn transmute_as_ref(&self) -> &$b {
                    &self.0
                }

                #[inline]
                fn transmute_as_mut(&mut self) -> &mut $b {
                    &mut self.0
                }
            }

            impl From<$e> for $r {
                #[inline]
                fn from(val: $e) -> Self {
                    $r(val as $b)
                }
            }

            $(
                impl From<crate::symbols::$v> for $r {
                    #[inline]
                    fn from(_: crate::symbols::$v) -> Self {
                        $r($g as $b)
                    }
                }

                impl TryFrom<$r> for crate::symbols::$v {
                    type Error = $r;

                    #[inline]
                    fn try_from(val: $r) -> Result<Self, Self::Error> {
                        if val == crate::symbols::$v.into() {
                            Ok(crate::symbols::$v)
                        } else {
                            Err(val)
                        }
                    }
                }
            )*

            $(#[$em])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr($b)]
            pub enum $e {
                $(
                    $v = $g as $b,
                )*
            }

            impl From<$e> for $b {
                #[inline]
                fn from(val: $e) -> Self {
                    val as $b
                }
            }

            impl TryFrom<$r> for $e {
                type Error = $r;

                #[inline]
                fn try_from(val: $r) -> Result<Self, Self::Error> {
                    match val {
                        $(
                            x if x.0 == $g as $b => Ok($e::$v),
                        )*
                            other => Err(other)
                    }
                }
            }

            $(
                impl From<crate::symbols::$v> for $e {
                    #[inline]
                    fn from(_: crate::symbols::$v) -> Self {
                        $e::$v
                    }
                }

                impl TryFrom<$e> for crate::symbols::$v {
                    type Error = $e;

                    #[inline]
                    fn try_from(val: $e) -> Result<Self, Self::Error> {
                        if val == crate::symbols::$v.into() {
                            Ok(crate::symbols::$v)
                        } else {
                            Err(val)
                        }
                    }
                }
            )*
        )*
    }
}

impl_enums! {
    struct UncheckedShaderKind(u32);
    /// The kind of a shader.
    ShaderKind {
        ComputeShader = gl::COMPUTE_SHADER,
        VertexShader = gl::VERTEX_SHADER,
        TessControlShader = gl::TESS_CONTROL_SHADER,
        TessEvaluationShader = gl::TESS_EVALUATION_SHADER,
        GeometryShader = gl::GEOMETRY_SHADER,
        FragmentShader = gl::FRAGMENT_SHADER,
    }

    struct UncheckedShaderCompileStatus(i32);
    /// The compile status of a shader.
    ShaderCompileStatus {
        Uncompiled = gl::FALSE,
        Compiled = gl::TRUE,
    }

    struct UncheckedProgramLinkStatus(i32);
    /// The compile status of a program.
    ProgramLinkStatus {
        Unlinked = gl::FALSE,
        Linked = gl::TRUE,
    }

    struct UncheckedGetShaderivParam(u32);
    /// Allowed values for the pname arguments of `glGetShaderiv`.
    GetShaderivParam {
        ShaderType = gl::SHADER_TYPE,
        DeleteStatus = gl::DELETE_STATUS,
        CompileStatus = gl::COMPILE_STATUS,
        InfoLogLength = gl::INFO_LOG_LENGTH,
        ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
    }

    struct UncheckedGetProgramivParam(u32);
    /// Allowed values for the pname arguments of `glGetProgramiv`.
    GetProgramivParam {
        ActiveAtomicCounterBuffers = gl::ACTIVE_ATOMIC_COUNTER_BUFFERS,
        ActiveAttributeMaxLength = gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
        ActiveAttributes = gl::ACTIVE_ATTRIBUTES,
        ActiveUniformBlockMaxNameLength = gl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
        ActiveUniformBlocks = gl::ACTIVE_UNIFORM_BLOCKS,
        ActiveUniformMaxLength = gl::ACTIVE_UNIFORM_MAX_LENGTH,
        ActiveUniforms = gl::ACTIVE_UNIFORMS,
        AttachedShaders = gl::ATTACHED_SHADERS,
        ComputeWorkGroupSize = gl::COMPUTE_WORK_GROUP_SIZE,
        DeleteStatus = gl::DELETE_STATUS,
        GeometryInputType = gl::GEOMETRY_INPUT_TYPE,
        GeometryOutputType = gl::GEOMETRY_OUTPUT_TYPE,
        GeometryShaderInvocations = gl::GEOMETRY_SHADER_INVOCATIONS,
        GeometryVerticesOut = gl::GEOMETRY_VERTICES_OUT,
        InfoLogLength = gl::INFO_LOG_LENGTH,
        LinkStatus = gl::LINK_STATUS,
        ProgramBinaryRetrievableHint = gl::PROGRAM_BINARY_RETRIEVABLE_HINT,
        ProgramSeparable = gl::PROGRAM_SEPARABLE,
        TessControlOutputVertices = gl::TESS_CONTROL_OUTPUT_VERTICES,
        TessGenMode = gl::TESS_GEN_MODE,
        TessGenPointMode = gl::TESS_GEN_POINT_MODE,
        TessGenVertexOrder = gl::TESS_GEN_VERTEX_ORDER,
        TransformFeedbackBufferMode = gl::TRANSFORM_FEEDBACK_BUFFER_MODE,
        TransformFeedbackVaryingMaxLength = gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
        TransformFeedbackVaryings = gl::TRANSFORM_FEEDBACK_VARYINGS,
        ValidateStatus = gl::VALIDATE_STATUS,
    }

    struct UncheckedGetStringParam(u32);
    /// Allowed values for the pname argument of `glGetString`.
    GetStringParam {
        Renderer = gl::RENDERER,
        Vendor = gl::VENDOR,
        Version = gl::VERSION,
        ShadingLanguageVersion = gl::SHADING_LANGUAGE_VERSION,
    }

    struct UncheckedTexImage2DTarget(u32);
    /// Allowed values for the target argument of `glTexImage2D`.
    TexImage2DTarget {
        ProxyTexture1DArray = gl::PROXY_TEXTURE_1D_ARRAY,
        ProxyTexture2D = gl::PROXY_TEXTURE_2D,
        ProxyTextureCubeMap = gl::PROXY_TEXTURE_CUBE_MAP,
        ProxyTextureRectangle = gl::PROXY_TEXTURE_RECTANGLE,
        Texture1DArray = gl::TEXTURE_1D_ARRAY,
        Texture2D = gl::TEXTURE_2D,
        TextureCubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        TextureCubeMapNegativeY = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        TextureCubeMapNegativeZ = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        TextureCubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        TextureCubeMapPositiveY = gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        TextureCubeMapPositiveZ = gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        TextureRectangle = gl::TEXTURE_RECTANGLE,
    }

    struct UncheckedTextureTarget(u32);
    TextureTarget {
        Texture1D = gl::TEXTURE_1D,
        Texture2D = gl::TEXTURE_2D,
        Texture3D = gl::TEXTURE_3D,
        Texture1DArray = gl::TEXTURE_1D_ARRAY,
        Texture2DArray = gl::TEXTURE_2D_ARRAY,
        TextureRectangle = gl::TEXTURE_RECTANGLE,
        TextureBuffer = gl::TEXTURE_BUFFER,
        TextureCubeMap = gl::TEXTURE_CUBE_MAP,
        TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
        Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
        Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    }

    struct UncheckedTextureTargetGE2D(u32);
    TextureTargetGE2D {
        // Texture1D = gl::TEXTURE_1D,
        Texture2D = gl::TEXTURE_2D,
        Texture3D = gl::TEXTURE_3D,
        // Texture1DArray = gl::TEXTURE_1D_ARRAY,
        Texture2DArray = gl::TEXTURE_2D_ARRAY,
        TextureRectangle = gl::TEXTURE_RECTANGLE,
        TextureBuffer = gl::TEXTURE_BUFFER,
        TextureCubeMap = gl::TEXTURE_CUBE_MAP,
        TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
        Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
        Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    }

    struct UncheckedTextureTargetGE3D(u32);
    TextureTargetGE3D {
        // Texture1D = gl::TEXTURE_1D,
        // Texture2D = gl::TEXTURE_2D,
        Texture3D = gl::TEXTURE_3D,
        // Texture1DArray = gl::TEXTURE_1D_ARRAY,
        // Texture2DArray = gl::TEXTURE_2D_ARRAY,
        // TextureRectangle = gl::TEXTURE_RECTANGLE,
        TextureBuffer = gl::TEXTURE_BUFFER,
        // TextureCubeMap = gl::TEXTURE_CUBE_MAP,
        // TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
        // Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
        // Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    }

    struct UncheckedDepthStencilTextureMode(i32);
    DepthStencilTextureMode {
        DepthComponent = gl::DEPTH_COMPONENT,
        StencilIndex = gl::STENCIL_INDEX,
    }

    struct UncheckedTexParameteriParam(i32);
    TexParameteriParam {
        DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
        TextureBaseLevel = gl::TEXTURE_BASE_LEVEL,
        TextureMagFilter = gl::TEXTURE_MAG_FILTER,
        TextureMinFilter = gl::TEXTURE_MIN_FILTER,
        TextureWrapS = gl::TEXTURE_WRAP_S,
        TextureWrapT = gl::TEXTURE_WRAP_T,
        TextureWrapR = gl::TEXTURE_WRAP_R,
    }

    struct UncheckedTextureMagFilter(i32);
    TextureMagFilter {
        Nearest = gl::NEAREST,
        Linear = gl::LINEAR,
    }

    struct UncheckedTextureMinFilter(i32);
    TextureMinFilter {
        Nearest = gl::NEAREST,
        Linear = gl::LINEAR,
        NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
        NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
        LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
        LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    }

    struct UncheckedTextureWrap(i32);
    TextureWrap {
        ClampToEdge = gl::CLAMP_TO_EDGE,
        Repeat = gl::REPEAT,
        ClampToBorder = gl::CLAMP_TO_BORDER,
        MirroredRepeat = gl::MIRRORED_REPEAT,
        MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
    }

    struct UncheckedFramebufferTarget(u32);
    FramebufferTarget {
        DrawFramebuffer = gl::DRAW_FRAMEBUFFER,
        ReadFramebuffer = gl::READ_FRAMEBUFFER,
        Framebuffer = gl::FRAMEBUFFER,
    }

    struct UncheckedFramebufferStatus(u32);
    FramebufferStatus {
        FramebufferUndefined = gl::FRAMEBUFFER_UNDEFINED,
        FramebufferIncompleteAttachment = gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
        FramebufferIncompleteMissingAttachment = gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
        FramebufferIncompleteDrawBuffer = gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
        FramebufferIncompleteReadBuffer = gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
        FramebufferUnsupported = gl::FRAMEBUFFER_UNSUPPORTED,
        FramebufferIncompleteMultisample = gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
        FramebufferIncompleteLayerTargets = gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
    }

    struct UncheckedBufferTarget(u32);
    BufferTarget {
        ArrayBuffer = gl::ARRAY_BUFFER,
        AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
        CopyReadBuffer = gl::COPY_READ_BUFFER,
        CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
        DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
        DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
        ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
        PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
        PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
        QueryBuffer = gl::QUERY_BUFFER,
        ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
        TextureBuffer = gl::TEXTURE_BUFFER,
        TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
        UniformBuffer = gl::UNIFORM_BUFFER,
    }
}

impl From<TextureTargetGE2D> for TextureTarget {
    #[inline]
    fn from(t: TextureTargetGE2D) -> Self {
        unsafe { std::mem::transmute(t) }
    }
}

impl From<TextureTargetGE3D> for TextureTarget {
    #[inline]
    fn from(t: TextureTargetGE3D) -> Self {
        unsafe { std::mem::transmute(t) }
    }
}

macro_rules! impl_struct_from_symbol {
    ($t:ident { $($v:ident = $g:path,)* }) => {
        $(
            impl From<crate::symbols::$v> for $t {
                #[inline]
                fn from(_: crate::symbols::$v) -> Self {
                    $t($g)
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MaxCombinedTextureImageUnits(u32);

impl MaxCombinedTextureImageUnits {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(index: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if index < max.as_u32() {
            Some(TextureUnit(gl::TEXTURE0 + index))
        } else {
            None
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        TextureUnit(gl::TEXTURE0 + index)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl_struct_from_symbol! (TextureUnit {
    Texture0 = gl::TEXTURE0,
    Texture1 = gl::TEXTURE1,
    Texture2 = gl::TEXTURE2,
    Texture3 = gl::TEXTURE3,
    Texture4 = gl::TEXTURE4,
    Texture5 = gl::TEXTURE5,
    Texture6 = gl::TEXTURE6,
    Texture7 = gl::TEXTURE7,
    Texture8 = gl::TEXTURE8,
    Texture9 = gl::TEXTURE9,
    Texture10 = gl::TEXTURE10,
    Texture11 = gl::TEXTURE11,
    Texture12 = gl::TEXTURE12,
    Texture13 = gl::TEXTURE13,
    Texture14 = gl::TEXTURE14,
    Texture15 = gl::TEXTURE15,
});

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MaxColorAttachments(u32);

impl MaxColorAttachments {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct FramebufferAttachment(u32);

impl FramebufferAttachment {
    #[inline]
    pub fn new(index: u32, max: MaxColorAttachments) -> Option<Self> {
        unsafe {
            if index < max.as_u32() {
                Some(FramebufferAttachment::new_unchecked(index))
            } else {
                None
            }
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        FramebufferAttachment(gl::COLOR_ATTACHMENT0 + index)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl_struct_from_symbol! (FramebufferAttachment {
    ColorAttachment0 = gl::COLOR_ATTACHMENT0,
    ColorAttachment1 = gl::COLOR_ATTACHMENT1,
    ColorAttachment2 = gl::COLOR_ATTACHMENT2,
    ColorAttachment3 = gl::COLOR_ATTACHMENT3,
    ColorAttachment4 = gl::COLOR_ATTACHMENT4,
    ColorAttachment5 = gl::COLOR_ATTACHMENT5,
    ColorAttachment6 = gl::COLOR_ATTACHMENT6,
    ColorAttachment7 = gl::COLOR_ATTACHMENT7,
    ColorAttachment8 = gl::COLOR_ATTACHMENT8,
    ColorAttachment9 = gl::COLOR_ATTACHMENT9,
    ColorAttachment10 = gl::COLOR_ATTACHMENT10,
    ColorAttachment11 = gl::COLOR_ATTACHMENT11,
    ColorAttachment12 = gl::COLOR_ATTACHMENT12,
    ColorAttachment13 = gl::COLOR_ATTACHMENT13,
    ColorAttachment14 = gl::COLOR_ATTACHMENT14,
    ColorAttachment15 = gl::COLOR_ATTACHMENT15,
});
