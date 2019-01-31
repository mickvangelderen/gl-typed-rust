use crate::gl;

pub trait Shader {
    type Kind: Into<ShaderKind> + Copy;

    fn from_parts(kind: Self::Kind, name: ShaderName) -> Self;

    fn into_parts(self) -> (Self::Kind, ShaderName);

    fn kind(&self) -> Self::Kind;

    fn name(&self) -> &ShaderName;
}

/// Kind of a shader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderKind {
    Compute = gl::COMPUTE_SHADER,
    Vertex = gl::VERTEX_SHADER,
    TessControl = gl::TESS_CONTROL_SHADER,
    TessEvaluation = gl::TESS_EVALUATION_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

/// Name of a shader, without the kind.
impl_name!(ShaderName);

/// Shader kind and name.
#[derive(Debug)]
pub struct ShaderKindName {
    kind: ShaderKind,
    name: ShaderName,
}

impl Shader for ShaderKindName {
    type Kind = ShaderKind;

    fn from_parts(kind: Self::Kind, name: ShaderName) -> Self {
        ShaderKindName { kind, name }
    }

    fn into_parts(self) -> (Self::Kind, ShaderName) {
        let ShaderKindName { kind, name } = self;
        (kind, name)
    }

    fn kind(&self) -> Self::Kind {
        self.kind
    }

    fn name(&self) -> &ShaderName {
        &self.name
    }
}

macro_rules! impl_shader_kinds_and_names {
    ($(($Kind: ident, $Name: ident, $const: ident, $value: expr $(,)?)),* $(,)?) => {
        $(
            /// Shader kind of which we know the variant at compile-time.
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            #[repr(transparent)]
            pub struct $Kind(ShaderKind);

            /// Convert from compile-time variant into run-time variant.
            impl From<$Kind> for ShaderKind {
                fn from(kind: $Kind) -> Self {
                    kind.0
                }
            }

            pub const $const: $Kind = $Kind($value);

            /// Shader name of which we know the kind at compile-time.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct $Name(ShaderName);

            // NOTE(mickvangelderen): I do not want to encourage losing the
            // kind. Maybe there is a use case for it though.
            // /// Permanently lose the shader kind.
            // impl From<$Name> for ShaderName {
            //     fn from(name: $Name) -> Self {
            //         name.0
            //     }
            // }

            /// Temporarily lose the shader kind.
            impl AsRef<ShaderName> for $Name {
                fn as_ref(&self) -> &ShaderName {
                    &self.0
                }
            }

            impl Shader for $Name {
                type Kind = $Kind;

                fn from_parts(_: Self::Kind, name: ShaderName) -> Self {
                    $Name(name)
                }

                fn into_parts(self) -> (Self::Kind, ShaderName) {
                    ($const, self.0)
                }

                fn kind(&self) -> Self::Kind {
                    $const
                }

                fn name(&self) -> &ShaderName {
                    self.as_ref()
                }
            }
        )*
    }
}

impl_shader_kinds_and_names!(
    (
        ComputeShaderKind,
        ComputeShaderName,
        COMPUTE_SHADER,
        ShaderKind::Compute,
    ),
    (
        VertexShaderKind,
        VertexShaderName,
        VERTEX_SHADER,
        ShaderKind::Vertex
    ),
    (
        TessControlShaderKind,
        TessControlShaderName,
        TESS_CONTROL_SHADER,
        ShaderKind::TessControl
    ),
    (
        TessEvaluationShaderKind,
        TessEvaluationShaderName,
        TESS_EVALUATION_SHADER,
        ShaderKind::TessEvaluation
    ),
    (
        GeometryShaderKind,
        GeometryShaderName,
        GEOMETRY_SHADER,
        ShaderKind::Geometry
    ),
    (
        FragmentShaderKind,
        FragmentShaderName,
        FRAGMENT_SHADER,
        ShaderKind::Fragment
    )
);
