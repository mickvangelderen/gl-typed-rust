//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;

macro_rules! impl_enums_u32 {
    ($($(#[$em:meta])* pub enum $e:ident { $($v:ident = $g:path,)* })*) => {
        $(
            $(#[$em])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr(u32)]
            pub enum $e {
                $(
                    $v = $g,
                )*
            }

            impl $e {
                #[inline]
                pub unsafe fn from_u32_unchecked(v: u32) -> Self {
                    std::mem::transmute(v)
                }
            }

            impl From<u32> for $e {
                #[inline]
                fn from(v: u32) -> Self {
                    match v {
                        $(
                            $g => $e::$v,
                        )*
                        v => panic!("$e has no variant corresponding to {}.", v),
                    }
                }
            }

            $(
                impl From<crate::symbols::$v> for $e {
                    #[inline]
                    fn from(_: crate::symbols::$v) -> Self {
                        $e::$v
                    }
                }
            )*

            impl From<$e> for crate::symbols::Unknown {
                #[inline]
                fn from(_: $e) -> Self {
                    crate::symbols::Unknown
                }
            }
        )*
    }
}

const TRUE: u32 = gl::TRUE as u32;
const FALSE: u32 = gl::FALSE as u32;

impl_enums_u32! {
    /// The kind of a shader.
    pub enum ShaderKind {
        Compute = gl::COMPUTE_SHADER,
        Vertex = gl::VERTEX_SHADER,
        TessControl = gl::TESS_CONTROL_SHADER,
        TessEvaluation = gl::TESS_EVALUATION_SHADER,
        Geometry = gl::GEOMETRY_SHADER,
        Fragment = gl::FRAGMENT_SHADER,
    }

    /// The compile status of a shader.
    pub enum CompileStatus {
        Uncompiled = FALSE,
        Compiled = TRUE,
    }

    /// Allowed pname arguments to `glGetShaderiv`.
    pub enum GetShaderivParam {
        ShaderType = gl::SHADER_TYPE,
        DeleteStatus = gl::DELETE_STATUS,
        CompileStatus = gl::COMPILE_STATUS,
        InfoLogLength = gl::INFO_LOG_LENGTH,
        ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
    }
}
