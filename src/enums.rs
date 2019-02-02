//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;
use crate::symbols;

macro_rules! impl_variants {
    ($e: ident { $($v: ident,)* }) => {
        $(
            impl_variants!(
                enum = $e,
                variant = $e::$v,
                symbol = symbols::$v,
            );
        )*
    };
    (enum = $enum: path, variant = $variant: path, symbol = $symbol: path,) => {
        impl From<$symbol> for $enum {
            fn from(_: $symbol) -> Self {
                $variant
            }
        }
    };
}

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

impl_variants!(
    ShaderKind {
        Compute,
        Vertex,
        TessControl,
        TessEvaluation,
        Geometry,
        Fragment,
    }
);

/// The compile status of a shader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CompileStatus {
    Uncompiled = gl::FALSE as u32,
    Compiled = gl::TRUE as u32,
}

impl_variants!(
    CompileStatus {
        Uncompiled,
        Compiled,
    }
);

/// Allowed pname arguments to `glGetShaderiv`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GetShaderivParam {
    ShaderType = gl::SHADER_TYPE,
    DeleteStatus = gl::DELETE_STATUS,
    CompileStatus = gl::COMPILE_STATUS,
    InfoLogLength = gl::INFO_LOG_LENGTH,
    ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
}

impl_variants!(
    GetShaderivParam {
        ShaderType,
        DeleteStatus,
        CompileStatus,
        InfoLogLength,
        ShaderSourceLength,
    }
);
