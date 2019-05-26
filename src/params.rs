use crate::*;

macro_rules! impl_param_read_single {
    (
        mod $mod:ident {
            $( $Symbol:ident($Raw:ty => $Value:ty), )*
        }
    ) => {
        pub mod $mod {
            use super::*;

            pub unsafe trait Variant: Symbol<u32> {
                type Raw;
                type Value: std::convert::TryFrom<Self::Raw>;
            }

            $(
                unsafe impl Variant for $Symbol {
                    type Raw = $Raw;
                    type Value = $Value;
                }
            )*
        }
    };
}

macro_rules! impl_param_write_single {
    (
        mod $mod:ident($Final:ty) {
            $( $Symbol:ident($Value:ty => $Intermediate:ty), )*
        }
    ) => {
        pub mod $mod {
            use super::*;

            pub trait Variant: Symbol<u32> {
                type Intermediate: std::convert::Into<$Final> + std::convert::From<Self::Value>;
                type Value: std::convert::Into<Self::Intermediate>;
            }

            $(
                impl Variant for $Symbol {
                    type Intermediate = $Intermediate;
                    type Value = $Value;
                }
            )*
        }
    };
}

impl_param_read_single! {
    mod get_shaderiv_param {
        COMPILE_STATUS(i32 => CompileStatus),
        // DELETE_STATUS(DeleteStatus),
        SHADER_TYPE(u32 => ShaderKind),
        INFO_LOG_LENGTH(u32 => usize),
        SHADER_SOURCE_LENGTH(u32 => usize),
    }
}

impl_param_read_single! {
    mod get_programiv_param {
// ACTIVE_ATOMIC_COUNTER_BUFFERS,
// ACTIVE_ATTRIBUTE_MAX_LENGTH,
// ACTIVE_ATTRIBUTES,
// ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
// ACTIVE_UNIFORM_BLOCKS,
// ACTIVE_UNIFORM_MAX_LENGTH,
// ACTIVE_UNIFORMS,
// ATTACHED_SHADERS,
// COMPUTE_WORK_GROUP_SIZE,
        // DELETE_STATUS(DeleteStatus),
// GEOMETRY_INPUT_TYPE,
// GEOMETRY_OUTPUT_TYPE,
// GEOMETRY_SHADER_INVOCATIONS,
// GEOMETRY_VERTICES_OUT,
        LINK_STATUS(i32 => LinkStatus),
        INFO_LOG_LENGTH(u32 => usize),
// PROGRAM_BINARY_RETRIEVABLE_HINT,
// PROGRAM_SEPARABLE,
// TESS_CONTROL_OUTPUT_VERTICES,
// TESS_GEN_MODE,
// TESS_GEN_POINT_MODE,
// TESS_GEN_VERTEX_ORDER,
// TRANSFORM_FEEDBACK_BUFFER_MODE,
// TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
// TRANSFORM_FEEDBACK_VARYINGS,
// VALIDATE_STATUS,
    }
}

impl_param_read_single! {
    mod get_integerv_param {
        MAX_COMPUTE_SHADER_STORAGE_BLOCKS(u32 => u32),
    }
}

impl_param_write_single! {
    mod tex_parameteri_param(crate::num::b32) {
// DEPTH_STENCIL_TEXTURE_MODE
// TEXTURE_BASE_LEVEL
        TEXTURE_MAX_LEVEL(u32 => u32),
        TEXTURE_MAG_FILTER(TextureMagFilter => i32),
        TEXTURE_MIN_FILTER(TextureMinFilter => i32),
        TEXTURE_WRAP_S(TextureWrap => i32),
        TEXTURE_WRAP_T(TextureWrap => i32),
        TEXTURE_WRAP_R(TextureWrap => i32),
    }
}

impl_param_write_single! {
    mod tex_parameterf_param(f32) {
        TEXTURE_MAX_ANISOTROPY(f32 => f32),
    }
}

impl_param_write_single! {
    mod sampler_parameteri_param(crate::num::b32) {
// DEPTH_STENCIL_TEXTURE_MODE
// TEXTURE_BASE_LEVEL
        TEXTURE_MAX_LEVEL(u32 => u32),
        TEXTURE_MAG_FILTER(TextureMagFilter => i32),
        TEXTURE_MIN_FILTER(TextureMinFilter => i32),
        TEXTURE_WRAP_S(TextureWrap => i32),
        TEXTURE_WRAP_T(TextureWrap => i32),
        TEXTURE_WRAP_R(TextureWrap => i32),
    }
}
