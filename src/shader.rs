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
    ($(($Kind: ident, $Shader: ident, $const: ident, $value: expr $(,)?)),* $(,)?) => {
        $(
            /// Shader kind known at compile-time.
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub struct $Kind(());

            /// Convert from compile-time variant into run-time variant.
            impl From<$Kind> for ShaderKind {
                fn from(_: $Kind) -> Self {
                    $value
                }
            }

            pub const $const: $Kind = $Kind(());

            /// Shader name of which we know the kind at compile-time.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct $Shader {
                kind: $Kind,
                name: ShaderName,
            }

            /// Permanently lose the shader kind.
            impl From<$Shader> for ShaderName {
                fn from(shader: $Shader) -> Self {
                    let (_, name) = shader.into_parts();
                    name
                }
            }

            /// Temporarily lose the shader kind.
            impl AsRef<ShaderName> for $Shader {
                fn as_ref(&self) -> &ShaderName {
                    &self.name
                }
            }

            impl Shader for $Shader {
                type Kind = $Kind;

                fn from_parts(kind: Self::Kind, name: ShaderName) -> Self {
                    $Shader { kind, name }
                }

                fn into_parts(self) -> (Self::Kind, ShaderName) {
                    let $Shader { kind, name } = self;
                    (kind, name)
                }

                fn kind(&self) -> Self::Kind {
                    self.kind
                }

                fn name(&self) -> &ShaderName {
                    &self.name
                }
            }
        )*
    }
}

impl_shader_kinds_and_names!(
    (
        ComputeShaderKind,
        ComputeShader,
        COMPUTE_SHADER,
        ShaderKind::Compute,
    ),
    (
        VertexShaderKind,
        VertexShader,
        VERTEX_SHADER,
        ShaderKind::Vertex
    ),
    (
        TessControlShaderKind,
        TessControlShader,
        TESS_CONTROL_SHADER,
        ShaderKind::TessControl
    ),
    (
        TessEvaluationShaderKind,
        TessEvaluationShader,
        TESS_EVALUATION_SHADER,
        ShaderKind::TessEvaluation
    ),
    (
        GeometryShaderKind,
        GeometryShader,
        GEOMETRY_SHADER,
        ShaderKind::Geometry
    ),
    (
        FragmentShaderKind,
        FragmentShader,
        FRAGMENT_SHADER,
        ShaderKind::Fragment
    )
);
