macro_rules! impl_received_invalid {
    ($Name: ident, $Error: ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $Error;

        impl std::fmt::Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "The OpenGL driver returned an invalid ",
                        stringify!($Name),
                        "."
                    )
                )
            }
        }

        impl std::error::Error for $Error {
            fn description(&self) -> &'static str {
                concat!(
                    "The OpenGL driver returned an invalid ",
                    stringify!($Name),
                    "."
                )
            }
        }
    };
    ($Error: ident($Value: ident), $what: tt) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $Error($Value);

        impl std::fmt::Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    concat!(
                        "The OpenGL driver returned an invalid ",
                        stringify!($Name),
                        ": {:?}."
                    ),
                    &self.0
                )
            }
        }

        impl std::error::Error for $Error {
            fn description(&self) -> &'static str {
                concat!(
                    "The OpenGL driver returned an invalid ",
                    stringify!($Name),
                    "."
                )
            }
        }
    };
}

macro_rules! impl_names {
    ($($Name: ident, $Error: ident,)*) => {
        $(
            impl_received_invalid!($Name, $Error);

            /// No guarantees are made about the validity of the object this
            /// name represents.
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $Name(pub ::std::num::NonZeroU32);

            unsafe impl convute::marker::Transmute<Option<$Name>> for $Name {}
            unsafe impl convute::marker::TryTransmute<$Name> for Option<$Name> {
                #[inline]
                fn can_transmute(&self) -> bool {
                    self.is_some()
                }
            }

            impl $Name {
                #[inline]
                pub fn new(name: u32) -> Result<Self, $Error> {
                    std::num::NonZeroU32::new(name).map($Name).ok_or($Error)
                }

                #[inline]
                pub const unsafe fn new_unchecked(name: u32) -> Self {
                    $Name(std::num::NonZeroU32::new_unchecked(name))
                }

                #[inline]
                pub fn into_u32(self) -> u32 {
                    self.0.get()
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
);

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
    #[inline]
    pub fn into_u32(self) -> u32 {
        match self {
            FramebufferName::NonDefault(name) => name.into_u32(),
            FramebufferName::Default => 0,
        }
    }
}
