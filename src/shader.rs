use crate::gl;

/// The kind of a shader.
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

/// The name of a shader, without the kind.
impl_name!(ShaderName);

pub mod generic {
    use super::*;

    /// A generic shader object for which we may know the kind at compile-time.
    #[derive(Debug)]
    pub struct Shader<K> {
        kind: K,
        name: ShaderName,
    }

    impl<K> Shader<K> {
        /// Does not verify whether name is actually of the given kind.
        pub const unsafe fn from_raw_parts(kind: K, name: ShaderName) -> Self {
            Shader { kind, name }
        }

        pub fn into_raw_parts(self) -> (K, ShaderName) {
            let Shader { kind, name } = self;
            (kind, name)
        }

        pub const fn kind(&self) -> &K {
            &self.kind
        }

        pub const fn name(&self) -> &ShaderName {
            &self.name
        }
    }

    impl<K> From<Shader<K>> for ShaderName {
        fn from(shader: Shader<K>) -> Self {
            let (_kind, name) = shader.into_raw_parts();
            name
        }
    }
}

/// A shader for which we know the kind at run-time.
pub type Shader = generic::Shader<ShaderKind>;

macro_rules! impl_shaders {
    ($(($Kind: ident, $Shader: ident, $const: ident, $value: expr $(,)?)),* $(,)?) => {
        $(
            impl_shaders!(IMPL $Kind, $Shader, $const, $value,
                concat!("The compile-time version of [", stringify!($value), "]."),
                concat!("Singleton for [", stringify!($Kind), "]."),
                concat!("Shader for which we know the kind is [", stringify!($value), "] at compile-time.")
            );
        )*
    };

    (IMPL $Kind: ident, $Shader: ident, $const: ident, $value: expr, $doc_kind: expr, $doc_const: expr, $doc_shader: expr) => {
        #[doc = $doc_kind]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $Kind(());

        /// Convert from compile-time variant into run-time variant.
        impl From<$Kind> for ShaderKind {
            fn from(_: $Kind) -> Self {
                $value
            }
        }

        #[doc = $doc_const]
        pub const $const: $Kind = $Kind(());

        #[doc = $doc_shader]
        pub type $Shader = generic::Shader<$Kind>;
    };
}

impl_shaders!(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn sizes_are_optimized() {
        assert_eq!(size_of::<Option<Shader>>(), 8);
        assert_eq!(size_of::<Option<ComputeShader>>(), 4);
    }
}
