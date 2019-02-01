//! This module defines a set of zero sized types that can be used as markers or
//! whatever. The symbols can be re-used to represent different variants in
//! multiple enums, which is why are defined in their own module.

macro_rules! impl_symbols {
    ($($Symbol: ident),*) => {
        $(
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub struct $Symbol;
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
    Unknown,
    Vertex
);
