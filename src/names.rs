macro_rules! impl_names {
    ($($Name: ident,)*) => {
        $(
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
                pub fn new(name: u32) -> Option<Self> {
                    std::num::NonZeroU32::new(name).map($Name)
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
    FramebufferName,
    ProgramName,
    ShaderName,
    TextureName,
    VertexArrayName,
);

// Even though there is a distinction to be made between an
// Option<FramebufferName> and the default framebuffer, I don't think it is
// likely enough to create the types NonDefaultFramebufferName, FramebufferName
// and the conversion between them for it. Maybe I'll change my mind.
