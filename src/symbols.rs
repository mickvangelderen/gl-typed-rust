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
    CompileStatus,
    Compiled,
    Compute,
    ComputeWorkGroupSize,
    DeleteStatus,
    False,
    Fragment,
    Geometry,
    GeometryInputType,
    GeometryOutputType,
    GeometryShaderInvocations,
    GeometryVerticesOut,
    InfoLogLength,
    LinkStatus,
    Linked,
    ProgramBinaryRetrievableHint,
    ProgramSeparable,
    ShaderSourceLength,
    ShaderType,
    TessControl,
    TessControlOutputVertices,
    TessEvaluation,
    TessGenMode,
    TessGenPointMode,
    TessGenVertexOrder,
    TransformFeedbackBufferMode,
    TransformFeedbackVaryingMaxLength,
    TransformFeedbackVaryings,
    True,
    Uncompiled,
    Unlinked,
    ValidateStatus,
    Vertex,
);
