macro_rules! impl_names {
    ($($(#[$m:meta])* $Name:ident,)*) => {
        $(
            $(#[$m])*
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct $Name(::std::num::NonZeroU32);

            impl $Name {
                /// Does not verify whether name is actually a valid name.
                #[inline]
                pub unsafe fn from_raw(name: u32) -> Option<Self> {
                    std::num::NonZeroU32::new(name).map($Name)
                }

                /// Does not verify whether name is actually a non-zero nor whether
                /// it is a valid name.
                #[inline]
                pub const unsafe fn from_raw_unchecked(name: u32) -> Self {
                    $Name(std::num::NonZeroU32::new_unchecked(name))
                }

                /// Converts the name into its raw representation without dropping.
                #[inline]
                pub unsafe fn into_raw(self) -> u32 {
                    std::mem::ManuallyDrop::new(self).as_u32()
                }

                #[inline]
                pub fn as_u32(&self) -> u32 {
                    self.0.get()
                }
            }

            impl Drop for $Name {
                #[cold]
                fn drop(&mut self) {
                    // TODO(mickvangelderen): Warn on drop/Abort on drop.
                }
            }
        )*
    };
}

impl_names!(
    /// The name of a shader, without the kind.
    BufferName,
    FramebufferName,
    ProgramName,
    ShaderName,
    TextureName,
    VertexArrayName,
);

pub struct DefaultFramebufferName;

pub trait MaybeDefaultFramebufferName {
    fn as_u32(&self) -> u32;
}

impl MaybeDefaultFramebufferName for FramebufferName {
    #[inline]
    fn as_u32(&self) -> u32 {
        FramebufferName::as_u32(self)
    }
}

impl MaybeDefaultFramebufferName for DefaultFramebufferName {
    #[inline]
    fn as_u32(&self) -> u32 {
        0
    }
}
