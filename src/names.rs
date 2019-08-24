macro_rules! impl_names {
    ($($Name: ident, $Error: ident,)*) => {
        $(
            impl_received_invalid!($Error, $Name);

            /// No guarantees are made about the validity of the object this
            /// name represents.
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $Name(pub ::std::num::NonZeroU32);

            impl $Name {
                #[inline]
                pub fn new(name: u32) -> Result<Self, $Error> {
                    std::num::NonZeroU32::new(name).map($Name).ok_or($Error)
                }

                #[inline]
                pub const unsafe fn new_unchecked(name: u32) -> Self {
                    $Name(std::num::NonZeroU32::new_unchecked(name))
                }

                #[deprecated]
                #[inline]
                pub fn into_u32(self) -> u32 {
                    self.0.get()
                }

                #[inline]
                pub fn to_u32(&self) -> u32 {
                    self.0.get()
                }
            }

            impl AsRef<$Name> for $Name {
                fn as_ref(&self) -> &Self {
                    self
                }
            }
        )*
    };
}

impl_names!(
    BufferName,
    ReceivedInvalidBufferName,
    RenderbufferName,
    ReceivedInvalidRenderbufferName,
    NonDefaultFramebufferName,
    ReceivedInvalidFramebufferName,
    ProgramName,
    ReceivedInvalidProgramName,
    ShaderName,
    ReceivedInvalidShaderName,
    TextureName,
    ReceivedInvalidTextureName,
    VertexArrayName,
    ReceivedInvalidVertexArrayName,
    SamplerName,
    ReceivedInvalidSamplerName,
    QueryName,
    ReceivedInvalidQueryName,
    ProgramPipelineName,
    ReceivedInvalidProgramPipelineName,
    TransformFeedbackName,
    ReceivedInvalidTransformFeedbackName,
);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FramebufferName {
    NonDefault(NonDefaultFramebufferName),
    Default,
}

impl Default for FramebufferName {
    fn default() -> Self {
        FramebufferName::Default
    }
}

impl From<NonDefaultFramebufferName> for FramebufferName {
    fn from(name: NonDefaultFramebufferName) -> Self {
        FramebufferName::NonDefault(name)
    }
}

impl FramebufferName {
    #[deprecated]
    #[inline]
    pub fn into_u32(self) -> u32 {
        match self {
            FramebufferName::NonDefault(name) => name.to_u32(),
            FramebufferName::Default => 0,
        }
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        match *self {
            FramebufferName::NonDefault(ref name) => name.to_u32(),
            FramebufferName::Default => 0,
        }
    }
}
