//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;
use crate::convert::*;

// TODO: Some of the unchecked types probably never will be used. Figure out if we
// want to expose a unchecked variant for all enums blindly or only specific ones.

macro_rules! impl_enums {
    ($($(#[$rm:meta])* struct $r:ident($b:ident); $(#[$em:meta])* $e:ident { $($v:ident = $g:path,)* })*) => {
        $(
            $(#[$rm])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $r($b);

            // Every $r is a valid $b and vice versa.
            unsafe impl convute::marker::Transmute<$b> for $r {}
            unsafe impl convute::marker::Transmute<$r> for $b {}

            // Every $e is a valid $b.
            unsafe impl convute::marker::Transmute<$b> for $e {}

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

    struct UncheckedBindBufferTarget(u32);
    BindBufferTarget {
        AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
        TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
        UniformBuffer = gl::UNIFORM_BUFFER,
        ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    }

    struct UncheckedRenderbufferTarget(u32);
    RenderbufferTarget {
        Renderbuffer = gl::RENDERBUFFER,
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
        TextureMaxLevel = gl::TEXTURE_MAX_LEVEL,
        TextureMagFilter = gl::TEXTURE_MAG_FILTER,
        TextureMinFilter = gl::TEXTURE_MIN_FILTER,
        TextureWrapS = gl::TEXTURE_WRAP_S,
        TextureWrapT = gl::TEXTURE_WRAP_T,
        TextureWrapR = gl::TEXTURE_WRAP_R,
    }

    struct UncheckedTexParameterfParam(i32);
    TexParameterfParam {
        // ARB_texture_filter_anisotropic
        TextureMaxAnisotropy = gl::TEXTURE_MAX_ANISOTROPY,
    }

    struct UncheckedSamplerParameteriParam(i32);
    SamplerParameteri {
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
        FramebufferComplete = gl::FRAMEBUFFER_COMPLETE,
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

    struct UncheckedBufferUsage(u32);
    BufferUsage {
        StreamDraw = gl::STREAM_DRAW,
        StreamRead = gl::STREAM_READ,
        StreamCopy = gl::STREAM_COPY,
        StaticDraw = gl::STATIC_DRAW,
        StaticRead = gl::STATIC_READ,
        StaticCopy = gl::STATIC_COPY,
        DynamicDraw = gl::DYNAMIC_DRAW,
        DynamicRead = gl::DYNAMIC_READ,
        DynamicCopy = gl::DYNAMIC_COPY,
    }

    struct UncheckedVertexAttribPointerType(u32);
    VertexAttribPointerType {
        Byte = gl::BYTE,
        UnsignedByte = gl::UNSIGNED_BYTE,
        Short = gl::SHORT,
        UnsignedShort = gl::UNSIGNED_SHORT,
        Int = gl::INT,
        UnsignedInt = gl::UNSIGNED_INT,
        HalfFloat = gl::HALF_FLOAT,
        Float = gl::FLOAT,
        Double = gl::DOUBLE,
        Fixed = gl::FIXED,
        Int2101010Rev = gl::INT_2_10_10_10_REV,
        UnsignedInt2101010Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
        UnsignedInt10f11f11fRev = gl::UNSIGNED_INT_10F_11F_11F_REV,
    }

    struct UncheckedBool(u8);
    Bool {
        True = gl::TRUE,
        False = gl::FALSE,
    }

    struct UncheckedDrawMode(u32);
    DrawMode {
        Points = gl::POINTS,
        LineStrip = gl::LINE_STRIP,
        LineLoop = gl::LINE_LOOP,
        Lines = gl::LINES,
        LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
        LinesAdjacency = gl::LINES_ADJACENCY,
        TriangleStrip = gl::TRIANGLE_STRIP,
        TriangleFan = gl::TRIANGLE_FAN,
        Triangles = gl::TRIANGLES,
        TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
        TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
        Patches = gl::PATCHES,
    }

    struct UncheckedInternalFormat(i32);
    InternalFormat {
        DepthComponent = gl::DEPTH_COMPONENT,
        DepthStencil = gl::DEPTH_STENCIL,
        Red = gl::RED,
        Rg = gl::RG,
        Rgb = gl::RGB,
        Rgba = gl::RGBA,
        R8 = gl::R8,
        R8Snorm = gl::R8_SNORM,
        R16 = gl::R16,
        R16Snorm = gl::R16_SNORM,
        Rg8 = gl::RG8,
        Rg8Snorm = gl::RG8_SNORM,
        Rg16 = gl::RG16,
        Rg16Snorm = gl::RG16_SNORM,
        R3G3B2 = gl::R3_G3_B2,
        Rgb4 = gl::RGB4,
        Rgb5 = gl::RGB5,
        Rgb8 = gl::RGB8,
        Rgb8Snorm = gl::RGB8_SNORM,
        Rgb10 = gl::RGB10,
        Rgb12 = gl::RGB12,
        Rgb16Snorm = gl::RGB16_SNORM,
        Rgba2 = gl::RGBA2,
        Rgba4 = gl::RGBA4,
        Rgb5A1 = gl::RGB5_A1,
        Rgba8 = gl::RGBA8,
        Rgba8Snorm = gl::RGBA8_SNORM,
        Rgb10A2 = gl::RGB10_A2,
        Rgb10A2ui = gl::RGB10_A2UI,
        Rgba12 = gl::RGBA12,
        Rgba16 = gl::RGBA16,
        Srgb8 = gl::SRGB8,
        Srgb8Alpha8 = gl::SRGB8_ALPHA8,
        R16f = gl::R16F,
        Rg16f = gl::RG16F,
        Rgb16f = gl::RGB16F,
        Rgba16f = gl::RGBA16F,
        R32f = gl::R32F,
        Rg32f = gl::RG32F,
        Rgb32f = gl::RGB32F,
        Rgba32f = gl::RGBA32F,
        R11fG11fB10f = gl::R11F_G11F_B10F,
        Rgb9E5 = gl::RGB9_E5,
        R8i = gl::R8I,
        R8ui = gl::R8UI,
        R16i = gl::R16I,
        R16ui = gl::R16UI,
        R32i = gl::R32I,
        R32ui = gl::R32UI,
        Rg8i = gl::RG8I,
        Rg8ui = gl::RG8UI,
        Rg16i = gl::RG16I,
        Rg16ui = gl::RG16UI,
        Rg32i = gl::RG32I,
        Rg32ui = gl::RG32UI,
        Rgb8i = gl::RGB8I,
        Rgb8ui = gl::RGB8UI,
        Rgb16i = gl::RGB16I,
        Rgb16ui = gl::RGB16UI,
        Rgb32i = gl::RGB32I,
        Rgb32ui = gl::RGB32UI,
        Rgba8i = gl::RGBA8I,
        Rgba8ui = gl::RGBA8UI,
        Rgba16i = gl::RGBA16I,
        Rgba16ui = gl::RGBA16UI,
        Rgba32i = gl::RGBA32I,
        Rgba32ui = gl::RGBA32UI,
        CompressedRed = gl::COMPRESSED_RED,
        CompressedRg = gl::COMPRESSED_RG,
        CompressedRgb = gl::COMPRESSED_RGB,
        CompressedRgba = gl::COMPRESSED_RGBA,
        CompressedSrgb = gl::COMPRESSED_SRGB,
        CompressedSrgbAlpha = gl::COMPRESSED_SRGB_ALPHA,
        CompressedRedRgtc1 = gl::COMPRESSED_RED_RGTC1,
        CompressedSignedRedRgtc1 = gl::COMPRESSED_SIGNED_RED_RGTC1,
        CompressedRgRgtc2 = gl::COMPRESSED_RG_RGTC2,
        CompressedSignedRgRgtc2 = gl::COMPRESSED_SIGNED_RG_RGTC2,
        CompressedRgbaBptcUnorm = gl::COMPRESSED_RGBA_BPTC_UNORM,
        CompressedSrgbAlphaBptcUnorm = gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
        CompressedRgbBptcSignedFloat = gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
        CompressedRgbBptcUnsignedFloat = gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
        DepthComponent16 = gl::DEPTH_COMPONENT16,
        DepthComponent24 = gl::DEPTH_COMPONENT24,
        DepthComponent32 = gl::DEPTH_COMPONENT32,
        DepthComponent32f = gl::DEPTH_COMPONENT32F,
        Depth24Stencil8 = gl::DEPTH24_STENCIL8,
        Depth32fStencil8 = gl::DEPTH32F_STENCIL8,
        StencilIndex1 = gl::STENCIL_INDEX1,
        StencilIndex4 = gl::STENCIL_INDEX4,
        StencilIndex8 = gl::STENCIL_INDEX8,
        StencilIndex16 = gl::STENCIL_INDEX16,
    }

    struct UncheckedFormat(i32);
    Format {
        Bgr = gl::BGR,
        BgrInteger = gl::BGR_INTEGER,
        Bgra = gl::BGRA,
        BgraInteger = gl::BGRA_INTEGER,
        Blue = gl::BLUE,
        BlueInteger = gl::BLUE_INTEGER,
        DepthComponent = gl::DEPTH_COMPONENT,
        DepthStencil = gl::DEPTH_STENCIL,
        Green = gl::GREEN,
        GreenInteger = gl::GREEN_INTEGER,
        Red = gl::RED,
        RedInteger = gl::RED_INTEGER,
        Rg = gl::RG,
        RgInteger = gl::RG_INTEGER,
        Rgb = gl::RGB,
        RgbInteger = gl::RGB_INTEGER,
        Rgba = gl::RGBA,
        RgbaInteger = gl::RGBA_INTEGER,
        StencilIndex = gl::STENCIL_INDEX,
    }

    struct UncheckedComponentFormat(i32);
    ComponentFormat {
        Byte = gl::BYTE,
        Float = gl::FLOAT,
        Float32UnsignedInt248Rev = gl::FLOAT_32_UNSIGNED_INT_24_8_REV,
        HalfFloat = gl::HALF_FLOAT,
        Int = gl::INT,
        Short = gl::SHORT,
        UnsignedByte = gl::UNSIGNED_BYTE,
        UnsignedByte233Rev = gl::UNSIGNED_BYTE_2_3_3_REV,
        UnsignedByte332 = gl::UNSIGNED_BYTE_3_3_2,
        UnsignedInt = gl::UNSIGNED_INT,
        UnsignedInt1010102 = gl::UNSIGNED_INT_10_10_10_2,
        UnsignedInt10f11f11fRev = gl::UNSIGNED_INT_10F_11F_11F_REV,
        UnsignedInt2101010Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
        UnsignedInt248 = gl::UNSIGNED_INT_24_8,
        UnsignedInt5999Rev = gl::UNSIGNED_INT_5_9_9_9_REV,
        UnsignedInt8888 = gl::UNSIGNED_INT_8_8_8_8,
        UnsignedInt8888Rev = gl::UNSIGNED_INT_8_8_8_8_REV,
        UnsignedShort = gl::UNSIGNED_SHORT,
        UnsignedShort1555Rev = gl::UNSIGNED_SHORT_1_5_5_5_REV,
        UnsignedShort4444 = gl::UNSIGNED_SHORT_4_4_4_4,
        UnsignedShort4444Rev = gl::UNSIGNED_SHORT_4_4_4_4_REV,
        UnsignedShort5551 = gl::UNSIGNED_SHORT_5_5_5_1,
        UnsignedShort565 = gl::UNSIGNED_SHORT_5_6_5,
        UnsignedShort565Rev = gl::UNSIGNED_SHORT_5_6_5_REV,
    }

    struct UncheckedDrawElementsType(u32);
    DrawElementsType {
        UnsignedByte = gl::UNSIGNED_BYTE,
        UnsignedShort = gl::UNSIGNED_SHORT,
        UnsignedInt = gl::UNSIGNED_INT,
    }

    struct UncheckedCapability(u32);
    Capability {
        Blend = gl::BLEND,
        ColorLogicOp = gl::COLOR_LOGIC_OP,
        CullFace = gl::CULL_FACE,
        DebugOutput = gl::DEBUG_OUTPUT,
        DebugOutputSynchronous = gl::DEBUG_OUTPUT_SYNCHRONOUS,
        DepthClamp = gl::DEPTH_CLAMP,
        DepthTest = gl::DEPTH_TEST,
        Dither = gl::DITHER,
        FramebufferSrgb = gl::FRAMEBUFFER_SRGB,
        LineSmooth = gl::LINE_SMOOTH,
        Multisample = gl::MULTISAMPLE,
        PolygonOffsetFill = gl::POLYGON_OFFSET_FILL,
        PolygonOffsetLine = gl::POLYGON_OFFSET_LINE,
        PolygonOffsetPoint = gl::POLYGON_OFFSET_POINT,
        PolygonSmooth = gl::POLYGON_SMOOTH,
        PrimitiveRestart = gl::PRIMITIVE_RESTART,
        PrimitiveRestartFixedIndex = gl::PRIMITIVE_RESTART_FIXED_INDEX,
        RasterizerDiscard = gl::RASTERIZER_DISCARD,
        SampleAlphaToCoverage = gl::SAMPLE_ALPHA_TO_COVERAGE,
        SampleAlphaToOne = gl::SAMPLE_ALPHA_TO_ONE,
        SampleCoverage = gl::SAMPLE_COVERAGE,
        SampleShading = gl::SAMPLE_SHADING,
        SampleMask = gl::SAMPLE_MASK,
        ScissorTest = gl::SCISSOR_TEST,
        StencilTest = gl::STENCIL_TEST,
        TextureCubeMapSeamless = gl::TEXTURE_CUBE_MAP_SEAMLESS,
        ProgramPointSize = gl::PROGRAM_POINT_SIZE,
    }

    struct UncheckedPolygonModeFace(u32);
    PolygonModeFace {
        FrontAndBack = gl::FRONT_AND_BACK,
    }

    struct UncheckedPolygonMode(u32);
    PolygonMode {
        Point = gl::POINT,
        Line = gl::LINE,
        Fill = gl::FILL,
    }

    struct UncheckedCullFace(u32);
    CullFace {
        Front = gl::FRONT,
        Back = gl::BACK,
        FrontAndBack = gl::FRONT_AND_BACK,
    }

    struct UncheckedMajorAxis(u8);
    MajorAxis {
        Column = gl::FALSE,
        Row = gl::TRUE,
    }

    struct UncheckedWriteMask(u8);
    WriteMask {
        Disabled = gl::FALSE,
        Enabled = gl::TRUE,
    }

    struct UncheckedDepthFunc(u32);
    DepthFunc {
        Never = gl::NEVER,
        LT = gl::LESS,
        EQ = gl::EQUAL,
        LE = gl::LEQUAL,
        GT = gl::GREATER,
        NE = gl::NOTEQUAL,
        GE = gl::GEQUAL,
        Always = gl::ALWAYS,
    }

    struct UncheckedClipControlOrigin(u32);
    ClipControlOrigin {
        LowerLeft = gl::LOWER_LEFT,
        UpperLeft = gl::UPPER_LEFT,
    }

    struct UncheckedClipControlDepth(u32);
    ClipControlDepth {
        NegativeOneToOne = gl::NEGATIVE_ONE_TO_ONE,
        ZeroToOne = gl::ZERO_TO_ONE,
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
    fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(index: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if index < max.into_u32() {
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
    pub fn into_u32(self) -> u32 {
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
    fn into_u32(self) -> u32 {
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
            if index < max.into_u32() {
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
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

impl_struct_from_symbol! (FramebufferAttachment {
    DepthStencilAttachment = gl::DEPTH_STENCIL_ATTACHMENT,
    DepthAttachment = gl::DEPTH_ATTACHMENT,
    StencilAttachment = gl::STENCIL_ATTACHMENT,
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

// FIXME: Bit flags and symbols don't play nice together.
bitflags::bitflags! {
    pub struct ClearFlags : u32 {
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER_BIT = gl::STENCIL_BUFFER_BIT;
    }
}

bitflags::bitflags! {
    pub struct ContextFlags: u32 {
        const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT = gl::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT;
        const CONTEXT_FLAG_DEBUG_BIT = gl::CONTEXT_FLAG_DEBUG_BIT;
        const CONTEXT_FLAG_ROBUST_ACCESS_BIT = gl::CONTEXT_FLAG_ROBUST_ACCESS_BIT;
        const CONTEXT_FLAG_NO_ERROR_BIT = gl::CONTEXT_FLAG_NO_ERROR_BIT;
    }
}
