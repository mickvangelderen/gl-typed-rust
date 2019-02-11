//! This module defines a set of zero sized types that can be used as markers or
//! whatever. The symbols can be re-used to represent different variants in
//! multiple enums, which is why are defined in their own module.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Unknown;

macro_rules! impl_unknown_from {
    ($T: path) => {
        impl From<$T> for Unknown {
            fn from(_: $T) -> Self {
                Unknown
            }
        }
    };
}

macro_rules! impl_symbols {
    ($($Symbol: ident,)*) => {
        $(
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub struct $Symbol;

            impl_unknown_from!($Symbol);
        )*
    };
}

impl_symbols!(
    ActiveAtomicCounterBuffers,
    ActiveAttributeMaxLength,
    ActiveAttributes,
    ActiveUniformBlockMaxNameLength,
    ActiveUniformBlocks,
    ActiveUniformMaxLength,
    ActiveUniforms,
    AttachedShaders,
    ClampToBorder,
    ClampToEdge,
    CompileStatus,
    Compiled,
    Compute,
    ComputeWorkGroupSize,
    DeleteStatus,
    DepthComponent,
    DepthStencilTextureMode,
    False,
    Fragment,
    Geometry,
    GeometryInputType,
    GeometryOutputType,
    GeometryShaderInvocations,
    GeometryVerticesOut,
    InfoLogLength,
    Linear,
    LinearMipmapLinear,
    LinearMipmapNearest,
    LinkStatus,
    Linked,
    MirrorClampToEdge,
    MirroredRepeat,
    Nearest,
    NearestMipmapLinear,
    NearestMipmapNearest,
    ProgramBinaryRetrievableHint,
    ProgramSeparable,
    ProxyTexture1DArray,
    ProxyTexture2D,
    ProxyTextureCubeMap,
    ProxyTextureRectangle,
    Renderer,
    Repeat,
    ShaderSourceLength,
    ShaderType,
    ShadingLanguageVersion,
    StencilIndex,
    TessControl,
    TessControlOutputVertices,
    TessEvaluation,
    TessGenMode,
    TessGenPointMode,
    TessGenVertexOrder,
    Texture0,
    Texture1,
    Texture10,
    Texture11,
    Texture12,
    Texture13,
    Texture14,
    Texture15,
    Texture1D,
    Texture1DArray,
    Texture2,
    Texture2D,
    Texture2DArray,
    Texture2DMultisample,
    Texture2DMultisampleArray,
    Texture3,
    Texture3D,
    Texture4,
    Texture5,
    Texture6,
    Texture7,
    Texture8,
    Texture9,
    TextureBaseLevel,
    TextureBuffer,
    TextureCubeMap,
    TextureCubeMapArray,
    TextureCubeMapNegativeX,
    TextureCubeMapNegativeY,
    TextureCubeMapNegativeZ,
    TextureCubeMapPositiveX,
    TextureCubeMapPositiveY,
    TextureCubeMapPositiveZ,
    TextureMagFilter,
    TextureMinFilter,
    TextureRectangle,
    TextureWrapR,
    TextureWrapS,
    TextureWrapT,
    TransformFeedbackBufferMode,
    TransformFeedbackVaryingMaxLength,
    TransformFeedbackVaryings,
    True,
    Uncompiled,
    Unlinked,
    ValidateStatus,
    Vendor,
    Version,
    Vertex,
);
