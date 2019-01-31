pub mod gl;

use std::num::NonZeroU32;

pub struct GlTyped {
    gl: gl::Gl,
}

pub trait ShaderKind: Copy {
    type Name: ShaderName;

    fn as_u32(&self) -> u32;
}

pub trait ShaderName: Sized {
    type Kind: ShaderKind;

    fn from_raw_parts(kind: Self::Kind, name: u32) -> Option<Self>;

    fn kind(&self) -> Self::Kind;

    fn as_u32(&self) -> u32;
}

macro_rules! impl_shader_kinds_and_names {
    ($($Kind: ident, $Name: ident, $const: ident, $value: expr),*) => {
        $(
            #[derive(Debug, Copy, Clone)]
            #[repr(transparent)]
            pub struct $Kind(u32);

            impl ShaderKind for $Kind {
                type Name = $Name;

                #[inline]
                fn as_u32(&self) -> u32 {
                    self.0
                }
            }

            pub const $const: $Kind = $Kind($value);

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct $Name(NonZeroU32);

            impl ShaderName for $Name {
                type Kind = $Kind;

                #[inline]
                fn from_raw_parts(_: $Kind, name: u32) -> Option<Self> {
                    NonZeroU32::new(name).map($Name)
                }

                #[inline]
                fn kind(&self) -> Self::Kind {
                    $const
                }

                #[inline]
                fn as_u32(&self) -> u32 {
                    self.0.get()
                }
            }
        )*
    }
}

impl_shader_kinds_and_names!(
    VertexShaderKind, VertexShaderName, VERTEX_SHADER, gl::VERTEX_SHADER,
    FragmentShaderKind, FragmentShaderName, FRAGMENT_SHADER, gl::FRAGMENT_SHADER
);

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum DynamicShaderKind {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

impl ShaderKind for DynamicShaderKind {
    type Name = DynamicShaderName;

    #[inline]
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

pub struct DynamicShaderName {
    kind: DynamicShaderKind,
    name: NonZeroU32,
}

impl ShaderName for DynamicShaderName {
    type Kind = DynamicShaderKind;

    #[inline]
    fn from_raw_parts(kind: DynamicShaderKind, name: u32) -> Option<Self> {
        NonZeroU32::new(name).map(|name| DynamicShaderName { kind, name })
    }

    #[inline]
    fn kind(&self) -> Self::Kind {
        self.kind
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        self.name.get()
    }
}

impl GlTyped {
    #[inline]
    pub unsafe fn create_shader<SN: ShaderName>(&self, kind: SN::Kind) -> Option<SN> {
        SN::from_raw_parts(kind, self.gl.CreateShader(kind.as_u32()))
    }
}
