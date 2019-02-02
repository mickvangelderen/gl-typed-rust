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
    CompileStatus,
    Compiled,
    Compute,
    DeleteStatus,
    Fragment,
    Geometry,
    InfoLogLength,
    ShaderSourceLength,
    ShaderType,
    TessControl,
    TessEvaluation,
    Uncompiled,
    Vertex,
);
