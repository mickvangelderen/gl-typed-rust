//! This module defines enums based on the OpenGL constants. Some of these enums
//! have overlapping values. For most enums compile-time variants are provided
//! through the symbols.

use crate::gl;
use crate::traits::UncheckedFrom;

macro_rules! impl_from_transmute {
    ($f:path, $t:path) => {
        impl From<$f> for $t {
            fn from(value: $f) -> Self {
                unsafe { std::mem::transmute(value) }
            }
        }
    };
}

macro_rules! impl_transparent_fns {
    ($outer:ident, $inner:ident) => {
        impl AsRef<$inner> for $outer {
            fn as_ref(&self) -> &$inner {
                unsafe { &*(self as *const Self as *const $inner) }
            }
        }

        impl AsMut<$inner> for $outer {
            fn as_mut(&mut self) -> &mut $inner {
                unsafe { &mut *(self as *mut Self as *mut $inner) }
            }
        }

        impl_from_transmute!($inner, $outer);
        impl_from_transmute!($outer, $inner);
    }
}

macro_rules! impl_enums_u32 {
    ($($(#[$rm:meta])* $r:ident $(#[$em:meta])* $e:ident { $($v:ident = $g:path,)* })*) => {
        $(
            $(#[$rm])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $r(u32);

            impl_transparent_fns!($r, u32);
            impl_transparent_fns!($r, i32);

            $(#[$em])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr(u32)]
            pub enum $e {
                $(
                    $v = $g,
                )*
            }

            impl From<$e> for $r {
                #[inline]
                fn from(val: $e) -> Self {
                    $r(val as u32)
                }
            }

            impl From<$r> for $e {
                /// # Panics
                /// Panics when the passed value does not correspond to any of
                /// the known variants.
                #[inline]
                fn from(raw: $r) -> Self {
                    match raw.0 {
                        $(
                            $g => $e::$v,
                        )*
                        v => panic!("No known variant corresponds to {}.", v),
                    }
                }
            }

            impl UncheckedFrom<$r> for $e {
                #[inline]
                unsafe fn unchecked_from(raw: $r) -> Self {
                    std::mem::transmute(raw)
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
        )*
    }
}

const TRUE: u32 = gl::TRUE as u32;
const FALSE: u32 = gl::FALSE as u32;

impl_enums_u32! {
    RawShaderKind
    /// The kind of a shader.
    ShaderKind {
        Compute = gl::COMPUTE_SHADER,
        Vertex = gl::VERTEX_SHADER,
        TessControl = gl::TESS_CONTROL_SHADER,
        TessEvaluation = gl::TESS_EVALUATION_SHADER,
        Geometry = gl::GEOMETRY_SHADER,
        Fragment = gl::FRAGMENT_SHADER,
    }

    RawShaderCompileStatus
    /// The compile status of a shader.
    ShaderCompileStatus {
        Uncompiled = FALSE,
        Compiled = TRUE,
    }

    RawGetShaderivParam
    /// Allowed pname arguments to `glGetShaderiv`.
    GetShaderivParam {
        ShaderType = gl::SHADER_TYPE,
        DeleteStatus = gl::DELETE_STATUS,
        CompileStatus = gl::COMPILE_STATUS,
        InfoLogLength = gl::INFO_LOG_LENGTH,
        ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
    }
}
