//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

#![allow(non_camel_case_types)]

use crate::*;
use std::convert::TryFrom;

// TODO: Some of the unchecked types probably never will be used. Figure out if we
// want to expose a unchecked variant for all enums blindly or only specific ones.

macro_rules! impl_enums {
    (
        $(
            $(#[$rm:meta])*
            struct $Error:ident($Raw:ident);
            $(#[$em:meta])*
            $Enum:ident {
                $($Variant:ident = $Symbol:ident,)*
            }
        )*
    ) => {
        $(
            impl_received_invalid!($Error($Raw), $Enum);

            $(#[$em])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr($Raw)]
            pub enum $Enum {
                $(
                    $Variant = <$Symbol as Symbol<$Raw>>::VALUE,
                )*
            }

            impl From<$Enum> for $Raw {
                #[inline]
                fn from(val: $Enum) -> Self {
                    val as $Raw
                }
            }

            impl TryFrom<$Raw> for $Enum {
                type Error = $Error;

                #[inline]
                fn try_from(val: $Raw) -> Result<Self, Self::Error> {
                    match val {
                        $(
                            <$Symbol as Symbol<$Raw>>::VALUE => Ok($Enum::$Variant),
                        )*
                        invalid => Err($Error(invalid)),
                    }
                }
            }

            $(
                impl From<$Symbol> for $Enum {
                    #[inline]
                    fn from(_: $Symbol) -> Self {
                        $Enum::$Variant
                    }
                }
            )*
        )*
    }
}

impl_enums! {
    struct ReceivedInvalidShaderKind(u32);
    /// The kind of a shader.
    ShaderKind {
        ComputeShader = COMPUTE_SHADER,
        VertexShader = VERTEX_SHADER,
        TessControlShader = TESS_CONTROL_SHADER,
        TessEvaluationShader = TESS_EVALUATION_SHADER,
        GeometryShader = GEOMETRY_SHADER,
        FragmentShader = FRAGMENT_SHADER,
    }

    struct ReceivedInvalidCompileStatus(i32);
    /// The compile status of a shader.
    CompileStatus {
        Uncompiled = FALSE,
        Compiled = TRUE,
    }

    struct ReceivedInvalidLinkStatus(i32);
    /// The compile status of a program.
    LinkStatus {
        Unlinked = FALSE,
        Linked = TRUE,
    }

    // struct ReceivedInvalidGetShaderivParam(u32);
    // /// Allowed values for the pname arguments of `glGetShaderiv`.
    // GetShaderivParam {
    //     ShaderType = SHADER_TYPE,
    //     DeleteStatus = DELETE_STATUS,
    //     CompileStatus = COMPILE_STATUS,
    //     InfoLogLength = INFO_LOG_LENGTH,
    //     ShaderSourceLength = SHADER_SOURCE_LENGTH,
    // }

    // struct ReceivedInvalidGetProgramivParam(u32);
    // /// Allowed values for the pname arguments of `glGetProgramiv`.
    // GetProgramivParam {
    //     ActiveAtomicCounterBuffers = ACTIVE_ATOMIC_COUNTER_BUFFERS,
    //     ActiveAttributeMaxLength = ACTIVE_ATTRIBUTE_MAX_LENGTH,
    //     ActiveAttributes = ACTIVE_ATTRIBUTES,
    //     ActiveUniformBlockMaxNameLength = ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
    //     ActiveUniformBlocks = ACTIVE_UNIFORM_BLOCKS,
    //     ActiveUniformMaxLength = ACTIVE_UNIFORM_MAX_LENGTH,
    //     ActiveUniforms = ACTIVE_UNIFORMS,
    //     AttachedShaders = ATTACHED_SHADERS,
    //     ComputeWorkGroupSize = COMPUTE_WORK_GROUP_SIZE,
    //     DeleteStatus = DELETE_STATUS,
    //     GeometryInputType = GEOMETRY_INPUT_TYPE,
    //     GeometryOutputType = GEOMETRY_OUTPUT_TYPE,
    //     GeometryShaderInvocations = GEOMETRY_SHADER_INVOCATIONS,
    //     GeometryVerticesOut = GEOMETRY_VERTICES_OUT,
    //     InfoLogLength = INFO_LOG_LENGTH,
    //     LinkStatus = LINK_STATUS,
    //     ProgramBinaryRetrievableHint = PROGRAM_BINARY_RETRIEVABLE_HINT,
    //     ProgramSeparable = PROGRAM_SEPARABLE,
    //     TessControlOutputVertices = TESS_CONTROL_OUTPUT_VERTICES,
    //     TessGenMode = TESS_GEN_MODE,
    //     TessGenPointMode = TESS_GEN_POINT_MODE,
    //     TessGenVertexOrder = TESS_GEN_VERTEX_ORDER,
    //     TransformFeedbackBufferMode = TRANSFORM_FEEDBACK_BUFFER_MODE,
    //     TransformFeedbackVaryingMaxLength = TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    //     TransformFeedbackVaryings = TRANSFORM_FEEDBACK_VARYINGS,
    //     ValidateStatus = VALIDATE_STATUS,
    // }

    struct ReceivedInvalidGetStringParam(u32);
    /// Allowed values for the pname argument of `glGetString`.
    GetStringParam {
        Renderer = RENDERER,
        Vendor = VENDOR,
        Version = VERSION,
        ShadingLanguageVersion = SHADING_LANGUAGE_VERSION,
    }

    struct ReceivedInvalidTexImage2DTarget(u32);
    /// Allowed values for the target argument of `glTexImage2D`.
    TexImage2DTarget {
        ProxyTexture1DArray = PROXY_TEXTURE_1D_ARRAY,
        ProxyTexture2D = PROXY_TEXTURE_2D,
        ProxyTextureCubeMap = PROXY_TEXTURE_CUBE_MAP,
        ProxyTextureRectangle = PROXY_TEXTURE_RECTANGLE,
        Texture1DArray = TEXTURE_1D_ARRAY,
        Texture2D = TEXTURE_2D,
        TextureCubeMapNegativeX = TEXTURE_CUBE_MAP_NEGATIVE_X,
        TextureCubeMapNegativeY = TEXTURE_CUBE_MAP_NEGATIVE_Y,
        TextureCubeMapNegativeZ = TEXTURE_CUBE_MAP_NEGATIVE_Z,
        TextureCubeMapPositiveX = TEXTURE_CUBE_MAP_POSITIVE_X,
        TextureCubeMapPositiveY = TEXTURE_CUBE_MAP_POSITIVE_Y,
        TextureCubeMapPositiveZ = TEXTURE_CUBE_MAP_POSITIVE_Z,
        TextureRectangle = TEXTURE_RECTANGLE,
    }

    struct ReceivedInvalidTextureTarget(u32);
    TextureTarget {
        Texture1D = TEXTURE_1D,
        Texture2D = TEXTURE_2D,
        Texture3D = TEXTURE_3D,
        Texture1DArray = TEXTURE_1D_ARRAY,
        Texture2DArray = TEXTURE_2D_ARRAY,
        TextureRectangle = TEXTURE_RECTANGLE,
        TextureBuffer = TEXTURE_BUFFER,
        TextureCubeMap = TEXTURE_CUBE_MAP,
        TextureCubeMapArray = TEXTURE_CUBE_MAP_ARRAY,
        Texture2DMultisample = TEXTURE_2D_MULTISAMPLE,
        Texture2DMultisampleArray = TEXTURE_2D_MULTISAMPLE_ARRAY,
    }

    struct ReceivedInvalidBindBufferTarget(u32);
    BindBufferTarget {
        AtomicCounterBuffer = ATOMIC_COUNTER_BUFFER,
        TransformFeedbackBuffer = TRANSFORM_FEEDBACK_BUFFER,
        UniformBuffer = UNIFORM_BUFFER,
        ShaderStorageBuffer = SHADER_STORAGE_BUFFER,
    }

    struct ReceivedInvalidRenderbufferTarget(u32);
    RenderbufferTarget {
        Renderbuffer = RENDERBUFFER,
    }

    struct ReceivedInvalidDepthStencilTextureMode(i32);
    DepthStencilTextureMode {
        DepthComponent = DEPTH_COMPONENT,
        StencilIndex = STENCIL_INDEX,
    }

    // struct ReceivedInvalidTexParameteriParam(i32);
    // TexParameteriParam {
    //     DepthStencilTextureMode = DEPTH_STENCIL_TEXTURE_MODE,
    //     TextureBaseLevel = TEXTURE_BASE_LEVEL,
    //     TextureMaxLevel = TEXTURE_MAX_LEVEL,
    //     TextureMagFilter = TEXTURE_MAG_FILTER,
    //     TextureMinFilter = TEXTURE_MIN_FILTER,
    //     TextureWrapS = TEXTURE_WRAP_S,
    //     TextureWrapT = TEXTURE_WRAP_T,
    //     TextureWrapR = TEXTURE_WRAP_R,
    // }

    // struct ReceivedInvalidTexParameterfParam(i32);
    // TexParameterfParam {
    //     // ARB_texture_filter_anisotropic
    //     TextureMaxAnisotropy = TEXTURE_MAX_ANISOTROPY,
    // }

    // struct ReceivedInvalidSamplerParameteriParam(i32);
    // SamplerParameteri {
    //     TextureMagFilter = TEXTURE_MAG_FILTER,
    //     TextureMinFilter = TEXTURE_MIN_FILTER,
    //     TextureWrapS = TEXTURE_WRAP_S,
    //     TextureWrapT = TEXTURE_WRAP_T,
    //     TextureWrapR = TEXTURE_WRAP_R,
    // }

    struct ReceivedInvalidTextureMagFilter(i32);
    TextureMagFilter {
        Nearest = NEAREST,
        Linear = LINEAR,
    }

    struct ReceivedInvalidTextureMinFilter(i32);
    TextureMinFilter {
        Nearest = NEAREST,
        Linear = LINEAR,
        NearestMipmapNearest = NEAREST_MIPMAP_NEAREST,
        NearestMipmapLinear = NEAREST_MIPMAP_LINEAR,
        LinearMipmapNearest = LINEAR_MIPMAP_NEAREST,
        LinearMipmapLinear = LINEAR_MIPMAP_LINEAR,
    }

    struct ReceivedInvalidTextureWrap(i32);
    TextureWrap {
        ClampToEdge = CLAMP_TO_EDGE,
        Repeat = REPEAT,
        ClampToBorder = CLAMP_TO_BORDER,
        MirroredRepeat = MIRRORED_REPEAT,
        MirrorClampToEdge = MIRROR_CLAMP_TO_EDGE,
    }

    struct ReceivedInvalidFramebufferTarget(u32);
    FramebufferTarget {
        DrawFramebuffer = DRAW_FRAMEBUFFER,
        ReadFramebuffer = READ_FRAMEBUFFER,
        Framebuffer = FRAMEBUFFER,
    }

    struct ReceivedInvalidFramebufferStatus(u32);
    FramebufferStatus {
        FramebufferComplete = FRAMEBUFFER_COMPLETE,
        FramebufferUndefined = FRAMEBUFFER_UNDEFINED,
        FramebufferIncompleteAttachment = FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
        FramebufferIncompleteMissingAttachment = FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
        FramebufferIncompleteDrawBuffer = FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
        FramebufferIncompleteReadBuffer = FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
        FramebufferUnsupported = FRAMEBUFFER_UNSUPPORTED,
        FramebufferIncompleteMultisample = FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
        FramebufferIncompleteLayerTargets = FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
    }

    struct ReceivedInvalidBufferTarget(u32);
    BufferTarget {
        ArrayBuffer = ARRAY_BUFFER,
        AtomicCounterBuffer = ATOMIC_COUNTER_BUFFER,
        CopyReadBuffer = COPY_READ_BUFFER,
        CopyWriteBuffer = COPY_WRITE_BUFFER,
        DispatchIndirectBuffer = DISPATCH_INDIRECT_BUFFER,
        DrawIndirectBuffer = DRAW_INDIRECT_BUFFER,
        ElementArrayBuffer = ELEMENT_ARRAY_BUFFER,
        PixelPackBuffer = PIXEL_PACK_BUFFER,
        PixelUnpackBuffer = PIXEL_UNPACK_BUFFER,
        QueryBuffer = QUERY_BUFFER,
        ShaderStorageBuffer = SHADER_STORAGE_BUFFER,
        TextureBuffer = TEXTURE_BUFFER,
        TransformFeedbackBuffer = TRANSFORM_FEEDBACK_BUFFER,
        UniformBuffer = UNIFORM_BUFFER,
    }

    struct ReceivedInvalidBufferUsage(u32);
    BufferUsage {
        StreamDraw = STREAM_DRAW,
        StreamRead = STREAM_READ,
        StreamCopy = STREAM_COPY,
        StaticDraw = STATIC_DRAW,
        StaticRead = STATIC_READ,
        StaticCopy = STATIC_COPY,
        DynamicDraw = DYNAMIC_DRAW,
        DynamicRead = DYNAMIC_READ,
        DynamicCopy = DYNAMIC_COPY,
    }

    struct ReceivedInvalidVertexAttributeType(u32);
    VertexAttributeType {
        Byte = BYTE,
        UnsignedByte = UNSIGNED_BYTE,
        Short = SHORT,
        UnsignedShort = UNSIGNED_SHORT,
        Int = INT,
        UnsignedInt = UNSIGNED_INT,
        HalfFloat = HALF_FLOAT,
        Float = FLOAT,
        Fixed = FIXED,
        Int2101010Rev = INT_2_10_10_10_REV,
        UnsignedInt2101010Rev = UNSIGNED_INT_2_10_10_10_REV,
        UnsignedInt10f11f11fRev = UNSIGNED_INT_10F_11F_11F_REV,
    }

    struct ReceivedInvalidVertexAttributeIType(u32);
    VertexAttributeIType {
        Byte = BYTE,
        UnsignedByte = UNSIGNED_BYTE,
        Short = SHORT,
        UnsignedShort = UNSIGNED_SHORT,
        Int = INT,
        UnsignedInt = UNSIGNED_INT,
    }

    struct ReceivedInvalidVertexAttributeLType(u32);
    VertexAttributeLType {
        Double = DOUBLE,
    }


    struct ReceivedInvalidDrawMode(u32);
    DrawMode {
        Points = POINTS,
        LineStrip = LINE_STRIP,
        LineLoop = LINE_LOOP,
        Lines = LINES,
        LineStripAdjacency = LINE_STRIP_ADJACENCY,
        LinesAdjacency = LINES_ADJACENCY,
        TriangleStrip = TRIANGLE_STRIP,
        TriangleFan = TRIANGLE_FAN,
        Triangles = TRIANGLES,
        TriangleStripAdjacency = TRIANGLE_STRIP_ADJACENCY,
        TrianglesAdjacency = TRIANGLES_ADJACENCY,
        Patches = PATCHES,
    }

    struct ReceivedInvalidInternalFormat(i32);
    InternalFormat {
        DepthComponent = DEPTH_COMPONENT,
        DepthStencil = DEPTH_STENCIL,
        Red = RED,
        Rg = RG,
        Rgb = RGB,
        Rgba = RGBA,
        R8 = R8,
        R8Snorm = R8_SNORM,
        R16 = R16,
        R16Snorm = R16_SNORM,
        Rg8 = RG8,
        Rg8Snorm = RG8_SNORM,
        Rg16 = RG16,
        Rg16Snorm = RG16_SNORM,
        R3G3B2 = R3_G3_B2,
        Rgb4 = RGB4,
        Rgb5 = RGB5,
        Rgb8 = RGB8,
        Rgb8Snorm = RGB8_SNORM,
        Rgb10 = RGB10,
        Rgb12 = RGB12,
        Rgb16Snorm = RGB16_SNORM,
        Rgba2 = RGBA2,
        Rgba4 = RGBA4,
        Rgb5A1 = RGB5_A1,
        Rgba8 = RGBA8,
        Rgba8Snorm = RGBA8_SNORM,
        Rgb10A2 = RGB10_A2,
        Rgb10A2ui = RGB10_A2UI,
        Rgba12 = RGBA12,
        Rgba16 = RGBA16,
        Srgb8 = SRGB8,
        Srgb8Alpha8 = SRGB8_ALPHA8,
        R16f = R16F,
        Rg16f = RG16F,
        Rgb16f = RGB16F,
        Rgba16f = RGBA16F,
        R32f = R32F,
        Rg32f = RG32F,
        Rgb32f = RGB32F,
        Rgba32f = RGBA32F,
        R11fG11fB10f = R11F_G11F_B10F,
        Rgb9E5 = RGB9_E5,
        R8i = R8I,
        R8ui = R8UI,
        R16i = R16I,
        R16ui = R16UI,
        R32i = R32I,
        R32ui = R32UI,
        Rg8i = RG8I,
        Rg8ui = RG8UI,
        Rg16i = RG16I,
        Rg16ui = RG16UI,
        Rg32i = RG32I,
        Rg32ui = RG32UI,
        Rgb8i = RGB8I,
        Rgb8ui = RGB8UI,
        Rgb16i = RGB16I,
        Rgb16ui = RGB16UI,
        Rgb32i = RGB32I,
        Rgb32ui = RGB32UI,
        Rgba8i = RGBA8I,
        Rgba8ui = RGBA8UI,
        Rgba16i = RGBA16I,
        Rgba16ui = RGBA16UI,
        Rgba32i = RGBA32I,
        Rgba32ui = RGBA32UI,
        CompressedRed = COMPRESSED_RED,
        CompressedRg = COMPRESSED_RG,
        CompressedRgb = COMPRESSED_RGB,
        CompressedRgba = COMPRESSED_RGBA,
        CompressedSrgb = COMPRESSED_SRGB,
        CompressedSrgbAlpha = COMPRESSED_SRGB_ALPHA,
        CompressedRedRgtc1 = COMPRESSED_RED_RGTC1,
        CompressedSignedRedRgtc1 = COMPRESSED_SIGNED_RED_RGTC1,
        CompressedRgRgtc2 = COMPRESSED_RG_RGTC2,
        CompressedSignedRgRgtc2 = COMPRESSED_SIGNED_RG_RGTC2,
        CompressedRgbaBptcUnorm = COMPRESSED_RGBA_BPTC_UNORM,
        CompressedSrgbAlphaBptcUnorm = COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
        CompressedRgbBptcSignedFloat = COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
        CompressedRgbBptcUnsignedFloat = COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
        DepthComponent16 = DEPTH_COMPONENT16,
        DepthComponent24 = DEPTH_COMPONENT24,
        DepthComponent32 = DEPTH_COMPONENT32,
        DepthComponent32f = DEPTH_COMPONENT32F,
        Depth24Stencil8 = DEPTH24_STENCIL8,
        Depth32fStencil8 = DEPTH32F_STENCIL8,
        StencilIndex1 = STENCIL_INDEX1,
        StencilIndex4 = STENCIL_INDEX4,
        StencilIndex8 = STENCIL_INDEX8,
        StencilIndex16 = STENCIL_INDEX16,
        // extension.
        COMPRESSED_RGB_S3TC_DXT1_EXT        = COMPRESSED_RGB_S3TC_DXT1_EXT,
        COMPRESSED_RGBA_S3TC_DXT1_EXT       = COMPRESSED_RGBA_S3TC_DXT1_EXT,
        COMPRESSED_RGBA_S3TC_DXT3_EXT       = COMPRESSED_RGBA_S3TC_DXT3_EXT,
        COMPRESSED_RGBA_S3TC_DXT5_EXT       = COMPRESSED_RGBA_S3TC_DXT5_EXT,
        // extension extension.
        COMPRESSED_SRGB_S3TC_DXT1_EXT       = COMPRESSED_SRGB_S3TC_DXT1_EXT,
        COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT = COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT,
        COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT = COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT,
        COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT = COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT,
    }

    struct ReceivedInvalidFormat(i32);
    Format {
        Bgr = BGR,
        BgrInteger = BGR_INTEGER,
        Bgra = BGRA,
        BgraInteger = BGRA_INTEGER,
        Blue = BLUE,
        BlueInteger = BLUE_INTEGER,
        DepthComponent = DEPTH_COMPONENT,
        DepthStencil = DEPTH_STENCIL,
        Green = GREEN,
        GreenInteger = GREEN_INTEGER,
        Red = RED,
        RedInteger = RED_INTEGER,
        Rg = RG,
        RgInteger = RG_INTEGER,
        Rgb = RGB,
        RgbInteger = RGB_INTEGER,
        Rgba = RGBA,
        RgbaInteger = RGBA_INTEGER,
        StencilIndex = STENCIL_INDEX,
    }

    struct ReceivedInvalidComponentFormat(i32);
    ComponentFormat {
        Byte = BYTE,
        Float = FLOAT,
        Float32UnsignedInt248Rev = FLOAT_32_UNSIGNED_INT_24_8_REV,
        HalfFloat = HALF_FLOAT,
        Int = INT,
        Short = SHORT,
        UnsignedByte = UNSIGNED_BYTE,
        UnsignedByte233Rev = UNSIGNED_BYTE_2_3_3_REV,
        UnsignedByte332 = UNSIGNED_BYTE_3_3_2,
        UnsignedInt = UNSIGNED_INT,
        UnsignedInt1010102 = UNSIGNED_INT_10_10_10_2,
        UnsignedInt10f11f11fRev = UNSIGNED_INT_10F_11F_11F_REV,
        UnsignedInt2101010Rev = UNSIGNED_INT_2_10_10_10_REV,
        UnsignedInt248 = UNSIGNED_INT_24_8,
        UnsignedInt5999Rev = UNSIGNED_INT_5_9_9_9_REV,
        UnsignedInt8888 = UNSIGNED_INT_8_8_8_8,
        UnsignedInt8888Rev = UNSIGNED_INT_8_8_8_8_REV,
        UnsignedShort = UNSIGNED_SHORT,
        UnsignedShort1555Rev = UNSIGNED_SHORT_1_5_5_5_REV,
        UnsignedShort4444 = UNSIGNED_SHORT_4_4_4_4,
        UnsignedShort4444Rev = UNSIGNED_SHORT_4_4_4_4_REV,
        UnsignedShort5551 = UNSIGNED_SHORT_5_5_5_1,
        UnsignedShort565 = UNSIGNED_SHORT_5_6_5,
        UnsignedShort565Rev = UNSIGNED_SHORT_5_6_5_REV,
    }

    struct ReceivedInvalidDrawElementsType(u32);
    DrawElementsType {
        UnsignedByte = UNSIGNED_BYTE,
        UnsignedShort = UNSIGNED_SHORT,
        UnsignedInt = UNSIGNED_INT,
    }

    struct ReceivedInvalidCapability(u32);
    Capability {
        Blend = BLEND,
        ColorLogicOp = COLOR_LOGIC_OP,
        CullFace = CULL_FACE,
        DebugOutput = DEBUG_OUTPUT,
        DebugOutputSynchronous = DEBUG_OUTPUT_SYNCHRONOUS,
        DepthClamp = DEPTH_CLAMP,
        DepthTest = DEPTH_TEST,
        Dither = DITHER,
        FramebufferSrgb = FRAMEBUFFER_SRGB,
        LineSmooth = LINE_SMOOTH,
        Multisample = MULTISAMPLE,
        PolygonOffsetFill = POLYGON_OFFSET_FILL,
        PolygonOffsetLine = POLYGON_OFFSET_LINE,
        PolygonOffsetPoint = POLYGON_OFFSET_POINT,
        PolygonSmooth = POLYGON_SMOOTH,
        PrimitiveRestart = PRIMITIVE_RESTART,
        PrimitiveRestartFixedIndex = PRIMITIVE_RESTART_FIXED_INDEX,
        RasterizerDiscard = RASTERIZER_DISCARD,
        SampleAlphaToCoverage = SAMPLE_ALPHA_TO_COVERAGE,
        SampleAlphaToOne = SAMPLE_ALPHA_TO_ONE,
        SampleCoverage = SAMPLE_COVERAGE,
        SampleShading = SAMPLE_SHADING,
        SampleMask = SAMPLE_MASK,
        ScissorTest = SCISSOR_TEST,
        StencilTest = STENCIL_TEST,
        TextureCubeMapSeamless = TEXTURE_CUBE_MAP_SEAMLESS,
        ProgramPointSize = PROGRAM_POINT_SIZE,
    }

    struct ReceivedInvalidPolygonModeFace(u32);
    PolygonModeFace {
        FrontAndBack = FRONT_AND_BACK,
    }

    struct ReceivedInvalidPolygonMode(u32);
    PolygonMode {
        Point = POINT,
        Line = LINE,
        Fill = FILL,
    }

    struct ReceivedInvalidCullFace(u32);
    CullFace {
        Front = FRONT,
        Back = BACK,
        FrontAndBack = FRONT_AND_BACK,
    }

    struct ReceivedInvalidMajorAxis(u32);
    MajorAxis {
        Column = FALSE,
        Row = TRUE,
    }

    struct ReceivedInvalidWriteMask(u32);
    WriteMask {
        Disabled = FALSE,
        Enabled = TRUE,
    }

    struct ReceivedInvalidDepthFunc(u32);
    DepthFunc {
        Never = NEVER,
        LT = LESS,
        EQ = EQUAL,
        LE = LEQUAL,
        GT = GREATER,
        NE = NOTEQUAL,
        GE = GEQUAL,
        Always = ALWAYS,
    }

    struct ReceivedInvalidClipControlOrigin(u32);
    ClipControlOrigin {
        LowerLeft = LOWER_LEFT,
        UpperLeft = UPPER_LEFT,
    }

    struct ReceivedInvalidClipControlDepth(u32);
    ClipControlDepth {
        N1P1 = NEGATIVE_ONE_TO_ONE,
        Z0P1 = ZERO_TO_ONE,
    }

    struct ReceivedInvalidQueryTarget(u32);
    QueryTarget {
        SamplesPassed = SAMPLES_PASSED,
        AnySamplesPassed = ANY_SAMPLES_PASSED,
        AnySamplesPassedConservative = ANY_SAMPLES_PASSED_CONSERVATIVE,
        TimeElapsed = TIME_ELAPSED,
        Timestamp = TIMESTAMP,
        PrimitivesGenerated = PRIMITIVES_GENERATED,
        TransformFeedbackPrimitivesWritten = TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
    }

    struct ReceivedInvalidScopeQueryTarget(u32);
    ScopeQueryTarget {
        SamplesPassed = SAMPLES_PASSED,
        AnySamplesPassed = ANY_SAMPLES_PASSED,
        AnySamplesPassedConservative = ANY_SAMPLES_PASSED_CONSERVATIVE,
        PrimitivesGenerated = PRIMITIVES_GENERATED,
        TransformFeedbackPrimitivesWritten = TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
        TimeElapsed = TIME_ELAPSED,
    }


    struct ReceivedInvalidBlitFilter(u32);
    BlitFilter {
        Nearest = NEAREST,
        Linear = LINEAR,
    }

    struct ReceivedInvalidBlendFactor(u32);
    BlendFactor {
        Zero = ZERO,
        One = ONE,
        SrcColor = SRC_COLOR,
        OneMinusSrcColor = ONE_MINUS_SRC_COLOR,
        DstColor = DST_COLOR,
        OneMinusDstColor = ONE_MINUS_DST_COLOR,
        SrcAlpha = SRC_ALPHA,
        OneMinusSrcAlpha = ONE_MINUS_SRC_ALPHA,
        DstAlpha = DST_ALPHA,
        OneMinusDstAlpha = ONE_MINUS_DST_ALPHA,
        ConstantColor = CONSTANT_COLOR,
        OneMinusConstantColor = ONE_MINUS_CONSTANT_COLOR,
        ConstantAlpha = CONSTANT_ALPHA,
        OneMinusConstantAlpha = ONE_MINUS_CONSTANT_ALPHA,
        SrcAlphaSaturate = SRC_ALPHA_SATURATE,
        Src1Color = SRC1_COLOR,
        OneMinusSrc1Color = ONE_MINUS_SRC1_COLOR,
        Src1Alpha = SRC1_ALPHA,
        OneMinusSrc1Alpha = ONE_MINUS_SRC1_ALPHA,
    }
}

macro_rules! impl_struct_from_symbol {
    ($Struct:ident { $($Variant:ident = $Symbol:ident,)* }) => {
        $(
            impl From<$Symbol> for $Struct {
                #[inline]
                fn from(_: $Symbol) -> Self {
                    $Struct($Symbol::VALUE)
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MaxCombinedTextureImageUnits(u32);

impl MaxCombinedTextureImageUnits {
    #[inline]
    pub(crate) fn to_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(index: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if index < max.to_u32() {
            Some(TextureUnit(<TEXTURE0 as Symbol<u32>>::VALUE + index))
        } else {
            None
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        TextureUnit(<TEXTURE0 as Symbol<u32>>::VALUE + index)
    }

    #[deprecated]
    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub(crate) fn to_u32(self) -> u32 {
        self.0
    }
}

impl_struct_from_symbol! (TextureUnit {
    Texture0 = TEXTURE0,
    Texture1 = TEXTURE1,
    Texture2 = TEXTURE2,
    Texture3 = TEXTURE3,
    Texture4 = TEXTURE4,
    Texture5 = TEXTURE5,
    Texture6 = TEXTURE6,
    Texture7 = TEXTURE7,
    Texture8 = TEXTURE8,
    Texture9 = TEXTURE9,
    Texture10 = TEXTURE10,
    Texture11 = TEXTURE11,
    Texture12 = TEXTURE12,
    Texture13 = TEXTURE13,
    Texture14 = TEXTURE14,
    Texture15 = TEXTURE15,
});

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MaxColorAttachments(u32);

impl MaxColorAttachments {
    #[deprecated]
    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub(crate) fn to_u32(&self) -> u32 {
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
            if index < max.to_u32() {
                Some(FramebufferAttachment::new_unchecked(index))
            } else {
                None
            }
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        FramebufferAttachment(<COLOR_ATTACHMENT0 as Symbol<u32>>::VALUE + index)
    }

    #[deprecated]
    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub(crate) fn to_u32(&self) -> u32 {
        self.0
    }
}

impl_struct_from_symbol! (
    FramebufferAttachment {
        DepthStencilAttachment = DEPTH_STENCIL_ATTACHMENT,
        DepthAttachment = DEPTH_ATTACHMENT,
        StencilAttachment = STENCIL_ATTACHMENT,
        ColorAttachment0 = COLOR_ATTACHMENT0,
        ColorAttachment1 = COLOR_ATTACHMENT1,
        ColorAttachment2 = COLOR_ATTACHMENT2,
        ColorAttachment3 = COLOR_ATTACHMENT3,
        ColorAttachment4 = COLOR_ATTACHMENT4,
        ColorAttachment5 = COLOR_ATTACHMENT5,
        ColorAttachment6 = COLOR_ATTACHMENT6,
        ColorAttachment7 = COLOR_ATTACHMENT7,
        ColorAttachment8 = COLOR_ATTACHMENT8,
        ColorAttachment9 = COLOR_ATTACHMENT9,
        ColorAttachment10 = COLOR_ATTACHMENT10,
        ColorAttachment11 = COLOR_ATTACHMENT11,
        ColorAttachment12 = COLOR_ATTACHMENT12,
        ColorAttachment13 = COLOR_ATTACHMENT13,
        ColorAttachment14 = COLOR_ATTACHMENT14,
        ColorAttachment15 = COLOR_ATTACHMENT15,
    }
);

bitflags::bitflags! {
    pub struct ClearFlag : u32 {
        const COLOR_BUFFER = crate::gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER = crate::gl::DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER = crate::gl::STENCIL_BUFFER_BIT;
    }
}

bitflags::bitflags! {
    pub struct ContextFlag : u32 {
        const FORWARD_COMPATIBLE = crate::gl::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT;
        const DEBUG = crate::gl::CONTEXT_FLAG_DEBUG_BIT;
        const ROBUST_ACCESS = crate::gl::CONTEXT_FLAG_ROBUST_ACCESS_BIT;
    }
}

bitflags::bitflags! {
    pub struct BlitMask : u32 {
        const COLOR_BUFFER_BIT = crate::gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER_BIT = crate::gl::DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER_BIT = crate::gl::STENCIL_BUFFER_BIT;
    }
}

bitflags::bitflags! {
    pub struct BufferStorageFlag : u32 {
        const DYNAMIC_STORAGE = crate::gl::DYNAMIC_STORAGE_BIT;
        const READ = crate::gl::MAP_READ_BIT;
        const WRITE = crate::gl::MAP_WRITE_BIT;
        const PERSISTENT = crate::gl::MAP_PERSISTENT_BIT;
        const COHERENT = crate::gl::MAP_COHERENT_BIT;
        const CLIENT_STORAGE = crate::gl::CLIENT_STORAGE_BIT;
    }
}

bitflags::bitflags! {
    pub struct MapAccessFlag : u32 {
        const READ_ONLY = crate::gl::READ_ONLY;
        const WRITE_ONLY = crate::gl::WRITE_ONLY;
        const READ_WRITE = crate::gl::READ_WRITE;
    }
}

bitflags::bitflags! {
    pub struct MapRangeAccessFlag : u32 {
        const READ = crate::gl::MAP_READ_BIT;
        const WRITE = crate::gl::MAP_WRITE_BIT;
        const PERSISTENT = crate::gl::MAP_PERSISTENT_BIT;
        const COHERENT = crate::gl::MAP_COHERENT_BIT;
        const INVALIDATE_RANGE = crate::gl::MAP_INVALIDATE_RANGE_BIT;
        const INVALIDATE_BUFFER = crate::gl::MAP_INVALIDATE_BUFFER_BIT;
        const FLUSH_EXPLICIT = crate::gl::MAP_FLUSH_EXPLICIT_BIT;
        const UNSYNCHRONIZED = crate::gl::MAP_UNSYNCHRONIZED_BIT;
    }
}

bitflags::bitflags! {
    pub struct MemoryBarrierFlag : u32 {
        const VERTEX_ATTRIB_ARRAY = crate::gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT;
        const ELEMENT_ARRAY = crate::gl::ELEMENT_ARRAY_BARRIER_BIT;
        const UNIFORM = crate::gl::UNIFORM_BARRIER_BIT;
        const TEXTURE_FETCH = crate::gl::TEXTURE_FETCH_BARRIER_BIT;
        const SHADER_IMAGE_ACCESS = crate::gl::SHADER_IMAGE_ACCESS_BARRIER_BIT;
        const COMMAND = crate::gl::COMMAND_BARRIER_BIT;
        const PIXEL_BUFFER = crate::gl::PIXEL_BUFFER_BARRIER_BIT;
        const TEXTURE_UPDATE = crate::gl::TEXTURE_UPDATE_BARRIER_BIT;
        const BUFFER_UPDATE = crate::gl::BUFFER_UPDATE_BARRIER_BIT;
        const FRAMEBUFFER = crate::gl::FRAMEBUFFER_BARRIER_BIT;
        const TRANSFORM_FEEDBACK = crate::gl::TRANSFORM_FEEDBACK_BARRIER_BIT;
        const ATOMIC_COUNTER = crate::gl::ATOMIC_COUNTER_BARRIER_BIT;
        const SHADER_STORAGE = crate::gl::SHADER_STORAGE_BARRIER_BIT;
        const QUERY_BUFFER = crate::gl::QUERY_BUFFER_BARRIER_BIT;
        const ALL = crate::gl::ALL_BARRIER_BITS;
    }
}
