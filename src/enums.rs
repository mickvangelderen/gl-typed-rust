//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;
use crate::symbols;
use crate::traits;

macro_rules! impl_variants {
    ($Enum: ty, $Trait: ty, $(($Symbol: ty, $variant: expr $(,)?)),* $(,)?) => {
        $(
            impl_variants!(IMPL $Enum, $Trait, $Symbol, $variant);
        )*
    };
    (IMPL $Enum: ty, $Trait: ty, $Symbol: ty, $variant: expr) => {
        /// Convert from compile-time variant into run-time variant.
        impl From<$Symbol> for $Enum {
            fn from(_: $Symbol) -> Self {
                $variant
            }
        }

        impl $Trait for $Symbol {}
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

impl traits::ShaderKind for ShaderKind {}

impl_variants!(
    ShaderKind,
    traits::ShaderKind,
    (symbols::Compute, ShaderKind::Compute),
    (symbols::Vertex, ShaderKind::Vertex),
    (symbols::TessControl, ShaderKind::TessControl),
    (symbols::TessEvaluation, ShaderKind::TessEvaluation),
    (symbols::Geometry, ShaderKind::Geometry),
    (symbols::Fragment, ShaderKind::Fragment),
);

/// The compile status of a shader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CompileStatus {
    Uncompiled = gl::FALSE as u32,
    Compiled = gl::TRUE as u32,
}

impl traits::CompileStatus for CompileStatus {}

impl_variants!(
    CompileStatus,
    traits::CompileStatus,
    (symbols::Uncompiled, CompileStatus::Uncompiled),
    (symbols::Compiled, CompileStatus::Compiled),
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

impl traits::GetShaderivParam for GetShaderivParam {}

impl_variants!(
    GetShaderivParam,
    traits::GetShaderivParam,
    (symbols::ShaderType, GetShaderivParam::ShaderType),
    (symbols::DeleteStatus, GetShaderivParam::DeleteStatus),
    (symbols::CompileStatus, GetShaderivParam::CompileStatus),
    (symbols::InfoLogLength, GetShaderivParam::InfoLogLength),
    (
        symbols::ShaderSourceLength,
        GetShaderivParam::ShaderSourceLength
    ),
);
