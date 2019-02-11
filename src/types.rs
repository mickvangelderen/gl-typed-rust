//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;
use crate::traits;

// TODO: Some of the raw types probably never will be used. Figure out if we
// want to expose a raw variant for all enums blindly or only specific ones.

macro_rules! impl_enums_u32 {
    ($($(#[$rm:meta])* $r:ident $(#[$em:meta])* $e:ident { $($v:ident = $g:path,)* })*) => {
        $(
            $(#[$rm])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $r(u32);

            unsafe impl traits::TransmuteMarker<u32> for $r {}
            unsafe impl traits::TransmuteMarker<i32> for $r {}

            $(#[$em])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr(u32)]
            pub enum $e {
                $(
                    $v = $g,
                )*
            }

            impl $e {
                /// # Warning
                /// The given value must have a corresponding value, UB
                /// otherwise. Consider using [$e::from] or compare the value in
                /// the raw domain: `raw == $r::from($e::<SomeVariant>)`.
                #[inline]
                pub unsafe fn from_unchecked(raw: $r) -> Self {
                    std::mem::transmute(raw)
                }
            }

            impl traits::FromExt<$e> for i32 {
                #[inline]
                fn from(val: $e) -> i32 {
                    val as u32 as i32
                }
            }

            impl traits::FromExt<$e> for u32 {
                #[inline]
                fn from(val: $e) -> u32 {
                    val as u32
                }
            }

            impl From<$e> for $r {
                #[inline]
                fn from(val: $e) -> Self {
                    $r(val as u32)
                }
            }

            impl From<$r> for $e {
                /// # Panics
                /// Panics when the passed value does not correspond to any of
                /// the known variants.
                #[inline]
                fn from(raw: $r) -> Self {
                    match raw.0 {
                        $(
                            $g => $e::$v,
                        )*
                        v => panic!("No known variant corresponds to {}.", v),
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
            )*
        )*
    }
}

const TRUE: u32 = gl::TRUE as u32;
const FALSE: u32 = gl::FALSE as u32;

impl_enums_u32! {
    RawShaderKind
    /// The kind of a shader.
    ShaderKind {
        Compute = gl::COMPUTE_SHADER,
        Vertex = gl::VERTEX_SHADER,
        TessControl = gl::TESS_CONTROL_SHADER,
        TessEvaluation = gl::TESS_EVALUATION_SHADER,
        Geometry = gl::GEOMETRY_SHADER,
        Fragment = gl::FRAGMENT_SHADER,
    }

    RawShaderCompileStatus
    /// The compile status of a shader.
    ShaderCompileStatus {
        Uncompiled = FALSE,
        Compiled = TRUE,
    }

    RawProgramLinkStatus
    /// The compile status of a program.
    ProgramLinkStatus {
        Unlinked = FALSE,
        Linked = TRUE,
    }

    RawGetShaderivParam
    /// Allowed values for the pname arguments of `glGetShaderiv`.
    GetShaderivParam {
        ShaderType = gl::SHADER_TYPE,
        DeleteStatus = gl::DELETE_STATUS,
        CompileStatus = gl::COMPILE_STATUS,
        InfoLogLength = gl::INFO_LOG_LENGTH,
        ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
    }

    RawGetProgramivParam
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

    RawGetStringParam
    /// Allowed values for the pname argument of `glGetString`.
    GetStringParam {
        Renderer = gl::RENDERER,
        Vendor = gl::VENDOR,
        Version = gl::VERSION,
        ShadingLanguageVersion = gl::SHADING_LANGUAGE_VERSION,
    }

    RawTexImage2DTarget
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

    RawTextureTarget
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

    RawTextureTargetGE2D
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

    RawTextureTargetGE3D
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

    RawDepthStencilTextureMode
    DepthStencilTextureMode {
        DepthComponent = gl::DEPTH_COMPONENT,
        StencilIndex = gl::STENCIL_INDEX,
    }

    RawTexParameteriParam
    TexParameteriParam {
        DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
        TextureBaseLevel = gl::TEXTURE_BASE_LEVEL,
        TextureMagFilter = gl::TEXTURE_MAG_FILTER,
        TextureMinFilter = gl::TEXTURE_MIN_FILTER,
        TextureWrapS = gl::TEXTURE_WRAP_S,
        TextureWrapT = gl::TEXTURE_WRAP_T,
        TextureWrapR = gl::TEXTURE_WRAP_R,
    }

    RawTextureMagFilter
    TextureMagFilter {
        Nearest = gl::NEAREST,
        Linear = gl::LINEAR,
    }

    RawTextureMinFilter
    TextureMinFilter {
        Nearest = gl::NEAREST,
        Linear = gl::LINEAR,
        NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
        NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
        LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
        LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    }

    RawTextureWrap
    TextureWrap {
        ClampToEdge = gl::CLAMP_TO_EDGE,
        Repeat = gl::REPEAT,
        ClampToBorder = gl::CLAMP_TO_BORDER,
        MirroredRepeat = gl::MIRRORED_REPEAT,
        MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MaxCombinedTextureImageUnits(u32);

impl MaxCombinedTextureImageUnits {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

unsafe impl traits::TransmuteMarker<u32> for MaxCombinedTextureImageUnits {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(unit: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if unit < max.as_u32() {
            Some(TextureUnit(gl::TEXTURE0 + unit))
        } else {
            None
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(unit: u32) -> Self {
        TextureUnit(gl::TEXTURE0 + unit)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
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
